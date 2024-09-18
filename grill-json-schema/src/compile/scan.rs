use crate::compile::{resolve::Resolved, Error as CompileError};
use crate::schema::CompiledSchema;
use grill_core::lang::source::Source;
use grill_core::lang::{Schemas, Sources};
use grill_core::resolve::Error as ResolveError;
use grill_core::{
    lang::{self},
    Resolve,
};
use grill_uri::AbsoluteUri;
use polonius_the_crab::polonius;
use slotmap::Key;
use std::fmt;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    spec::{found::Anchor, Specification},
    JsonSchema,
};

use super::resolve::{resolve, resolved};

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

pub(super) enum Scanned<'scan, K: 'static + Key> {
    Previous(&'scan Scan),
    Pending,
    Scanned(&'scan Scan),
    Compiled(K),
}

#[derive(Debug)]
pub(super) struct Scan {
    pub(super) id: Option<AbsoluteUri>,
    pub(super) anchors: Vec<Anchor>,
    pub(super) uris: Vec<AbsoluteUri>,
    pub(super) source: Source<'static>,
    pub(super) subschemas: Vec<PathBuf>,
}

#[derive(Debug, Default)]
pub(super) struct Scanner {
    scans: Vec<Scan>,
    scanned: HashMap<AbsoluteUri, usize>,
}
impl Scanner {
    pub(super) async fn scan<'scan, 'cmp, 'int, 'txn, 'res, R, S, K>(
        &'scan mut self,
        ctx: &mut lang::compile::Context<'int, 'txn, 'res, JsonSchema<K, S>, R, K>,
        uri: &'cmp AbsoluteUri,
    ) -> Result<Scanned<'scan, K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
        'cmp: 'scan,
        'res: 'int,
    {
        if let Some(&index) = self.scanned.get(uri) {
            return Ok(Scanned::Previous(&self.scans[index]));
        }

        match resolve(&mut ctx.state.sources, ctx.resolve, uri).await? {
            Resolved::Source(src) => self.resolve_src(ctx, uri, src),
            Resolved::Document(doc) => self.resolve_doc(ctx, uri, doc),
        }
    }

    fn resolve_src<'scan, 'cmp, 'int, 'txn, 'res, R, S, K>(
        &'scan mut self,
        ctx: &mut lang::compile::Context<'int, 'txn, 'res, JsonSchema<K, S>, R, K>,
        uri: &AbsoluteUri,
        src: resolved::Src,
    ) -> Result<Scanned<'scan, K>, Error<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        todo!()
    }

    fn resolve_doc<'scan, 'cmp, 'int, 'txn, 'res, R, S, K>(
        &self,
        ctx: &mut lang::compile::Context<'int, 'txn, 'res, JsonSchema<K, S>, R, K>,
        uri: &AbsoluteUri,
        doc: resolved::Doc,
    ) -> Result<Scanned<'_, K>, Error<R>>
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
