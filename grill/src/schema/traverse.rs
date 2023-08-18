use super::Schemas;
use crate::{source::Sources, Schema};
use either::Either;
use std::{
    collections::HashSet,
    iter::{once, Chain, Copied, Empty, Once},
    marker::PhantomData,
    slice,
};

macro_rules! impl_iterator {
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
    traverse: Slice<'i, Key, Once<Key>>,
}

impl_iterator!(AllDependents, all_dependents);

/// A [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
/// [`Iterator`] which traverses both direct and indirect dependencies of
/// a [`Schema`].
pub struct TransitiveDependencies<'i, Key: slotmap::Key> {
    traverse: Slice<'i, Key, Once<Key>>,
}

impl_iterator!(TransitiveDependencies, transitive_dependencies);

impl<'i, Key> TransitiveDependencies<'i, Key>
where
    Key: slotmap::Key,
{
    pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
        Self {
            traverse: DepthFirst::new(once(key), schemas, sources, transitive_dependencies),
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
/// An [`Iterator`] over dependencies of a given [`Schema`]
pub struct DirectDependencies {}

struct DepthFirst<'i, Key, Seed, Iter, Func>
where
    Key: slotmap::Key,
    Seed: IntoIterator<Item = Key>,
    Iter: Iterator<Item = Key>,
    Func: Fn(Schema<'i, Key>) -> Iter,
{
    handle: Func,
    seed: Seed::IntoIter,
    queue: Option<Chain<Iter, Box<dyn Iterator<Item = Key> + 'i>>>,
    sent: HashSet<Key>,
    schemas: &'i Schemas<Key>,
    sources: &'i Sources,
}

impl<'i, Key, Seed, Iter, Func> DepthFirst<'i, Key, Seed, Iter, Func>
where
    Key: slotmap::Key,
    Seed: IntoIterator<Item = Key>,
    Iter: 'i + Iterator<Item = Key>,
    Func: Fn(Schema<'i, Key>) -> Iter,
{
    pub(crate) fn new(
        seed: Seed,
        schemas: &'i Schemas<Key>,
        sources: &'i Sources,
        handle: Func,
    ) -> Self {
        let mut seed = seed.into_iter();
        let first = seed.next();
        let queue = first.map(|first| {
            let schema = schemas.get(first, sources).unwrap();
            handle(schema).chain(Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Key>>)
        });

        let sent = HashSet::new();
        Self {
            handle,
            seed,
            queue,
            sent,
            schemas,
            sources,
        }
    }
    fn exec(&self, schema: Schema<'i, Key>) -> Iter {
        (self.handle)(schema)
    }
}

impl<'i, Key, Seed, Iter, Func> Iterator for DepthFirst<'i, Key, Seed, Iter, Func>
where
    Key: slotmap::Key,
    Seed: IntoIterator<Item = Key>,
    Iter: 'i + Iterator<Item = Key>,
    Func: Fn(Schema<'i, Key>) -> Iter,
{
    type Item = Schema<'i, Key>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.queue.as_mut().and_then(Iterator::next);
        let key = next.or_else(|| {
            self.seed.next().and_then(|key| {
                let schema = self.schemas.get_unchecked(key, self.sources);
                self.queue = Some(
                    self.exec(schema)
                        .chain(Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Key>>),
                );
                self.queue.as_mut().and_then(Iterator::next)
            })
        })?;

        if self.sent.contains(&key) {
            return self.next();
        }
        let schema = self.schemas.get(key, self.sources).unwrap();
        self.sent.insert(key);
        self.queue = self.queue.take().map(|queue| {
            self.exec(schema.clone())
                .chain(Box::new(queue) as Box<dyn Iterator<Item = Key>>)
        });
        Some(schema)
    }
}

fn transitive_dependencies<Key>(schema: Schema<'_, Key>) -> std::vec::IntoIter<Key>
where
    Key: slotmap::Key,
{
    #[allow(clippy::unnecessary_to_owned)]
    schema.dependencies.into_owned().into_iter()
}

fn empty<Key>(_: Schema<'_, Key>) -> Empty<Key>
where
    Key: slotmap::Key,
{
    std::iter::empty()
}

fn ancestors<Key: slotmap::Key>(schema: Schema<'_, Key>) -> Either<Once<Key>, Empty<Key>> {
    if schema.dependencies.is_empty() {
        either::Right(std::iter::empty())
    } else {
        either::Left(std::iter::once(schema.key))
    }
}

fn all_dependents() {}

// impl_iterator!(AllDependents, all_dependents);

// pub struct DirectDependents<'i, Key: slotmap::Key> {
//     traverse: Slice<'i, Key>,
// }

type Slice<'i, Key, Seed> = DepthFirst<
    'i,
    Key,
    Seed,
    std::vec::IntoIter<Key>,
    fn(Schema<'i, Key>) -> std::vec::IntoIter<Key>,
>;

type Instance<'i, Key, Seed> = DepthFirst<
    'i,
    Key,
    Seed,
    Either<Once<Key>, Empty<Key>>,
    fn(Schema<'i, Key>) -> Either<Once<Key>, Empty<Key>>,
>;

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use jsonptr::Pointer;
    use lazy_static::__Deref;
    use serde_json::json;
    use slotmap::SlotMap;

    use crate::{
        schema::CompiledSchema,
        source::{deserialize_json, Deserializers, Source},
        AbsoluteUri, SchemaKey,
    };

    use super::*;

    #[test]
    fn test_transitive_dependencies() {
        let (root_keys, schemas, sources) = build_graph();
        println!("{root_keys:?}");
        let traverse = TransitiveDependencies::new(root_keys[0], &schemas, &sources);
        let ids = traverse
            .map(|schema| schema.id.unwrap().path_or_nss().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            &ids,
            &[
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
                "/a/dependency_d/transitive_d/distant_transitive_c"
            ],
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

    fn create_schema(uri: impl ToString) -> CompiledSchema<SchemaKey> {
        let mut uri = uri.to_string();
        if !uri.starts_with("https") {
            if uri.starts_with('/') {
                uri = uri.strip_prefix('/').unwrap().to_string();
            }
            uri = format!("https://test.com/{uri}");
        }
        let uri: AbsoluteUri = uri.parse().unwrap();
        let metaschema: AbsoluteUri = "https://json-schema.org/draft/2020-12/schema"
            .parse()
            .unwrap();
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
