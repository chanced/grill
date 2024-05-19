//! Graph traversal [`Iterator`]s for [`Schema`]s

use super::{Reference, Schemas};
use crate::{language::Language, source::Sources, Key, Schema};
use either::Either;
use grill_uri::AbsoluteUri;
use std::{
    collections::HashSet,
    iter::{empty, once, Empty, Map, Once},
    vec::IntoIter,
};

// TODO: create an impl for each mode of traversal for `CompiledSchema`
// the current way of handling any internal op, which doesn't need the
// source currently has to pull it anyway.
// However... the bump in WASM / binary output size and compile time may not warrant this.

// TODO: Before creating an impl of traversal for `CompiledSchema`, benchmark
// & measure WASM output & compile time. This is low priority as this only
// pertains to "compile"/setup

/// A trait composed of utility methods for dealing with [`Iterator`]s of [`Schema`]s.
pub trait Traverse<'i, L, K, Iter>: Iterator<Item = Schema<'i, L, K>>
where
    L: 'static + Language<K>,
    K: 'static + Key,
    Self: Sized,
    Iter: Iterator<Item = Schema<'i, L, K>>,
{
    /// Returns a new [`Keys`] [`Iterator`] which consumes this `Iterator` and
    /// yields an `Iterator` of `Key`
    fn keys(self) -> Keys<'i, L, K, Iter>;

    /// Returns a new [`MapIntoOwned`] [`Iterator`] which consumes this
    /// `Iterator` of [`Schema<'i, L, K>`] and yields owned copies (i.e.
    /// [`Schema<'static>`]).
    fn map_into_owned(self) -> MapIntoOwned<'i, L, K, Self>;

    /// Searches the [`Iterator`] for a [`Schema`] with the specified
    /// [`AbsoluteUri`] in it's set of URIs
    fn find_by_uri(self, uri: &AbsoluteUri) -> Option<Schema<'i, L, K>>;
}

impl<'i, L, K, I> Traverse<'i, L, K, I> for I
where
    L: 'static + Language<K>,
    K: 'static + Key,
    I: Iterator<Item = Schema<'i, L, K>>,
{
    fn keys(self) -> Keys<'i, L, K, I> {
        Keys { iter: self }
    }

    fn map_into_owned(self) -> MapIntoOwned<'i, L, K, I> {
        MapIntoOwned { iter: self }
    }

    fn find_by_uri(mut self, uri: &AbsoluteUri) -> Option<Schema<'i, L, K>> {
        self.find(|schema| schema.id.as_deref() == Some(uri) || schema.uris.contains(uri))
    }
}

/// Maps an [`Iterator`] of [`Schema<'i, L, K>`](`Schema`) into one of [`Schema<'static>`](`Schema`).
pub struct MapIntoOwned<'i, L, K, I>
where
    L: 'static + Language<K>,
    K: 'static + Key,
    I: Iterator<Item = Schema<'i, L, K>>,
{
    iter: I,
}
impl<'i, L, K, I> Iterator for MapIntoOwned<'i, L, K, I>
where
    L: 'static + Language<K>,
    K: 'static + Key,
    I: Iterator<Item = Schema<'i, L, K>>,
{
    type Item = Schema<'static, L, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Schema::into_owned)
    }
}

/// Maps an [`Iterator`] of [`Schema<'i, L, K>`](crate::schema::Schema) into one of `Key`
///
/// See [`Traverse::keys`] for usage.
pub struct Keys<'i, L, K, I>
where
    L: 'static + Language<K>,
    K: 'static + Key,
    I: Iterator<Item = Schema<'i, L, K>>,
{
    iter: I,
}

impl<'i, L, K, I> Iterator for Keys<'i, L, K, I>
where
    L: 'static + Language<K>,
    K: 'static + Key,
    I: Iterator<Item = Schema<'i, L, K>>,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|s| s.key)
    }
}

macro_rules! impl_traverse {
    ($name:ident, $func:ident) => {
        impl<'i, L, K> Iterator for $name<'i, L, K>
        where
            L: Criterion<K>,
            K: 'static + Key,
        {
            type Item = Schema<'i, L, K>;
            fn next(&mut self) -> Option<Self::Item> {
                self.traverse.next()
            }
        }
    };
}

/// A [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
/// [`Iterator`] which traverses both direct and indirect dependents of
/// a [`Schema`].
pub struct AllDependents<'i, L: Language<K>, K: 'static + Key> {
    traverse: Slices<'i, L, K>,
}

impl_traverse!(AllDependents, all_dependents);

fn all_dependents<L, K>(schema: Schema<'_, L, K>) -> IntoIter<K>
where
    L: Language<K>,
    K: 'static + Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependents.into_owned().into_iter()
}

impl<'i, L, K> AllDependents<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub(crate) fn new(key: K, schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(key, schemas, sources, all_dependents),
        }
    }
}

/// A [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
/// [`Iterator`] which traverses both direct and indirect dependencies of
/// a [`Schema`].
pub struct TransitiveDependencies<'i, L: Language<K>, K: 'static + Key> {
    traverse: TransitiveDeps<'i, L, K>,
}

impl_traverse!(TransitiveDependencies, transitive_dependencies);
fn transitive_dependencies<L: Language<K>, K: 'static + Key>(schema: Schema<'_, L, K>) -> Deps<K> {
    #[allow(clippy::unnecessary_to_owned)]
    schema.references.into_owned().into_iter().map(|r| r.key)
}

impl<'i, L, K> TransitiveDependencies<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub(crate) fn new(key: K, schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(key, schemas, sources, transitive_dependencies),
        }
    }
}

/// An [`Iterator`] over the hiearchy of a given [`Schema`].
///
///
/// Note that the JSON Schema specification states that if a schema is
/// identified (by having either an `$id` field for Draft 07 and beyond or an
/// `id` field for Draft 04 and earlier), then it must be the document root. As
/// such, embedded schemas with an id  will not have a parent, even if the
/// [`Schema`] is embedded.
pub struct Ancestors<'i, L: Language<K>, K: 'static + Key> {
    traverse: Instances<'i, L, K>,
}

impl_traverse!(Ancestors, ancestors);
fn ancestors<L, K>(schema: Schema<'_, L, K>) -> Either<Once<K>, Empty<K>>
where
    L: Language<K>,
    K: 'static + Key,
{
    if let Some(parent) = schema.parent {
        Either::Left(once(parent))
    } else {
        Either::Right(empty())
    }
}
impl<'i, L, K> Ancestors<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub(crate) fn new(key: K, schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(key, schemas, sources, ancestors),
        }
    }
}

/// An [`Iterator`] over the hiearchy of a given [`Schema`].
///
///
/// Note that the JSON Schema specification states that if a schema is
/// identified (by having either an `$id` field for Draft 07 and beyond or an
/// `id` field for Draft 04 and earlier), then it must be the document root. As
/// such, embedded schemas with an id  will not have a parent, even if the
/// [`Schema`] is embedded.
pub struct Descendants<'i, L: Language<K>, K: 'static + Key> {
    traverse: Slices<'i, L, K>,
}
impl_traverse!(Descendants, descendants);
fn descendants<L, K>(schema: Schema<'_, L, K>) -> IntoIter<K>
where
    L: Language<K>,
    K: 'static + Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.subschemas.into_owned().into_iter()
}
impl<'i, L, K> Descendants<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub(crate) fn new(key: K, schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(key, schemas, sources, descendants),
        }
    }
}

struct Flat<'i, L, K, Inner>
where
    L: Language<K>,
    K: 'static + Key,
    Inner: Iterator<Item = K>,
{
    iter: Inner,
    schemas: &'i Schemas<L, K>,
    sources: &'i Sources,
}
impl<'i, L, K, Inner> Flat<'i, L, K, Inner>
where
    L: Language<K>,
    K: 'static + Key,
    Inner: Iterator<Item = K>,
{
    fn new(iter: Inner, schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            iter,
            schemas,
            sources,
        }
    }
}

impl<'i, L, K, Inner> Iterator for Flat<'i, L, K, Inner>
where
    L: Language<K>,
    K: 'static + Key,
    Inner: Iterator<Item = K>,
{
    type Item = Schema<'i, L, K>;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.iter.next()?;
        Some(self.schemas.get_unchecked(key, self.sources))
    }
}

struct DepthFirst<'i, L, K, Iter, Func>
where
    L: Language<K>,
    K: 'static + Key,
    Iter: Iterator<Item = K>,
    Func: Fn(Schema<'i, L, K>) -> Iter,
{
    handle: Func,
    stack: Vec<Iter>,
    sent: HashSet<K>,
    schemas: &'i Schemas<L, K>,
    sources: &'i Sources,
}

impl<'i, L, K, Iter, Func> DepthFirst<'i, L, K, Iter, Func>
where
    L: Language<K>,
    K: 'static + Key,
    Iter: 'i + Iterator<Item = K>,
    Func: Fn(Schema<'i, L, K>) -> Iter,
{
    pub(crate) fn new(
        key: K,
        schemas: &'i Schemas<L, K>,
        sources: &'i Sources,
        handle: Func,
    ) -> Self {
        let first = handle(schemas.get_unchecked(key, sources));
        Self {
            handle,
            stack: Vec::from([first]),
            sent: HashSet::new(),
            schemas,
            sources,
        }
    }
    fn exec(&self, schema: Schema<'i, L, K>) -> Iter {
        (self.handle)(schema)
    }
}

impl<'i, L, K, Iter, Func> Iterator for DepthFirst<'i, L, K, Iter, Func>
where
    L: Language<K>,
    K: 'static + Key,
    Iter: 'i + Iterator<Item = K>,
    Func: Fn(Schema<'i, L, K>) -> Iter,
{
    type Item = Schema<'i, L, K>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.stack.is_empty() {
                return None;
            }
            let front = self.stack.last_mut()?;
            let next = front.next();
            if next.is_none() {
                self.stack.pop();
                continue;
            }
            let next = next.unwrap();
            if self.sent.contains(&next) {
                continue;
            }
            let next = self.schemas.get_unchecked(next, self.sources);
            self.stack.push(self.exec(next.clone()));
            return Some(next);
        }
    }
}

macro_rules! iter {
    (
        $(#[$($attrss:tt)*])*
        $vis:vis $name:ident @ $func:ident -> $iter:ident

    ) => {
        $(#[$($attrss)*])*
        $vis struct $name<'i, L: Criterion<K>, K: 'static + Key> {
            iter: Flat<'i, L, K, $iter<K>>,
        }
        impl<'i, L, K>  $name<'i, L, K>  where L: Criterion<K>, K: 'static + Key {
            #[doc=concat!("Creates a new ", stringify!($name))]
            pub(crate) fn new(key: K, schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self
            {
                let schema = schemas.get_unchecked(key, sources);
                let iter = $func(schema);
                let iter = Flat::new(iter, schemas, sources);
                Self { iter }
            }
        }
        impl<'i, L, K> Iterator for $name<'i, L, K>
        where
            L: Criterion<K>,
            K: 'static + Key
        {
            type Item = Schema<'i, L, K>;

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
        }
    };
}

iter! {
    /// An [`Iterator`] over the direct dependencies of a [`Schema`]
    pub DirectDependencies @ direct_dependencies -> Deps
}

type IntoKeyIter<K> = IntoIter<K>;

type Deps<K> = Map<IntoIter<Reference<K>>, fn(Reference<K>) -> K>;
type TransitiveDeps<'i, L, K> = DepthFirst<'i, L, K, Deps<K>, fn(Schema<'i, L, K>) -> Deps<K>>;

fn direct_dependencies<L, K>(schema: Schema<'_, L, K>) -> Deps<K>
where
    L: Language<K>,
    K: 'static + Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.references.into_owned().into_iter().map(|r| r.key)
}

iter! {
    /// An [`Iterator`] over [`Schema`](crate::schema::Schema)s which directly
    /// depend on a specified [`Schema`](crate::schema::Schema)
    pub DirectDependents @ direct_dependents -> IntoKeyIter
}
fn direct_dependents<L: Language<K>, K: 'static + Key>(schema: Schema<'_, L, K>) -> IntoIter<K> {
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependents.into_owned().into_iter()
}

type Slices<'i, L, K> =
    DepthFirst<'i, L, K, IntoKeyIter<K>, fn(Schema<'i, L, K>) -> IntoKeyIter<K>>;

type Instances<'i, L, K> = DepthFirst<
    'i,
    L,
    K,
    Either<Once<K>, Empty<K>>,
    fn(Schema<'i, L, K>) -> Either<Once<K>, Empty<K>>,
>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         schema::{CompiledSchema, Reference},
//         source::{deserialize_json, Deserializers},
//         AbsoluteUri, Key,
//     };
//     use jsonptr::Pointer;
//     use serde_json::json;
//     use std::borrow::Cow;

//     fn id_paths<K>(schema: Schema<'_, K>) -> String {
//         schema.id.unwrap().path_or_nss().to_owned()
//     }
//     #[test]
//     fn test_direct_dependents() {
//         let (roots, schemas, sources) = build_graph();
//         let traverse = DirectDependencies::new(roots[0], &schemas, &sources);
//         let ids = traverse.map(id_paths).collect::<Vec<_>>();
//         assert_eq!(
//             &ids,
//             &[
//                 "/a/dependency_a",
//                 "/a/dependency_b",
//                 "/a/dependency_c",
//                 "/a/dependency_d",
//             ]
//         );
//     }

//     #[test]
//     /// This test ignores the rule surrounding identified schemas being document roots.
//     fn test_ancestors() {
//         let (_, schemas, sources) = build_graph();
//         let leaf_id =
//             create_test_uri("/a/subschema_a/nested_subschema_a/deeply_nested_subschema_a");
//         let leaf_key = schemas.get_key(&leaf_id).unwrap();
//         let traverse = Ancestors::new(leaf_key, &schemas, &sources);
//         let ids = traverse.map(id_paths).collect::<Vec<_>>();

//         assert_eq!(
//             &ids,
//             &["/a/subschema_a/nested_subschema_a", "/a/subschema_a", "/a",]
//         );
//     }

//     #[test]
//     fn test_all_dependents() {
//         let (_, schemas, sources) = build_graph();
//         let leaf_id = create_test_uri("/a/dependency_b/transitive_b/distant_transitive_c");
//         let leaf_key = schemas.get_key(&leaf_id).unwrap();

//         let traverse = AllDependents::new(leaf_key, &schemas, &sources);

//         let ids = traverse.map(id_paths).collect::<Vec<_>>();

//         assert_eq!(
//             &ids,
//             &["/a/dependency_b/transitive_b", "/a/dependency_b", "/a",]
//         );
//     }

//     #[test]
//     fn test_transitive_dependencies() {
//         use similar::{ChangeTag, TextDiff};
//         let (root_keys, schemas, sources) = build_graph();
//         let traverse = TransitiveDependencies::new(root_keys[0], &schemas, &sources);
//         let ids = traverse
//             .map(|schema| schema.id.unwrap().path_or_nss().to_owned())
//             .collect::<Vec<_>>();
//         let expected = &[
//             "/a/dependency_a",
//             "/a/dependency_a/transitive_a",
//             "/a/dependency_a/transitive_a/distant_transitive_a",
//             "/a/dependency_a/transitive_a/distant_transitive_b",
//             "/a/dependency_a/transitive_a/distant_transitive_c",
//             "/a/dependency_a/transitive_b",
//             "/a/dependency_a/transitive_b/distant_transitive_a",
//             "/a/dependency_a/transitive_b/distant_transitive_b",
//             "/a/dependency_a/transitive_b/distant_transitive_c",
//             "/a/dependency_a/transitive_c",
//             "/a/dependency_a/transitive_c/distant_transitive_a",
//             "/a/dependency_a/transitive_c/distant_transitive_b",
//             "/a/dependency_a/transitive_c/distant_transitive_c",
//             "/a/dependency_a/transitive_d",
//             "/a/dependency_a/transitive_d/distant_transitive_a",
//             "/a/dependency_a/transitive_d/distant_transitive_b",
//             "/a/dependency_a/transitive_d/distant_transitive_c",
//             "/a/dependency_b",
//             "/a/dependency_b/transitive_a",
//             "/a/dependency_b/transitive_a/distant_transitive_a",
//             "/a/dependency_b/transitive_a/distant_transitive_b",
//             "/a/dependency_b/transitive_a/distant_transitive_c",
//             "/a/dependency_b/transitive_b",
//             "/a/dependency_b/transitive_b/distant_transitive_a",
//             "/a/dependency_b/transitive_b/distant_transitive_b",
//             "/a/dependency_b/transitive_b/distant_transitive_c",
//             "/a/dependency_b/transitive_c",
//             "/a/dependency_b/transitive_c/distant_transitive_a",
//             "/a/dependency_b/transitive_c/distant_transitive_b",
//             "/a/dependency_b/transitive_c/distant_transitive_c",
//             "/a/dependency_b/transitive_d",
//             "/a/dependency_b/transitive_d/distant_transitive_a",
//             "/a/dependency_b/transitive_d/distant_transitive_b",
//             "/a/dependency_b/transitive_d/distant_transitive_c",
//             "/a/dependency_c",
//             "/a/dependency_c/transitive_a",
//             "/a/dependency_c/transitive_a/distant_transitive_a",
//             "/a/dependency_c/transitive_a/distant_transitive_b",
//             "/a/dependency_c/transitive_a/distant_transitive_c",
//             "/a/dependency_c/transitive_b",
//             "/a/dependency_c/transitive_b/distant_transitive_a",
//             "/a/dependency_c/transitive_b/distant_transitive_b",
//             "/a/dependency_c/transitive_b/distant_transitive_c",
//             "/a/dependency_c/transitive_c",
//             "/a/dependency_c/transitive_c/distant_transitive_a",
//             "/a/dependency_c/transitive_c/distant_transitive_b",
//             "/a/dependency_c/transitive_c/distant_transitive_c",
//             "/a/dependency_c/transitive_d",
//             "/a/dependency_c/transitive_d/distant_transitive_a",
//             "/a/dependency_c/transitive_d/distant_transitive_b",
//             "/a/dependency_c/transitive_d/distant_transitive_c",
//             "/a/dependency_d",
//             "/a/dependency_d/transitive_a",
//             "/a/dependency_d/transitive_a/distant_transitive_a",
//             "/a/dependency_d/transitive_a/distant_transitive_b",
//             "/a/dependency_d/transitive_a/distant_transitive_c",
//             "/a/dependency_d/transitive_b",
//             "/a/dependency_d/transitive_b/distant_transitive_a",
//             "/a/dependency_d/transitive_b/distant_transitive_b",
//             "/a/dependency_d/transitive_b/distant_transitive_c",
//             "/a/dependency_d/transitive_c",
//             "/a/dependency_d/transitive_c/distant_transitive_a",
//             "/a/dependency_d/transitive_c/distant_transitive_b",
//             "/a/dependency_d/transitive_c/distant_transitive_c",
//             "/a/dependency_d/transitive_d",
//             "/a/dependency_d/transitive_d/distant_transitive_a",
//             "/a/dependency_d/transitive_d/distant_transitive_b",
//             "/a/dependency_d/transitive_d/distant_transitive_c",
//         ];

//         assert_eq!(
//             &ids,
//             expected,
//             "{}",
//             TextDiff::from_lines(&format!("{expected:#?}"), &format!("{ids:#?}"))
//                 .iter_all_changes()
//                 .map(|change| {
//                     let sign = match change.tag() {
//                         ChangeTag::Delete => "-",
//                         ChangeTag::Insert => "+",
//                         ChangeTag::Equal => " ",
//                     };
//                     format!("{sign}{change}")
//                 })
//                 .fold(String::new(), |acc, c| format!("{acc}{c}"))
//         );
//     }

//     fn build_graph() -> (Vec<K>, Schemas, Sources) {
//         const REF: &str = "$ref";

//         let mut schemas: Schemas = Schemas::new();
//         let deserializers = Deserializers::new(vec![("json", Box::new(deserialize_json))]);
//         let mut sources = Sources::new(vec![], &deserializers).unwrap();
//         schemas.start_txn();
//         sources.start_txn();
//         let mut root_keys = vec![];
//         // builds subschemas
//         for r in 'a'..='d' {
//             let root_key = schemas
//                 .insert(create_schema(&r.to_string(), &mut sources))
//                 .unwrap();
//             root_keys.push(root_key);
//             for n in 'a'..='d' {
//                 let id = format!("{r}/subschema_{n}");
//                 let sub_key = schemas.insert(create_schema(id, &mut sources)).unwrap();
//                 {
//                     let sub = schemas.get_mut(sub_key).unwrap();
//                     sub.parent = Some(root_key);
//                 }
//                 {
//                     let root = schemas.get_mut(root_key).unwrap();
//                     root.subschemas.push(sub_key);
//                 }
//                 for n2 in 'a'..'d' {
//                     let id = format!("{r}/subschema_{n}/nested_subschema_{n2}");
//                     let sub_sub_key = schemas.insert(create_schema(id, &mut sources)).unwrap();
//                     {
//                         let sub_sub = schemas.get_mut(sub_sub_key).unwrap();
//                         sub_sub.parent = Some(sub_key);
//                     }
//                     {
//                         let parent = schemas.get_mut(sub_key).unwrap();
//                         parent.subschemas.push(sub_sub_key);
//                     }
//                     for n3 in 'a'..'d' {
//                         let sub_sub_sub_key = schemas
//                             .insert(create_schema(format!(
//                                 "{r}/subschema_{n}/nested_subschema_{n2}/deeply_nested_subschema_{n3}"
//                             ), &mut sources))
//                             .unwrap();
//                         {
//                             let sub_sub_sub = schemas.get_mut(sub_sub_sub_key).unwrap();
//                             sub_sub_sub.parent = Some(sub_sub_key);
//                         }
//                         {
//                             let parent = schemas.get_mut(sub_sub_key).unwrap();
//                             parent.subschemas.push(sub_sub_key);
//                         }
//                     }
//                 }
//             }
//             for d in 'a'..='d' {
//                 let uri = create_test_uri(&format!("{r}/dependency_{d}"));
//                 let dep_key = schemas
//                     .insert(create_schema(uri.clone(), &mut sources))
//                     .unwrap();
//                 {
//                     let root = schemas.get_mut(root_key).unwrap();
//                     root.references.push(Reference {
//                         key: dep_key,
//                         absolute_uri: uri.clone(),
//                         uri: uri.clone().into(),
//                         keyword: REF,
//                     });
//                 }
//                 {
//                     let dep = schemas.get_mut(dep_key).unwrap();
//                     dep.dependents.push(root_key);
//                 }
//                 for t in 'a'..='d' {
//                     let uri = create_test_uri(format!("{r}/dependency_{d}/transitive_{t}"));
//                     let transitive_dep_key = schemas
//                         .insert(create_schema(uri.clone(), &mut sources))
//                         .unwrap();
//                     {
//                         let dep = schemas.get_mut(dep_key).unwrap();
//                         dep.references.push(Reference {
//                             key: transitive_dep_key,
//                             absolute_uri: uri.clone(),
//                             uri: uri.clone().into(),
//                             keyword: REF,
//                         });
//                     }
//                     {
//                         let transitive_dep = schemas.get_mut(transitive_dep_key).unwrap();
//                         transitive_dep.dependents.push(dep_key);
//                     }
//                     for t2 in 'a'..'d' {
//                         let transitive_dep_key_2 = schemas
//                             .insert(create_schema(
//                                 format!(
//                                     "{r}/dependency_{d}/transitive_{t}/distant_transitive_{t2}"
//                                 ),
//                                 &mut sources,
//                             ))
//                             .unwrap();
//                         {
//                             let transitive_dep = schemas.get_mut(transitive_dep_key).unwrap();
//                             let uri = create_test_uri(
//                                 "{r}/dependency_{d}/transitive_{t}/distant_transitive_{t2}",
//                             );

//                             transitive_dep.references.push(Reference {
//                                 key: transitive_dep_key_2,
//                                 absolute_uri: uri.clone(),
//                                 uri: uri.clone().into(),
//                                 keyword: REF,
//                             });
//                         }
//                         {
//                             let transitive_dep_2 = schemas.get_mut(transitive_dep_key_2).unwrap();
//                             transitive_dep_2.dependents.push(transitive_dep_key);
//                         }
//                     }
//                 }
//             }
//         }
//         schemas.commit_txn();
//         sources.commit_txn();

//         (root_keys, schemas, sources)
//     }
//     fn create_test_uri(uri: impl ToString) -> AbsoluteUri {
//         let mut uri = uri.to_string();
//         if !uri.starts_with("https") {
//             if uri.starts_with('/') {
//                 uri = uri.strip_prefix('/').unwrap().to_string();
//             }
//             uri = format!("https://example.com/{uri}");
//         }
//         AbsoluteUri::parse(&uri).unwrap()
//     }
//     fn metaschema() -> AbsoluteUri {
//         "https://json-schema.org/draft/2020-12/schema"
//             .parse()
//             .unwrap()
//     }
//     fn create_schema(uri: impl ToString, sources: &mut Sources) -> CompiledSchema {
//         let uri: AbsoluteUri = create_test_uri(uri);
//         let (_, link, _) = sources
//             .insert_value(uri.clone(), Cow::Owned(json!({"$id": uri.clone()})))
//             .unwrap();
//         let metaschema = metaschema();
//         CompiledSchema {
//             id: Some(uri.clone()),
//             anchors: Vec::default(),
//             parent: None,
//             references: Vec::new(),
//             dependents: Vec::new(),
//             keywords: Vec::new().into_boxed_slice(),
//             subschemas: vec![],
//             uris: vec![uri.clone()],
//             link,
//             path: Pointer::default(),
//             metaschema,
//             compiled: false,
//         }
//     }
// }
