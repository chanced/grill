use super::Schemas;
use crate::{source::Sources, Schema};
use either::Either;
use std::{
    collections::HashSet,
    iter::{once, Chain, Copied, Empty, Once},
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

pub struct TransitiveDependencies<'i, Key: slotmap::Key> {
    traverse: Slice<'i, Key, Once<Key>>,
}
impl_iterator!(TransitiveDependencies, transitive_dependencies);

impl<'i, Key> TransitiveDependencies<'i, Key>
where
    Key: slotmap::Key,
{
    pub(crate) fn new(key: Key, schemas: &'i Schemas<Key>, sources: &'i Sources) -> Self {
        let mut this = Self {
            traverse: DepthFirst::new(once(key), schemas, sources, transitive_dependencies),
        };
        // dumping the first result, which is `key`
        this.next();
        this
    }
}

pub struct DirectDependencies {}

type Slice<'i, Key, Seed> = DepthFirst<
    'i,
    Key,
    Seed,
    Copied<slice::Iter<'i, Key>>,
    for<'x> fn(&'x Schema<'i, Key>) -> Copied<std::slice::Iter<'i, Key>>,
>;

type Instance<'i, Key, Seed> = DepthFirst<
    'i,
    Key,
    Seed,
    Either<Once<Key>, Empty<Key>>,
    for<'x> fn(&'x Schema<'i, Key>) -> Either<Once<Key>, Empty<Key>>,
>;

struct DepthFirst<'i, Key, Seed, Iter, Func>
where
    Key: slotmap::Key,
    Seed: IntoIterator<Item = Key>,
    Iter: Iterator<Item = Key>,
    Func: for<'x> Fn(&'x Schema<'i, Key>) -> Iter,
{
    func: Func,
    first: Option<Key>,
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
    Func: for<'x> Fn(&'x Schema<'i, Key>) -> Iter,
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
            handle(&schema).chain(Box::new(once(first)) as Box<dyn Iterator<Item = Key>>)
        });

        let sent = HashSet::new();
        Self {
            first,
            func: handle,
            seed,
            queue,
            sent,
            sources,
            schemas,
        }
    }
    fn exec<'x>(&'x self, schema: &'x Schema<'i, Key>) -> Iter {
        (self.func)(schema)
    }
}

impl<'i, Key, Seed, Iter, Func> Iterator for DepthFirst<'i, Key, Seed, Iter, Func>
where
    Key: slotmap::Key,
    Seed: IntoIterator<Item = Key>,
    Iter: 'i + Iterator<Item = Key>,
    Func: for<'x> Fn(&'x Schema<'i, Key>) -> Iter,
{
    type Item = Schema<'i, Key>;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self
            .first
            .take()
            .or_else(|| self.queue.as_mut().map(Iterator::next).flatten())
            .or_else(|| self.seed.next())?;

        if self.sent.contains(&key) {
            return self.next();
        }
        let schema = self.schemas.get(key, self.sources).unwrap();
        self.sent.insert(key);
        self.queue = self.queue.take().map(|queue| {
            self.exec(&schema)
                .chain(Box::new(queue) as Box<dyn Iterator<Item = Key>>)
        });
        Some(schema)
    }
}

fn empty<'i, Key>(_: Schema<'i, Key>) -> Empty<Key>
where
    Key: slotmap::Key,
{
    std::iter::empty()
}

fn transitive_dependencies<'i, 'x, Key>(schema: &'x Schema<'i, Key>) -> Copied<slice::Iter<'i, Key>>
where
    Key: slotmap::Key,
    'x: 'i,
{
    schema.dependencies.iter().copied()
}

fn ancestors<'i, Key: slotmap::Key>(schema: Schema<'i, Key>) -> Either<Once<Key>, Empty<Key>> {
    if schema.dependencies.is_empty() {
        either::Right(std::iter::empty())
    } else {
        either::Left(std::iter::once(schema.key))
    }
}

// pub struct AllDependents<'i, Key: slotmap::Key> {
//     traverse: Slice<'i, Key>,
// }
// impl_iterator!(AllDependents, all_dependents);

// pub struct DirectDependents<'i, Key: slotmap::Key> {
//     traverse: Slice<'i, Key>,
// }

#[cfg(test)]
mod tests {
    use crate::{schema::CompiledSchema, AbsoluteUri, SchemaKey};

    use super::*;

    #[test]
    fn test_transitive_dependencies() {
        let leaf1: CompiledSchema<SchemaKey> = CompiledSchema {
            id: Some(AbsoluteUri::parse("leaf1").unwrap()),
            anchors: Default::default(),
            container: None,
            dependencies: vec![].into_boxed_slice(),
            dependents: vec![].into_boxed_slice(),
            handlers: vec![].into_boxed_slice(),
            metaschema: "https://json-schema.org/draft/2020-12/schema"
                .parse()
                .unwrap(),
            source_path: Default::default(),
            source_uri: "https://example.com/leaf1".parse().unwrap(),
            subschemas: vec![].into_boxed_slice(),
            uris: vec![].into_boxed_slice(),
        };
    }
}
