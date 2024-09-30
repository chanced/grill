use crate::{
    compile::Error as CompileError,
    dialect::{Dialect, DialectKey, Dialects},
    keyword::context,
    schema::CompiledSchema,
    spec::{keyword::Found, Specification},
};
use grill_core::{
    resolve::Error as ResolveError,
    schema::Schemas,
    source::{Fragment, LinkError, New, Source, SourceConflictError, SourceKey, Sources},
    Resolve,
};
use grill_uri::{AbsoluteUri, Uri};
use jsonptr::{Pointer, Resolve as _};
use serde_json::Value;
use slotmap::Key;
use std::{collections::HashMap, fmt};

mod resolve;
use resolve::{resolve, resolved, Resolved};

pub(super) enum Scanned<'scan, K> {
    Scan(&'scan mut Scan<K>),
    Compiled(K),
}

#[derive(Debug)]
pub(super) struct Scan<K> {
    pub index: usize,
    pub id: Option<AbsoluteUri>,
    pub dialect_key: DialectKey,
    pub anchors: Vec<Anchor>,
    pub source: Source<'static>,
    pub embeds: Vec<Embed<K>>,
    pub references: Vec<Reference<K>>,
}
impl<K> Scan<K> {
    pub(super) fn unresolved_refs(&mut self) -> impl Iterator<Item = &mut Reference<K>> {
        self.references
            .iter_mut()
            .filter(|r| r.schema_key.is_none())
    }
}

#[derive(Debug, Default)]
pub(super) struct Scanner<K> {
    scans: Vec<Scan<K>>,
    scanned: HashMap<AbsoluteUri, usize>,
}

impl<K> Scanner<K>
where
    K: 'static + Key + Send + Sync,
{
    pub(super) async fn scan<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        ctx: &mut context::Compile<'int, 'txn, 'res, R, S, K>,
        uri: &'cmp AbsoluteUri,
    ) -> Result<Scanned<K>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        'cmp: 'scan,
        'res: 'int,
    {
        if let Some(key) = ctx.context.state.schemas.get_key_of(uri) {
            return Ok(Scanned::Compiled(key));
        }
        if let Some(&index) = self.scanned.get(uri) {
            return Ok(Scanned::Scan(&mut self.scans[index]));
        }
        match resolve(ctx.context.state.sources, ctx.context.resolve, uri).await? {
            Resolved::Source(src) => self.scan_src(ctx, uri, src),
            Resolved::UnknownAnchor(doc) => self.scan_for_anchor(ctx, uri, doc),
        }
    }

    fn scan_src<'scan, R, S>(
        &'scan mut self,
        ctx: &mut context::Compile<R, S, K>,
        _uri: &AbsoluteUri,
        src: resolved::Src,
    ) -> Result<Scanned<'scan, K>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        let sources = &mut *ctx.context.state.sources;
        let schemas = &mut *ctx.context.state.schemas;
        let dialects = ctx.dialects;
        let source = sources.source(src.source_key).into_owned();
        let mut stack = vec![(source, Pointer::root())];
        let mut super_key = None;

        while let Some((source, remaining)) = stack.pop() {
            self.scan_src_item(
                &mut stack,
                &mut super_key,
                sources,
                schemas,
                dialects,
                source,
                remaining,
            )?;
        }
        todo!()
    }

    fn scan_src_item<'r, R, S>(
        &mut self,
        stack: &mut Vec<(Source<'static>, &'r Pointer)>,
        default_dialect_key: &mut Option<DialectKey>,
        sources: &mut Sources,
        schemas: &mut Schemas<CompiledSchema<S, K>, K>,
        dialects: &Dialects<S, K>,
        source: Source<'static>,
        remaining: &'r Pointer,
    ) -> Result<(), Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
    {
        // checking to see if we have already scanned the source
        if let Some(scan_idx) = self.scanned.get(source.uri()).copied() {
            // we have already scanned this source so we have a dialect key to use
            default_dialect_key.replace(self.scans[scan_idx].dialect_key);
            return Ok(());
        }
        // checking to see if we can determine the dialect of the source
        if let Some(dialect_key) = dialects
            .find_dialect_key(source.resolve())
            .or(*default_dialect_key)
        {
            // we were able to determine the dialect so we can scan the value
            return self.scan_src_item_found_dialect(
                dialect_key,
                stack,
                sources,
                schemas,
                dialects,
                default_dialect_key,
                source,
                remaining,
            );
        }
        if let Some((remaining, token)) = remaining.split_back() {
            let absolute_path = source.absolute_path().with_trailing_token(token);
            let uri = source.uri().with_fragment(absolute_path.as_str()).unwrap();
            let fragment = Some(Fragment::Pointer(absolute_path.clone()));
            let document_key = source.document_key();
            let source_key = sources.link(New {
                uri,
                fragment,
                document_key,
                absolute_path,
            })?;
            let source = sources.source(source_key).into_owned();
            stack.push((source, remaining));
            return Ok(());
        }
        default_dialect_key.replace(dialects.default_dialect_key());
        stack.push((source, remaining));
        Ok(())
    }

    fn scan_src_item_found_dialect<'r, R, S>(
        &mut self,
        dialect_key: DialectKey,
        stack: &mut Vec<(Source<'static>, &'r Pointer)>,
        sources: &mut Sources,
        schemas: &mut Schemas<CompiledSchema<S, K>, K>,
        dialects: &Dialects<S, K>,
        super_key: &mut Option<DialectKey>,
        source: Source<'static>,
        remaining: &'r Pointer,
    ) -> Result<(), Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
    {
        super_key.replace(dialect_key);
        let scan = match self.scan_value::<R, S>(schemas, sources, dialects, source, dialect_key) {
            Ok(scan) => Ok(scan),
            Err(err) if remaining.is_root() => Err(err.cause),
            Err(err) => {
                // we are not at the root but failed to scan the value
                let source = err.src;
                // in order to move on, we check the next node in the path to
                // see if we can find the container schema of our target
                let (_, remaining) = remaining.split_front().unwrap();
                if remaining.is_root() {
                    // next node is our target - we assume the default dialect
                    super_key.replace(dialects.default_dialect_key());
                    return Ok(());
                }
                let document_key = source.document_key();
                let absolute_path = source.absolute_path().concat(remaining);
                let uri = source.uri().with_fragment(absolute_path.as_str()).unwrap();
                let fragment = Some(Fragment::Pointer(absolute_path.clone()));
                let source_key = sources.link(New {
                    uri,
                    fragment,
                    document_key,
                    absolute_path,
                })?;
                let source = sources.source(source_key).into_owned();

                stack.push((source, remaining));
                return Ok(());
            }
        }?;
        let path = scan.source.absolute_path();
        let document_key = scan.source.document_key();
        // At this point, the target schema's path is not discoverable with the
        // current schema's embeds. We need to pop a token off the path and
        // try again.

        let (tok, remaining) = remaining.split_front().unwrap();
        let ptr = Pointer::parse(tok.encoded()).unwrap();
        let absolute_path = path.concat(ptr);
        let uri = scan
            .source
            .uri()
            .with_fragment(absolute_path.as_str())
            .unwrap();
        let fragment = Some(Fragment::Pointer(absolute_path.clone()));
        let source_key = sources.link(New {
            absolute_path,
            uri,
            fragment,
            document_key,
        })?;
        let source = sources.source(source_key).into_owned();
        stack.push((source, remaining));
        Ok(())
    }

    fn insert_scan(&mut self, scan: Scan<K>) -> usize {
        let idx = self.scans.len();
        self.scanned.insert(scan.source.uri().clone(), idx);
        self.scans.push(scan);
        idx
    }

    fn scan_for_anchor<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        _ctx: &mut context::Compile<'int, 'txn, 'res, R, S, K>,
        _uri: &AbsoluteUri,
        _doc: resolved::UnknownAnchor,
    ) -> Result<Scanned<'scan, K>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        todo!()
    }

    fn scan_value<R, S>(
        &mut self,
        schemas: &mut Schemas<CompiledSchema<S, K>, K>,
        sources: &mut Sources,
        dialects: &Dialects<S, K>,
        src: Source<'static>,
        dialect_key: DialectKey,
    ) -> Result<&Scan<K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        let dialect = dialects.get(dialect_key);
        let value = src.resolve();
        let uri = src.uri();

        let handle_err = |cause| Error {
            cause,
            src: src.clone(),
        };

        let references =
            link_and_collect_refs(schemas, dialect, uri, value).map_err(|cause| Error {
                cause,
                src: src.clone(),
            })?;

        let embeds = link_and_collect_embeds(schemas, sources, dialect, &src, uri, value)
            .map_err(handle_err)?;

        let anchors =
            link_and_collect_anchors(sources, dialect, uri, value, &src).map_err(handle_err)?;

        let id = dialect
            .identify(value)
            .map_err(|err| handle_err(err.into()))?
            .map(|id| uri.resolve(&id))
            .transpose()
            .map_err(|err| handle_err(err.into()))?;

        let index = self.scans.len();
        self.scanned.insert(src.uri().clone(), index);
        let scan = Scan {
            index,
            id,
            dialect_key,
            anchors,
            source: src,
            embeds,
            references,
        };
        self.scans.push(scan);
        Ok(&self.scans[index])
    }
}

fn link_and_collect_anchors<S, K, R>(
    sources: &mut Sources,
    dialect: &Dialect<S, K>,
    uri: &AbsoluteUri,
    value: &Value,
    src: &Source,
) -> Result<Vec<Anchor>, Cause<R>>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve + Send + Sync,
{
    let mut anchors = Vec::new();
    for Found { keyword, value } in dialect.anchors(value) {
        let uri = uri.with_fragment(&value).unwrap();
        let source_key = sources.link(New {
            uri,
            document_key: src.document_key(),
            absolute_path: src.absolute_path().to_buf(),
            fragment: Some(Fragment::Anchor(value)),
        })?;
        anchors.push(Anchor {
            keyword,
            source_key,
        });
    }
    Ok(anchors)
}

fn link_and_collect_embeds<'int, 'txn, 'res, R, S, K>(
    schemas: &Schemas<CompiledSchema<S, K>, K>,
    sources: &mut Sources,
    dialect: &Dialect<S, K>,
    source: &Source,
    uri: &AbsoluteUri,
    value: &Value,
) -> Result<Vec<Embed<K>>, Cause<R>>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve + Send + Sync,
{
    let mut embeds = Vec::new();
    for embed in dialect.embedded_schemas(value) {
        let embed_uri = uri.with_fragment(embed.value.as_str()).unwrap();
        let absolute_path = source.absolute_path().to_buf().concat(&embed.value);
        let source_key = sources.link(New {
            uri: embed_uri,
            document_key: source.document_key(),
            absolute_path,
            fragment: Some(Fragment::Pointer(embed.value)),
        })?;

        embeds.push(Embed {
            source_key,
            schema_key: schemas.get_key_of(uri),
        });
    }
    Ok(embeds)
}

fn link_and_collect_refs<'int, 'txn, 'res, R, S, K>(
    schemas: &Schemas<CompiledSchema<S, K>, K>,
    dialect: &Dialect<S, K>,
    uri: &AbsoluteUri,
    value: &Value,
) -> Result<Vec<Reference<K>>, Cause<R>>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve + Send + Sync,
{
    let mut refs = Vec::new();
    for reference in dialect.references(value) {
        let Ok(ref_uri) = Uri::parse(&reference.value) else {
            continue;
        };
        let Ok(ref_absolute_uri) = uri.resolve(&ref_uri) else {
            continue;
        };
        refs.push(Reference {
            absolute_uri: ref_absolute_uri,
            uri: ref_uri,
            keyword: reference.keyword,
            schema_key: schemas.get_key_of(uri),
        });
    }
    Ok(refs)
}

#[derive(Debug)]
pub(super) struct Anchor {
    pub keyword: &'static str,
    pub source_key: SourceKey,
}

#[derive(Debug)]
pub(super) struct Reference<K> {
    pub absolute_uri: AbsoluteUri,
    pub uri: Uri,
    pub keyword: &'static str,
    pub schema_key: Option<K>,
}

#[derive(Debug)]
pub(super) struct Embed<K> {
    pub source_key: SourceKey,
    pub schema_key: Option<K>,
}

pub(super) struct Error<R: 'static + Resolve> {
    pub src: Source<'static>,
    pub cause: Cause<R>,
}

#[derive(Debug)]
pub(super) enum Cause<R: 'static + Resolve> {
    Resolve(ResolveError<R>),
    SourceConflict(Box<SourceConflictError>),
    Uri(grill_uri::Error),
}
impl<R: 'static + Resolve> From<LinkError> for Cause<R> {
    fn from(value: LinkError) -> Self {
        match value {
            LinkError::SourceConflict(e) => Cause::SourceConflict(e),
            LinkError::InvalidPath(_) => unreachable!(),
        }
    }
}
impl<R: 'static + Resolve> fmt::Display for Cause<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to scan schema")
    }
}
impl<R: 'static + Resolve> From<grill_uri::Error> for Cause<R> {
    fn from(value: grill_uri::Error) -> Self {
        Cause::Uri(value)
    }
}

impl<O, R> From<Cause<R>> for CompileError<O, R>
where
    R: 'static + Resolve,
{
    fn from(value: Cause<R>) -> Self {
        match value {
            Cause::Resolve(e) => CompileError::FailedToResolve(Box::new(e)),
            Cause::SourceConflict(e) => CompileError::SourceConflict(e),
            Cause::Uri(e) => CompileError::InvalidUri(Box::new(e)),
        }
    }
}
impl<R: 'static + Resolve> From<ResolveError<R>> for Cause<R> {
    fn from(value: ResolveError<R>) -> Self {
        Cause::Resolve(value)
    }
}
