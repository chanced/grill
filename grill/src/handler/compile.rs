use std::marker::PhantomData;

#[derive(Debug)]
pub struct Compile<'i> {
    marker: PhantomData<&'i ()>, // location: Location,
                                 // anchors: Vec<(String, Anchor<'s>)>,
                                 // schemas: HashMap<Keyword<'s>, Subschema<'s>>,
                                 // numbers: HashMap<Keyword<'s>, &'s Number>,
                                 // references: HashMap<Keyword<'s>, &'s str>,
}

// impl<'s> Compile<'s> {
//     #[must_use]
//     pub fn new(location: Location) -> Self {
//         Self {
//             location,
//             anchors: Vec::new(),
//             schemas: HashMap::default(),
//             numbers: HashMap::default(),
//             references: HashMap::default(),
//         }
//     }

//     pub fn anchor(&mut self, anchor: Anchor<'s>) {
//         self.anchors
//             .push((self.location.absolute_keyword_location.clone(), anchor));
//     }
//     pub fn schema(&mut self, keyword: Keyword<'s>, schema: Subschema<'s>) {
//         self.schemas.insert(keyword, schema);
//     }
//     pub fn reference(&mut self, keyword: Keyword<'s>, reference: &'s str) {
//         self.references.insert(keyword, reference);
//     }

//     /// # Errors
//     pub fn number<'x>(&'x mut self, keyword: Keyword<'s>, number: &'s Number) {
//         self.numbers.entry(keyword).or_insert_with(|| number);
//     }
// }
