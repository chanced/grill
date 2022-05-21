// use std::{collections::BTreeMap, sync::Arc};

// pub struct Location {
//     parts: Arc<Vec<Arc<str>>>,
// }

// use crate::{BoxedError, Value};
// /// Annotations make up the tree
// pub enum Annotation {
//     /// Value is a
//     Value(Arc<Value>),
//     Annotations(Annotations),
//     Error(BoxedError),
// }

// pub struct Annotations {
//     keyword_location: Arc<Location>,
//     instance_location: Arc<Location>,
//     absolute_keyword_location: Arc<Location>,
//     inner: Arc<BTreeMap<String, Annotation>>,
//     is_valid: bool,
// }
