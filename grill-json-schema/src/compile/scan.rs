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
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
};

mod resolve;
use resolve::{resolve, resolved, Resolved};

pub(super) enum Scanned<'scan, K> {
    Scan(&'scan mut Scan<K>),
    Compiled(K),
}

#[derive(Debug, Default)]
pub struct Scans<K> {
    list: Vec<Scan<K>>,
    by_uri: HashMap<AbsoluteUri, usize>,
    by_source_key: BTreeMap<SourceKey, usize>,
}

impl<K> Scans<K> {
    fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&Scan<K>> {
        self.by_uri.get(uri).map(|&idx| &self.list[idx])
    }
    fn get(&self, idx: usize) -> &Scan<K> {
        &self.list[idx]
    }
    fn get_mut_by_uri(&mut self, uri: &AbsoluteUri) -> Option<&mut Scan<K>> {
        self.by_uri.get(uri).map(|&idx| &mut self.list[idx])
    }
    fn contains_uri(&self, uri: &AbsoluteUri) -> bool {
        self.by_uri.contains_key(uri)
    }
    fn contains_source_key(&self, key: SourceKey) -> bool {
        self.by_source_key.contains_key(&key)
    }
    fn get_by_source_key(&self, source_key: SourceKey) -> Option<&Scan<K>> {
        self.by_source_key
            .get(&source_key)
            .copied()
            .map(|idx| &self.list[idx])
    }
    fn get_mut_by_source_key(&mut self, source_key: SourceKey) -> Option<&mut Scan<K>> {
        self.by_source_key
            .get(&source_key)
            .copied()
            .map(|idx| &mut self.list[idx])
    }

    fn insert(&mut self, mut scan: Scan<K>) -> &Scan<K> {
        let index = self.list.len();
        scan.index = index;
        self.by_uri.insert(scan.source.uri().clone(), index);
        self.by_source_key.insert(scan.source.key(), index);
        self.list.push(scan);
        &self.list[index]
    }
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
    scans: Scans<K>,
}

impl<K> Scanner<K>
where
    K: 'static + Key + Send + Sync,
{
    pub(super) fn new() -> Self {
        Self::default()
    }
    pub(super) async fn scan<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        ctx: &mut context::Compile<'int, 'txn, 'res, R, S, K>,
        uri: &'cmp AbsoluteUri,
    ) -> Result<Scanned<'scan, K>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        'cmp: 'scan,
        'res: 'int,
    {
        if let Some(key) = ctx.interrogator.state.schemas.get_key_of(uri) {
            return Ok(Scanned::Compiled(key));
        }
        if self.scans.contains_uri(uri) {
            return Ok(Scanned::Scan(self.scans.get_mut_by_uri(uri).unwrap()));
        }
        match resolve(
            ctx.interrogator.state.sources,
            ctx.interrogator.resolve,
            uri,
        )
        .await?
        {
            Resolved::Source(src) => self.scan_src(ctx, src),
            Resolved::UnknownAnchor(doc) => self.scan_for_anchor(ctx, doc),
        }
    }

    fn scan_src<'scan, R, S>(
        &'scan mut self,
        ctx: &mut context::Compile<R, S, K>,
        src: resolved::Src,
    ) -> Result<Scanned<'scan, K>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        // In order to scan this source, we first need to determine the
        // dialect. In order to do so, we may need to look at some of the
        // schema's ancestors.
        //
        // We check each source along the way, until we find a schema with a
        // specified dialect or the root of the document. If we reach the root
        // and do not find a $schema field, we use the user's specified deafult
        // dialect
        let sources = &mut *ctx.interrogator.state.sources;
        let schemas = &mut *ctx.interrogator.state.schemas;
        let dialects = ctx.dialects;
        let source_key = sources.source(src.source_key).key();
        let mut stack = vec![source_key];
        let mut super_dialect_key = None;
        let target = source_key;
        while let Some(source_key) = stack.pop() {
            super_dialect_key = self
                .scan_src_item(
                    &mut stack,
                    source_key,
                    sources,
                    schemas,
                    dialects,
                    target,
                    super_dialect_key,
                )?
                .or(super_dialect_key);
        }
        todo!()
    }

    /// Attempts to scan an item on the stack of [`scan_src`](Self::scan_src).
    fn scan_src_item<R, S>(
        &mut self,
        stack: &mut Vec<SourceKey>,
        source_key: SourceKey,
        sources: &mut Sources,
        schemas: &mut Schemas<CompiledSchema<S, K>, K>,
        dialects: &Dialects<S, K>,
        target_source_key: SourceKey,
        default_dialect_key: Option<DialectKey>,
    ) -> Result<Option<DialectKey>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
    {
        // checking to see if we have already scanned the source
        if self.scans.contains_source_key(source_key) {
            // existing scan found so we have a dialect key to use
            return Ok(Some(
                self.scans
                    .get_by_source_key(source_key)
                    .unwrap()
                    .dialect_key,
            ));
        }

        // checking the value to see if a dialect has been specified
        if let Some(dialect_key) = dialects
            .find_dialect_key(sources.source(source_key).resolve())
            .or(default_dialect_key)
        {
            // we were able to determine the dialect so we can scan the value
            return self.scan_src_item_found_dialect(
                stack,
                dialect_key,
                source_key,
                sources,
                schemas,
                dialects,
                target_source_key,
                default_dialect_key,
            );
        }

        // couldn't find the dialect

        stack.push(source_key);

        // if we aren't at the root, we need to check the next node in the path
        let source = sources.source(source_key);

        if source.absolute_path().is_root() {
            // we are at the root and we need to use the default dialect
            return Ok(default_dialect_key);
        }

        let absolute_path = source.absolute_path().split_back().unwrap().0.to_buf();
        let uri = source.uri().with_fragment(absolute_path.as_str()).unwrap();
        let fragment = Some(Fragment::Pointer(absolute_path.clone()));
        let document_key = source.document_key();
        let source_key = sources.link(New {
            uri,
            fragment,
            document_key,
            absolute_path,
        })?;
        stack.push(source_key);
        Ok(default_dialect_key)
    }

    fn scan_src_item_found_dialect<'r, R, S>(
        &mut self,
        stack: &mut Vec<SourceKey>,
        dialect_key: DialectKey,
        source_key: SourceKey,
        sources: &mut Sources,
        schemas: &mut Schemas<CompiledSchema<S, K>, K>,
        dialects: &Dialects<S, K>,
        target_source_key: SourceKey,
        previous_dialect_key: Option<DialectKey>,
    ) -> Result<Option<DialectKey>, Cause<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
    {
        // let scan =
        //     match self.scan_value::<R, S>(schemas, sources, dialects, source_key, dialect_key) {
        //         Ok(scan) => Ok(scan), // successfully scanned the source
        //         Err(err) if source_key == target_source_key => Err(err.cause), // failed to scan target
        //         Err(err) => return Ok(previous_dialect_key) // scan failed but this source isn't the target so we skip
        //     }?;
        // let path = scan.source.absolute_path();
        // let document_key = scan.source.document_key();
        // // At this point, the target schema's path is not discoverable with the
        // // current schema's embeds. We need to pop a token off the path and
        // // try again.

        // let (tok, remaining) = remaining.split_front().unwrap();
        // let ptr = Pointer::parse(tok.encoded()).unwrap();
        // let absolute_path = path.concat(ptr);
        // let uri = scan
        //     .source
        //     .uri()
        //     .with_fragment(absolute_path.as_str())
        //     .unwrap();
        // let fragment = Some(Fragment::Pointer(absolute_path.clone()));
        // let source_key = sources.link(New {
        //     uri,
        //     fragment,
        //     document_key,
        //     absolute_path,
        // })?;
        // let source = sources.source(source_key).into_owned();
        // stack.push((source, remaining));
        // Ok(())
        todo!()
    }

    fn insert_scan(&mut self, scan: Scan<K>) -> usize {
        // let idx = self.scans.len();
        // self.scanned.insert(scan.source.uri().clone(), idx);
        // self.scans.push(scan);
        // idx
        todo!()
    }

    fn scan_for_anchor<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        _ctx: &mut context::Compile<'int, 'txn, 'res, R, S, K>,
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
        source_key: SourceKey,
        dialect_key: DialectKey,
    ) -> Result<&Scan<K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        let source = sources.source(source_key).into_owned();
        let dialect = dialects.get(dialect_key);
        let value = source.resolve();
        let uri = source.uri();
        // let handle_err = |cause| Error { cause, source_key };

        // let references =
        //     link_and_collect_refs(schemas, dialect, uri, value).map_err(|cause| Error {
        //         cause,
        //         src: source.clone(),
        //     })?;

        // let embeds = link_and_collect_embeds(schemas, sources, dialect, &source, uri, value)
        //     .map_err(handle_err)?;

        // let anchors =
        //     link_and_collect_anchors(sources, dialect, uri, value, &source).map_err(handle_err)?;

        // let id = dialect
        //     .identify(value)
        //     .map_err(|err| handle_err(err.into()))?
        //     .map(|id| uri.resolve(&id))
        //     .transpose()
        //     .map_err(|err| handle_err(err.into()))?;

        // let index = self.scans.len();
        // self.scanned.insert(source.uri().clone(), index);
        // let scan = Scan {
        //     index,
        //     id,
        //     dialect_key,
        //     anchors,
        //     source,
        //     embeds,
        //     references,
        // };

        // self.scans.insert()
        // self.scans.push(scan);
        // Ok(&self.scans[index])
        todo!()
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
    pub source_key: SourceKey,
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
