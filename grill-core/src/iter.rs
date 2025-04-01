// //! Various [`Iterator`]s.

// use slotmap::Key;

// use crate::{
//     key::InternalKey,
//     schema::{CompiledSchema, Schemas},
//     source::Sources,
// };

// /// TODO: stubbed
// pub struct Ancestors<'int, S> {
//     _schema: S,
//     _marker: std::marker::PhantomData<&'int ()>,
// }

// pub fn all_compiled_schemas<S>(schemas: &Schemas<S>) -> AllCompiledSchemas<'_, S> {
//     AllCompiledSchemas::new(schemas)
// }

// #[derive(Debug)]
// pub struct AllCompiledSchemas<'int, S> {
//     iter: slotmap::basic::Iter<'int, InternalKey, S>,
// }

// impl<'int, S> AllCompiledSchemas<'int, S> {
//     pub fn new(schemas: &'int Schemas<S>) -> Self {
//         Self {
//             iter: schemas.map.iter(),
//         }
//     }
//     pub fn into_all_schemas(self, sources: &'int Sources) -> AllSchemas<'int, S> {
//         AllSchemas {
//             compiled: self,
//             sources,
//         }
//     }
// }
// impl<'int, S> Iterator for AllCompiledSchemas<'int, S> {
//     type Item = &'int S;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(|(_, v)| v)
//     }
// }

// pub struct Iter<'int, I, S> {
//     iter: I,
//     schemas: &'int Schemas<S>,
//     sources: &'int Sources,
// }

// impl<'int, I, S> Iterator for Iter<'int, I, S>
// where
//     I: Iterator,
//     I::Item: AsRef,
//     S: CompiledSchema,
//     K: Key,
// {
//     type Item = S::Schema<'int>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter
//             .next()
//             .map(|i| *i.as_ref())
//             .map(|k| self.schemas.get_compiled(k).to_schema(self.sources))
//     }
// }

// impl<'int, I, S> Iter<'int, I, S>
// where
//     I: Iterator<Item = K>,
//     K: Key,
// {
//     pub fn new(schemas: &'int Schemas<S>, sources: &'int Sources, keys: I) -> Self {
//         Self {
//             iter: keys,
//             schemas,
//             sources,
//         }
//     }
// }

// pub struct AllSchemas<'int, S> {
//     compiled: AllCompiledSchemas<'int, S>,
//     sources: &'int Sources,
// }

// impl<'int, S> Iterator for AllSchemas<'int, S>
// where
//     S: CompiledSchema,
// {
//     type Item = S::Schema<'int>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.compiled.next().map(|s| s.to_schema(self.sources))
//     }
// }
