use crate::{
    compile::Error as CompileError,
    dialect::{Dialect, DialectKey, Dialects},
    keyword::context,
    schema::CompiledSchema,
    spec::Specification,
};
use grill_core::{
    resolve::Error as ResolveError,
    schema::Schemas,
    source::{LinkError, Source, SourceConflictError, SourceKey, Sources},
    Resolve,
};
use grill_uri::{AbsoluteUri, Uri};
use jsonptr::{Pointer, PointerBuf, Token};
use serde_json::Value;
use slotmap::Key;

use jsonptr::Resolve as _;
use std::{
    collections::{vec_deque, HashMap, VecDeque},
    fmt,
    path::PathBuf,
};

mod resolve;
use resolve::{resolve, resolved, Resolved};
pub(super) enum Scanned<'scan, K> {
    Scan(&'scan mut Scan<K>),
    Compiled(K),
}

#[derive(Debug)]
pub(super) struct Scan<K> {
    pub id: Option<AbsoluteUri>,
    pub dialect_key: DialectKey,
    pub anchors: Vec<Anchor>,
    pub uris: Vec<AbsoluteUri>,
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
    ) -> Result<Scanned<K>, Error<R>>
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

    fn scan_src<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        ctx: &mut context::Compile<'int, 'txn, 'res, R, S, K>,
        uri: &AbsoluteUri,
        src: resolved::Src,
    ) -> Result<Scanned<'scan, K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        #[derive(Debug)]
        struct Cursor {
            source: Source<'static>,
            remaining: PointerBuf,
            super_dialect_key: Option<DialectKey>,
            next: Option<Box<Cursor>>,
        }

        let sources = &mut *ctx.context.state.sources;
        let schemas = &mut *ctx.context.state.schemas;

        let source = sources.source(src.source_key).into_owned();
        let doc_key = source.document().key();
        let doc_value = source.document().value_arc();

        let dialects = ctx.dialects;
        let ptr = source.path();

        let mut cursor = Some(Cursor {
            source,
            remaining: PointerBuf::default(),
            super_dialect_key: None,
            next: None,
        });

        while let Some(c) = cursor.take() {
            let Cursor {
                source,
                mut remaining,
                super_dialect_key,
                next,
            } = c;
            let mut scan_and_insert = |dialect_key| {
                let scan = self.scan_value(schemas, sources, dialects, source, dialect_key)?;
                let idx =self.scans.len();
                self.scanned.insert(source.uri().clone(), idx);
                self.scans.push(scan);
                Ok(idx)
            };
            let mut advance_cursor = | dialect_key | {
                if let Some(mut next) = next {
                    next.super_dialect_key = Some(dialect_key);
                    cursor = Some(*next);
                }
            };

            if let Some(scan_idx) = self.scanned.get(source.uri()).copied() {
            }
            let value = source.resolve();


            let Some(dialect_key) = dialects.find_dialect_key(value) else {
                if let Some((ptr, tok)) = source.path().split_back() {

                    sources.link(uri, key, path)
                }
                else {
                    scan_and_insert(dialects.primary_dialect_key())?;
                    advance_cursor(dialects.primary_dialect_key());
                    continue;
                } 
                todo!()
            };

            let Some(dialect_key) =  else {
                if remaining.is_root() {
                }
            };
        }
        todo!()
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
    ) -> Result<Scanned<'scan, K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        todo!()
    }

    fn scan_value<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        schemas: &mut Schemas<CompiledSchema<S, K>, K>,
        sources: &mut Sources,
        dialects: &Dialects<S, K>,
        source: Source<'static>,
        dialect_key: DialectKey,
    ) -> Result<Scan<K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        let dialect = dialects.get(dialect_key);
        let value = source.resolve();
        let uri = source.uri();
        let references = link_and_collect_refs(schemas, dialect, uri, value)?;
        let embeds = link_and_collect_embeds(schemas, sources, dialect, &source, uri, value)?;
        let anchors = link_and_collect_anchors(sources, dialect, uri, value, &source)?;
        let id = dialect
            .identify(value)?
            .map(|id| uri.resolve(&id))
            .transpose()?;
        let mut uris = Vec::new();
        if let Some(id) = &id {
            uris.push(id.clone());
        }
        if Some(uri) != id.as_ref() {
            uris.push(uri.clone());
        }

        Ok(Scan {
            id,
            anchors,
            uris,
            source,
            embeds,
            references,
        })
    }
}

fn link_and_collect_anchors<S, K, R>(
    sources: &mut Sources,
    dialect: &Dialect<S, K>,
    uri: &AbsoluteUri,
    value: &Value,
    src: &Source,
) -> Result<Vec<Anchor>, Error<R>>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve + Send + Sync,
{
    let mut anchors = Vec::new();
    for anchor in dialect.anchors(value) {
        let uri = uri.with_fragment(Some(&anchor.value)).unwrap();
        let source_key = sources.link(uri.clone(), src.document_key(), src.path().to_buf())?;
        anchors.push(Anchor {
            uri,
            name: anchor.value,
            keyword: anchor.keyword,
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
) -> Result<Vec<Embed<K>>, Error<R>>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve + Send + Sync,
{
    let mut embeds = Vec::new();
    for embed in dialect.embedded_schemas(value) {
        let embed_uri = uri.with_fragment(Some(embed.value.as_str())).unwrap();
        let mut path = source.path().to_buf();
        path.append(&embed.value);
        let source_key = sources.link(embed_uri.clone(), source.document_key(), path)?;
        embeds.push(Embed {
            relative_pointer: embed.value.clone(),
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
) -> Result<Vec<Reference<K>>, Error<R>>
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
    pub uri: AbsoluteUri,
    pub name: String,
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
    pub relative_pointer: PointerBuf,
    pub source_key: SourceKey,
    pub schema_key: Option<K>,
}

#[derive(Debug)]
pub(super) enum Error<R: 'static + Resolve> {
    Resolve(ResolveError<R>),
    SourceConflict(Box<SourceConflictError>),
    Uri(grill_uri::Error),
}
impl<R: 'static + Resolve> From<LinkError> for Error<R> {
    fn from(value: LinkError) -> Self {
        match value {
            LinkError::SourceConflict(e) => Error::SourceConflict(e),
            LinkError::InvalidPath(_) => unreachable!(),
        }
    }
}
impl<R: 'static + Resolve> fmt::Display for Error<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to scan schema")
    }
}
impl<R: 'static + Resolve> From<grill_uri::Error> for Error<R> {
    fn from(value: grill_uri::Error) -> Self {
        Error::Uri(value)
    }
}

impl<O, R> From<Error<R>> for CompileError<O, R>
where
    R: 'static + Resolve,
{
    fn from(value: Error<R>) -> Self {
        match value {
            Error::Resolve(e) => CompileError::FailedToResolve(Box::new(e)),
            Error::SourceConflict(e) => CompileError::SourceConflict(e),
            Error::Uri(e) => CompileError::InvalidUri(Box::new(e)),
        }
    }
}
impl<R: 'static + Resolve> From<ResolveError<R>> for Error<R> {
    fn from(value: ResolveError<R>) -> Self {
        Error::Resolve(value)
    }
}
