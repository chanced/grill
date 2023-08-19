use super::Schemas;
use crate::{source::Sources, Schema};
use either::Either;
use std::{
    collections::{HashSet, VecDeque},
    iter::{empty, once, Empty, Map, Once},
    vec::IntoIter,
};

/// A trait composed of utility methods for dealing with [`Iterator`]s of [`Schema`]s.
pub trait Traverse<'i, Key, Iter>: Iterator<Item = Schema<'i, Key>>
where
    Self: Sized,
    Key: 'i + slotmap::Key,
    Iter: Iterator<Item = Schema<'i, Key>>,
{
    /// Returns a new [`Keys`] [`Iterator`] which consumes this `Iterator` and
    /// yields an `Iterator` of `Key`
    fn keys(self) -> Keys<'i, Key, Iter>;
    /// Returns a new [`MapIntoOwned`] [`Iterator`] which consumes this
    /// `Iterator` of [`Schema<'i, Key>`] and yields owned copies (i.e.
    /// [`Schema<'static, Key>`]).
    fn map_into_owned(self) -> MapIntoOwned<'i, Key, Self>;
}
impl<'i, Key, Iter> Traverse<'i, Key, Iter> for Iter
where
    Iter: Iterator<Item = Schema<'i, Key>>,
    Key: 'static + slotmap::Key,
{
    fn keys(self) -> Keys<'i, Key, Iter> {
        Keys { iter: self }
    }

    fn map_into_owned(self) -> MapIntoOwned<'i, Key, Iter> {
        MapIntoOwned { iter: self }
    }
}

pub struct MapIntoOwned<'i, Key, Iter>
where
    Key: 'static + slotmap::Key,
    Iter: Iterator<Item = Schema<'i, Key>>,
{
    iter: Iter,
}
impl<'i, Key, Iter> Iterator for MapIntoOwned<'i, Key, Iter>
where
    Key: slotmap::Key,
    Iter: Iterator<Item = Schema<'i, Key>>,
{
    type Item = Schema<'static, Key>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Schema::into_owned)
    }
}

fn into_owned<Key: slotmap::Key>(schema: Schema<'_, Key>) -> Schema<'static, Key> {
    schema.into_owned()
}

/// Maps an [`Iterator`] of [`Schema<'_, Key>`](crate::schema::Schema) into an [`Iterator`] of `Key`
///
/// See [`Traverse::keys`] for usage.
pub struct Keys<'i, Key, Iter>
where
    Key: 'i + slotmap::Key,
    Iter: Iterator<Item = Schema<'i, Key>>,
{
    iter: Iter,
}

impl<'i, Key, Iter> Iterator for Keys<'i, Key, Iter>
where
    Key: 'i + slotmap::Key,
    Iter: Iterator<Item = Schema<'i, Key>>,
{
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|s| s.key)
    }
}

macro_rules! impl_traverse {
    ($name:ident, $func:ident) => {
        impl<'i, Key> Iterator for $name<'i, Key>
        where
            Key: slotmap::Key,
        {
            type Item = Schema<'i, Key>;
            fn next(&mut self) -> Option<Self::Item> {
                self.traverse.next()
            }
        }
    };
}

/// A [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
/// [`Iterator`] which traverses both direct and indirect dependents of
/// a [`Schema`].
pub struct AllDependents<'i, Key: slotmap::Key> {
    traverse: Slices<'i, Key>,
}

impl_traverse!(AllDependents, all_dependents);

fn all_dependents<Key>(schema: Schema<'_, Key>) -> IntoIter<Key>
where
    Key: slotmap::Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependents.into_owned().into_iter()
}

impl<'i, Key> AllDependents<'i, Key>
where
    Key: slotmap::Key,
{
    pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(key, schemas, sources, all_dependents),
        }
    }
}

/// A [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
/// [`Iterator`] which traverses both direct and indirect dependencies of
/// a [`Schema`].
pub struct TransitiveDependencies<'i, Key: slotmap::Key> {
    traverse: Slices<'i, Key>,
}

impl_traverse!(TransitiveDependencies, transitive_dependencies);

fn transitive_dependencies<Key>(schema: Schema<'_, Key>) -> IntoIter<Key>
where
    Key: slotmap::Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependencies.into_owned().into_iter()
}

impl<'i, Key> TransitiveDependencies<'i, Key>
where
    Key: slotmap::Key,
{
    pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
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
pub struct Ancestors<'i, Key: slotmap::Key> {
    traverse: Instances<'i, Key>,
}

impl_traverse!(Ancestors, ancestors);
fn ancestors<Key>(schema: Schema<'_, Key>) -> Either<Once<Key>, Empty<Key>>
where
    Key: slotmap::Key,
{
    if let Some(parent) = schema.parent {
        Either::Left(once(parent))
    } else {
        Either::Right(empty())
    }
}
impl<'i, Key> Ancestors<'i, Key>
where
    Key: slotmap::Key,
{
    pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
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
pub struct Descendants<'i, Key: slotmap::Key> {
    traverse: Slices<'i, Key>,
}
impl_traverse!(Descendants, descendants);
fn descendants<Key>(schema: Schema<'_, Key>) -> IntoIter<Key>
where
    Key: slotmap::Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.subschemas.into_owned().into_iter()
}
impl<'i, Key> Descendants<'i, Key>
where
    Key: slotmap::Key,
{
    pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(key, schemas, sources, descendants),
        }
    }
}

struct Iter<'i, Key: slotmap::Key, Inner: Iterator<Item = Key>> {
    iter: Inner,
    schemas: &'i Schemas<Key>,
    sources: &'i Sources,
}
impl<'i, Key, Inner> Iter<'i, Key, Inner>
where
    Key: slotmap::Key,
    Inner: Iterator<Item = Key>,
{
    fn new(iter: Inner, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
        Self {
            iter,
            schemas,
            sources,
        }
    }
}

impl<'i, Key, Inner> Iterator for Iter<'i, Key, Inner>
where
    Key: slotmap::Key,
    Inner: Iterator<Item = Key>,
{
    type Item = Schema<'i, Key>;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.iter.next()?;
        Some(self.schemas.get_unchecked(key, self.sources))
    }
}

struct DepthFirst<'i, Key, Iter, Func>
where
    Key: slotmap::Key,
    Iter: Iterator<Item = Key>,
    Func: Fn(Schema<'i, Key>) -> Iter,
{
    handle: Func,
    queue: VecDeque<Iter>,
    sent: HashSet<Key>,
    schemas: &'i Schemas<Key>,
    sources: &'i Sources,
}

impl<'i, Key, Iter, Func> DepthFirst<'i, Key, Iter, Func>
where
    Key: slotmap::Key,
    Iter: 'i + Iterator<Item = Key>,
    Func: Fn(Schema<'i, Key>) -> Iter,
{
    pub(crate) fn new(
        key: Key,
        schemas: &'i Schemas<Key>,
        sources: &'i Sources,
        handle: Func,
    ) -> Self {
        let first = handle(schemas.get_unchecked(key, sources));
        Self {
            handle,
            queue: VecDeque::from([first]),
            sent: HashSet::new(),
            schemas,
            sources,
        }
    }
    fn exec(&self, schema: Schema<'i, Key>) -> Iter {
        (self.handle)(schema)
    }
}

impl<'i, Key, Iter, Func> Iterator for DepthFirst<'i, Key, Iter, Func>
where
    Key: slotmap::Key,
    Iter: 'i + Iterator<Item = Key>,
    Func: Fn(Schema<'i, Key>) -> Iter,
{
    type Item = Schema<'i, Key>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.queue.is_empty() {
                return None;
            }
            let front = self.queue.front_mut()?;
            let next = front.next();
            if next.is_none() {
                self.queue.pop_front();
                continue;
            }
            let next = next.unwrap();
            if self.sent.contains(&next) {
                continue;
            }
            let next = self.schemas.get_unchecked(next, self.sources);
            self.queue.push_front(self.exec(next.clone()));
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
        $vis struct $name<'i, Key: slotmap::Key> {
            iter: Iter<'i, Key, $iter<Key>>,
        }

        impl<'i, Key> $name<'i, Key>
        where
            Key: slotmap::Key,
            {
                #[doc=concat!("Creates a new ", stringify!($name))]
                pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self
            {
                let schema = schemas.get_unchecked(key, sources);
                let iter = $func(schema);
                let iter = Iter::new(iter, schemas, sources);
                Self { iter }
            }
        }
        impl<'i, Key> Iterator for $name<'i, Key>
        where
            Key: slotmap::Key,
        {
            type Item = Schema<'i, Key>;

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
        }
    };
}
iter! {
    /// An [`Iterator`] over the direct dependencies of a [`Schema`]
    pub DirectDependencies @ direct_dependencies -> IntoIter
}
fn direct_dependencies<Key: slotmap::Key>(schema: Schema<'_, Key>) -> IntoIter<Key> {
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependencies.into_owned().into_iter()
}

iter! {
    /// An [`Iterator`] over [`Schema`](crate::schema::Schema)s which directly
    /// depend on a specified [`Schema`](crate::schema::Schema)
    pub DirectDependents @ direct_dependents -> IntoIter
}
fn direct_dependents<Key: slotmap::Key>(schema: Schema<'_, Key>) -> IntoIter<Key> {
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependents.into_owned().into_iter()
}

type Slices<'i, Key> = DepthFirst<'i, Key, IntoIter<Key>, fn(Schema<'i, Key>) -> IntoIter<Key>>;

type Instances<'i, Key> = DepthFirst<
    'i,
    Key,
    Either<Once<Key>, Empty<Key>>,
    fn(Schema<'i, Key>) -> Either<Once<Key>, Empty<Key>>,
>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::format;

    use crate::{
        schema::CompiledSchema,
        source::{deserialize_json, Deserializers, Source},
        AbsoluteUri, SchemaKey,
    };
    use jsonptr::Pointer;
    use serde_json::json;
    use slotmap::SlotMap;

    fn id_paths<Key: slotmap::Key>(schema: Schema<'_, Key>) -> String {
        schema.id.unwrap().path_or_nss().to_owned()
    }
    #[test]
    fn test_direct_dependents() {
        let (roots, schemas, sources) = build_graph();
        let traverse = DirectDependencies::new(roots[0], &schemas, &sources);
        let ids = traverse.map(id_paths).collect::<Vec<_>>();
        assert_eq!(
            &ids,
            &[
                "/a/dependency_a",
                "/a/dependency_b",
                "/a/dependency_c",
                "/a/dependency_d",
            ]
        );
    }

    #[test]
    /// This test ignores the rule surrounding identified schemas being document roots.
    fn test_ancestors() {
        let (_, schemas, sources) = build_graph();
        let leaf_id =
            create_test_uri("/a/subschema_a/nested_subschema_a/deeply_nested_subschema_a");
        let leaf_key = schemas.get_key_by_id(&leaf_id).unwrap();
        let traverse = Ancestors::new(leaf_key, &schemas, &sources);
        let ids = traverse.map(id_paths).collect::<Vec<_>>();

        assert_eq!(
            &ids,
            &["/a/subschema_a/nested_subschema_a", "/a/subschema_a", "/a",]
        );
    }

    #[test]
    fn test_all_dependents() {
        let (_, schemas, sources) = build_graph();
        let leaf_id = create_test_uri("/a/dependency_b/transitive_b/distant_transitive_c");
        let leaf_key = schemas.get_key_by_id(&leaf_id).unwrap();

        let traverse = AllDependents::new(leaf_key, &schemas, &sources);

        let ids = traverse.map(id_paths).collect::<Vec<_>>();

        assert_eq!(
            &ids,
            &["/a/dependency_b/transitive_b", "/a/dependency_b", "/a",]
        );
    }

    #[test]
    fn test_transitive_dependencies() {
        use similar::{ChangeTag, TextDiff};

        let (root_keys, schemas, sources) = build_graph();
        let traverse = TransitiveDependencies::new(root_keys[0], &schemas, &sources);
        let ids = traverse
            .map(|schema| schema.id.unwrap().path_or_nss().to_owned())
            .collect::<Vec<_>>();
        let expected = &[
            "/a/dependency_a",
            "/a/dependency_a/transitive_a",
            "/a/dependency_a/transitive_a/distant_transitive_a",
            "/a/dependency_a/transitive_a/distant_transitive_b",
            "/a/dependency_a/transitive_a/distant_transitive_c",
            "/a/dependency_a/transitive_b",
            "/a/dependency_a/transitive_b/distant_transitive_a",
            "/a/dependency_a/transitive_b/distant_transitive_b",
            "/a/dependency_a/transitive_b/distant_transitive_c",
            "/a/dependency_a/transitive_c",
            "/a/dependency_a/transitive_c/distant_transitive_a",
            "/a/dependency_a/transitive_c/distant_transitive_b",
            "/a/dependency_a/transitive_c/distant_transitive_c",
            "/a/dependency_a/transitive_d",
            "/a/dependency_a/transitive_d/distant_transitive_a",
            "/a/dependency_a/transitive_d/distant_transitive_b",
            "/a/dependency_a/transitive_d/distant_transitive_c",
            "/a/dependency_b",
            "/a/dependency_b/transitive_a",
            "/a/dependency_b/transitive_a/distant_transitive_a",
            "/a/dependency_b/transitive_a/distant_transitive_b",
            "/a/dependency_b/transitive_a/distant_transitive_c",
            "/a/dependency_b/transitive_b",
            "/a/dependency_b/transitive_b/distant_transitive_a",
            "/a/dependency_b/transitive_b/distant_transitive_b",
            "/a/dependency_b/transitive_b/distant_transitive_c",
            "/a/dependency_b/transitive_c",
            "/a/dependency_b/transitive_c/distant_transitive_a",
            "/a/dependency_b/transitive_c/distant_transitive_b",
            "/a/dependency_b/transitive_c/distant_transitive_c",
            "/a/dependency_b/transitive_d",
            "/a/dependency_b/transitive_d/distant_transitive_a",
            "/a/dependency_b/transitive_d/distant_transitive_b",
            "/a/dependency_b/transitive_d/distant_transitive_c",
            "/a/dependency_c",
            "/a/dependency_c/transitive_a",
            "/a/dependency_c/transitive_a/distant_transitive_a",
            "/a/dependency_c/transitive_a/distant_transitive_b",
            "/a/dependency_c/transitive_a/distant_transitive_c",
            "/a/dependency_c/transitive_b",
            "/a/dependency_c/transitive_b/distant_transitive_a",
            "/a/dependency_c/transitive_b/distant_transitive_b",
            "/a/dependency_c/transitive_b/distant_transitive_c",
            "/a/dependency_c/transitive_c",
            "/a/dependency_c/transitive_c/distant_transitive_a",
            "/a/dependency_c/transitive_c/distant_transitive_b",
            "/a/dependency_c/transitive_c/distant_transitive_c",
            "/a/dependency_c/transitive_d",
            "/a/dependency_c/transitive_d/distant_transitive_a",
            "/a/dependency_c/transitive_d/distant_transitive_b",
            "/a/dependency_c/transitive_d/distant_transitive_c",
            "/a/dependency_d",
            "/a/dependency_d/transitive_a",
            "/a/dependency_d/transitive_a/distant_transitive_a",
            "/a/dependency_d/transitive_a/distant_transitive_b",
            "/a/dependency_d/transitive_a/distant_transitive_c",
            "/a/dependency_d/transitive_b",
            "/a/dependency_d/transitive_b/distant_transitive_a",
            "/a/dependency_d/transitive_b/distant_transitive_b",
            "/a/dependency_d/transitive_b/distant_transitive_c",
            "/a/dependency_d/transitive_c",
            "/a/dependency_d/transitive_c/distant_transitive_a",
            "/a/dependency_d/transitive_c/distant_transitive_b",
            "/a/dependency_d/transitive_c/distant_transitive_c",
            "/a/dependency_d/transitive_d",
            "/a/dependency_d/transitive_d/distant_transitive_a",
            "/a/dependency_d/transitive_d/distant_transitive_b",
            "/a/dependency_d/transitive_d/distant_transitive_c",
        ];

        assert_eq!(
            &ids,
            expected,
            "{}",
            TextDiff::from_lines(&format!("{expected:#?}"), &format!("{ids:#?}"))
                .iter_all_changes()
                .map(|change| {
                    let sign = match change.tag() {
                        ChangeTag::Delete => "-",
                        ChangeTag::Insert => "+",
                        ChangeTag::Equal => " ",
                    };
                    format!("{sign}{change}")
                })
                .fold(String::new(), |acc, c| format!("{acc}{c}"))
        );
    }

    fn build_graph() -> (Vec<SchemaKey>, Schemas<SchemaKey>, Sources) {
        let mut schemas: Schemas<SchemaKey> = Schemas::new();
        let deserializers = Deserializers::new(vec![("json", Box::new(deserialize_json))]);
        let mut sources = Sources::new(vec![], &deserializers).unwrap();
        schemas.start_txn();
        let mut root_keys = vec![];
        // builds subschemas
        for r in 'a'..='d' {
            let root_key = schemas.insert(create_schema(&r.to_string())).unwrap();
            root_keys.push(root_key);
            for n in 'a'..='d' {
                let id = format!("{r}/subschema_{n}");
                let sub_key = schemas.insert(create_schema(id)).unwrap();
                {
                    let sub = schemas.get_mut_unchecked(sub_key);
                    sub.parent = Some(root_key);
                }
                {
                    let root = schemas.get_mut_unchecked(root_key);
                    root.subschemas.push(sub_key);
                }
                for n2 in 'a'..'d' {
                    let id = format!("{r}/subschema_{n}/nested_subschema_{n2}");
                    let sub_sub_key = schemas.insert(create_schema(id)).unwrap();
                    {
                        let sub_sub = schemas.get_mut_unchecked(sub_sub_key);
                        sub_sub.parent = Some(sub_key);
                    }
                    {
                        let parent = schemas.get_mut_unchecked(sub_key);
                        parent.subschemas.push(sub_sub_key);
                    }
                    for n3 in 'a'..'d' {
                        let sub_sub_sub_key = schemas
                            .insert(create_schema(format!(
                                "{r}/subschema_{n}/nested_subschema_{n2}/deeply_nested_subschema_{n3}"
                            )))
                            .unwrap();
                        {
                            let sub_sub_sub = schemas.get_mut_unchecked(sub_sub_sub_key);
                            sub_sub_sub.parent = Some(sub_sub_key);
                        }
                        {
                            let parent = schemas.get_mut_unchecked(sub_sub_key);
                            parent.subschemas.push(sub_sub_key);
                        }
                    }
                }
            }
            for d in 'a'..='d' {
                let dep_key = schemas
                    .insert(create_schema(&format!("{r}/dependency_{d}")))
                    .unwrap();
                {
                    let root = schemas.get_mut_unchecked(root_key);
                    root.dependencies.push(dep_key);
                }
                {
                    let dep = schemas.get_mut_unchecked(dep_key);
                    dep.dependents.push(root_key);
                }
                for t in 'a'..='d' {
                    let transitive_dep_key = schemas
                        .insert(create_schema(format!("{r}/dependency_{d}/transitive_{t}")))
                        .unwrap();
                    {
                        let dep = schemas.get_mut_unchecked(dep_key);
                        dep.dependencies.push(transitive_dep_key);
                    }
                    {
                        let transitive_dep = schemas.get_mut_unchecked(transitive_dep_key);
                        transitive_dep.dependents.push(dep_key);
                    }
                    for t2 in 'a'..'d' {
                        let transitive_dep_key_2 = schemas
                            .insert(create_schema(format!(
                                "{r}/dependency_{d}/transitive_{t}/distant_transitive_{t2}"
                            )))
                            .unwrap();
                        {
                            let transitive_dep = schemas.get_mut_unchecked(transitive_dep_key);
                            transitive_dep.dependencies.push(transitive_dep_key_2);
                        }
                        {
                            let transitive_dep_2 = schemas.get_mut_unchecked(transitive_dep_key_2);
                            transitive_dep_2.dependents.push(transitive_dep_key);
                        }
                    }
                }
            }
        }
        schemas.accept_txn();
        for (_, schema) in schemas.iter_compiled() {
            let id = schema.id.clone().unwrap();
            sources
                .insert(
                    Source::Value(id.clone(), json!({"$id": id.clone()})),
                    &deserializers,
                )
                .unwrap();
        }
        (root_keys, schemas, sources)
    }
    fn create_test_uri(uri: impl ToString) -> AbsoluteUri {
        let mut uri = uri.to_string();
        if !uri.starts_with("https") {
            if uri.starts_with('/') {
                uri = uri.strip_prefix('/').unwrap().to_string();
            }
            uri = format!("https://test.com/{uri}");
        }
        AbsoluteUri::parse(&uri).unwrap()
    }
    fn metaschema() -> AbsoluteUri {
        "https://json-schema.org/draft/2020-12/schema"
            .parse()
            .unwrap()
    }
    fn create_schema(uri: impl ToString) -> CompiledSchema<SchemaKey> {
        let uri: AbsoluteUri = create_test_uri(uri);
        let metaschema = metaschema();
        CompiledSchema {
            id: Some(uri.clone()),
            anchors: Vec::default(),
            parent: None,
            dependencies: vec![],
            dependents: vec![],
            handlers: vec![].into_boxed_slice(),
            metaschema,
            source_path: Pointer::default(),
            source_uri: uri.clone(),
            subschemas: vec![],
            uris: vec![uri],
        }
    }
}
