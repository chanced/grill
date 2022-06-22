use crate::{Annotation, Applicator, Graph,  Result, Schema};
use parking_lot::RwLock;
use std::{ sync::Arc};
struct Layer<T: Clone + Send + Sync + 'static>(T);

/// Manages schemas and extensions.
#[derive(Clone)]
pub struct Interrogator {
    // schemas: Arc<DashMap<String, Schema>>,
    graph: Arc<RwLock<Graph>>,
    // resolver: Arc<dyn Resolver>,
    // applicators: Arc<RwLock<dyn Fn(&Schema) -> Arc<dyn FnOnce(&mut Schema) -> Result<(), Error>>>>,
}
// impl<R> Interrogator {}
impl Interrogator {
    pub fn new() -> Self {
        Self {
            graph: Arc::new(RwLock::new(Graph::new(&[]).unwrap())),
            // resolver: Arc::new(RwLock::new(Resolver::new())),
            // applicators: Arc::new(RwLock::new(Applicators::new())),
        }
    }

    /// temp method to see if this will execute
    pub fn call<A>(&self, applicator: A, schema: Schema) -> Result<Annotation>
    where
        A: Applicator,
    {
        let applicator = applicator.setup(self.clone(), schema);
        let v = Value::Null;
        applicator
        todo!()
    }
}
