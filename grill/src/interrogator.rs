use crate::{Applicator, Extensions, Graph, Implementation, Injectable};
use parking_lot::RwLock;
use std::sync::Arc;
struct Layer<T: Clone + Send + Sync + 'static>(T);

/// Manages schemas and extensions.
#[derive(Clone)]
pub struct Interrogator<I> {
    // schemas: Arc<DashMap<String, Schema>>,
    extensions: Arc<RwLock<Extensions>>,
    graph: Arc<RwLock<Graph>>,
    // resolver: Arc<dyn Resolver>,
    // applicators: Arc<RwLock<dyn Fn(&Schema) -> Arc<dyn FnOnce(&mut Schema) -> Result<(), Error>>>>,
    implementation: I,
}
// impl<R> Interrogator {}
impl<I> Interrogator<I>
where
    I: Implementation + 'static,
{
    pub fn new(implementation: I) -> Self {
        Self {
            implementation,
            extensions: Arc::new(RwLock::new(Extensions::new())),
            graph: Arc::new(RwLock::new(Graph::new(&[]).unwrap())),
            // resolver: Arc::new(RwLock::new(Resolver::new())),
            // applicators: Arc::new(RwLock::new(Applicators::new())),
        }
    }
    pub fn implementation(&self) -> &I {
        &self.implementation
    }
    pub fn context<T>(&mut self, ctx: T)
    where
        T: Send + Sync + Clone + 'static,
    {
        let mut exts = self.extensions.write();
        exts.insert(Layer(ctx));
    }

    /// temp method to see if this will execute
    pub fn call<A, V>(&self, run: A)
    where
        A: Applicator<V, I>,
    {
        let bf = run.setup(self.clone());
    }

    /// Resolves a registered `Injectable`
    pub fn resolve<T, V>(&self) -> V
    where
        V: Clone + Send + Sync + 'static,
        T: Injectable<Value = V>,
    {
        let exts = self.extensions.write();
        let layer = exts.get::<Layer<V>>();
        if layer.is_none() {
            panic!("layer not found")
        }
        V::from(layer.unwrap().0.clone())
    }
}
