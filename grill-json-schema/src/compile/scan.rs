use crate::{compile::Error as CompileError, keyword::context, spec::Specification};
use grill_core::{resolve::Error as ResolveError, source::Source, Resolve};
use grill_uri::{AbsoluteUri, Uri};
use jsonptr::PointerBuf;
use slotmap::Key;
use std::{collections::HashMap, fmt, path::PathBuf};

mod resolve;
use resolve::{resolve, resolved, Resolved};

#[derive(Debug)]
pub(super) enum Error<R: 'static + Resolve> {
    Resolve(ResolveError<R>),
}
impl<O, R> From<Error<R>> for CompileError<O, R>
where
    R: 'static + Resolve,
{
    fn from(value: Error<R>) -> Self {
        match value {
            Error::Resolve(e) => CompileError::FailedToResolve(e),
        }
    }
}
impl<R: 'static + Resolve> fmt::Display for Error<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Resolve(e) => fmt::Display::fmt(e, f),
        }
    }
}
impl<R: 'static + Resolve> From<ResolveError<R>> for Error<R> {
    fn from(value: ResolveError<R>) -> Self {
        Error::Resolve(value)
    }
}
pub(super) enum Scanned<'scan, K> {
    Scan(&'scan mut Scan<K>),
    Compiled(K),
}

#[derive(Debug)]
pub(super) struct Scan<K> {
    pub id: Option<AbsoluteUri>,
    pub anchors: Vec<Anchor>,
    pub uris: Vec<AbsoluteUri>,
    pub source: Source<'static>,
    pub embeds: Vec<PathBuf>,
    pub refs: Vec<Reference<K>>,
}
impl<K> Scan<K> {
    pub(super) fn unresolved_refs(&mut self) -> impl Iterator<Item = &mut Reference<K>> {
        self.refs.iter_mut().filter(|r| r.schema_key.is_none())
    }
}

#[derive(Debug)]
pub(super) struct Anchor {
    pub uri: AbsoluteUri,
    pub name: String,
    pub keyword: &'static str,
}

#[derive(Debug)]
pub(super) struct Reference<K> {
    pub absolute_uri: AbsoluteUri,
    pub uri: Uri,
    pub keyword: &'static str,
    pub schema_key: Option<K>,
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
        let source = ctx.context.state.sources.source(src.source_key);
        let root = source.document().value_ref();
        let mut path = source.pointer().to_buf();
    }

    fn scan_for_anchor<'scan, 'cmp, 'int, 'txn, 'res, R, S>(
        &'scan mut self,
        ctx: &mut context::Compile<'int, 'txn, 'res, R, S, K>,
        uri: &AbsoluteUri,
        doc: resolved::UnknownAnchor,
    ) -> Result<Scanned<'scan, K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        todo!()
    }
}

// struct Scan<'scan, 'int, 'txn, 'res, C, R, S, K>
// where
//     C: crate::spec::Compile<'int, 'txn, 'res, R, S, K>,
//     R: 'static + Resolve + Send + Sync,
//     S: 'static + Specification<K>,
//     K: 'static + Key + Send + Sync,
// {
//     scanned: &'scan mut HashMap<AbsoluteUri, Link<'static>>,
//     ctx: &'scan mut C,
//     uri: &'scan AbsoluteUri,
//     _marker: PhantomData<(&'int R, &'txn S, &'res R, &'int K)>,
// }

// impl<'scan, 'int, 'txn, 'res, C, R, S, K> Scan<'scan, 'int, 'txn, 'res, C, R, S, K>
// where
//     C: crate::spec::Compile<'int, 'txn, 'res, R, S, K>,
//     R: 'static + Resolve + Send + Sync,
//     S: 'static + Specification<K>,
//     K: 'static + Key + Send + Sync,
// {
//     fn exec<E>(
//         scanned: &'scan mut HashMap<AbsoluteUri, Link<'static>>,
//         ctx: &'scan mut C,
//         uri: &'scan AbsoluteUri,
//     ) -> Result<Scanned<'scan, K>, E>
//     where
//         E: crate::spec::CompileError<R, S, K>,
//     {
//         Self {
//             scanned,
//             ctx,
//             uri,
//             _marker: PhantomData,
//         }
//         .scan()
//     }

//     fn scan<E>(self) -> Result<Scanned<'scan, K>, E>
//     where
//         E: crate::spec::CompileError<R, S, K>,
//     {
//         todo!()
//     }
// }
