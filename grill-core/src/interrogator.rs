// use std::sync::Arc;

// use dashmap::DashMap;
// use parking_lot::RwLock;

// use crate::{error::Error, graph::Graph, Resolver, Schema};

use std::sync::Arc;

use crate::{Applicator, Error, Eval, Extensions, Graph, Injectable, Result, Value};
use parking_lot::RwLock;
struct Layer<T: Clone + Send + Sync + 'static>(T);
#[derive(Clone)]
pub struct Interrogator {
    // schemas: Arc<DashMap<String, Schema>>,
    extensions: Arc<RwLock<Extensions>>,
    graph: Arc<RwLock<Graph>>,
    // resolver: Arc<dyn Resolver>,
    // applicators: Arc<RwLock<dyn Fn(&Schema) -> Arc<dyn FnOnce(&mut Schema) -> Result<(), Error>>>>,
}
// impl<R> Interrogator {}
impl Interrogator {
    pub fn new() -> Self {
        Interrogator {
            extensions: Arc::new(RwLock::new(Extensions::new())),
            graph: Arc::new(RwLock::new(Graph::new(&[]).unwrap())),
            // resolver: Arc::new(RwLock::new(Resolver::new())),
            // applicators: Arc::new(RwLock::new(Applicators::new())),
        }
    }

    pub fn context<T>(&mut self, ctx: T)
    where
        T: Send + Sync + Clone + 'static,
    {
        let mut exts = self.extensions.write();
        exts.insert(Layer(ctx));
    }

    /// temp method to see if this will execute
    pub fn call<A, V, N>(&self, run: A)
    where
        N: FnOnce(Value) -> Result<Eval>,
        A: Applicator<V, N>,
    {
        let bf = run.setup(self.clone());
    }

    /// Provides
    pub fn resolve<I, T>(&self) -> I
    where
        T: Clone + Send + Sync + 'static,
        I: Injectable<Value = T>,
    {
        let exts = self.extensions.write();
        let layer = exts.get::<Layer<T>>();
        if layer.is_none() {
            // TODO: error
            panic!("layer not found")
        }
        I::from(layer.unwrap().0.clone())
    }
}
