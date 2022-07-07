use crate::applicator::{ExecutorFn, SetupFn};
use arc_swap::ArcSwap;
use std::sync::Arc;

#[derive(Clone)]
pub(super) struct Functions {
    setup_fns: Arc<ArcSwap<Vec<Box<SetupFn>>>>,
    executor_fns: Arc<ArcSwap<Vec<Box<ExecutorFn>>>>,
}

impl Functions {
    pub(super) fn new() -> Self {
        Self {
            setup_fns: Arc::new(ArcSwap::from_pointee(Vec::new())),
            executor_fns: Arc::new(ArcSwap::from_pointee(Vec::new())),
        }
    }
    pub(super) fn update(&self, fns: Functions) {
        self.setup_fns.store(fns.setup_fns.load().clone());
        self.executor_fns.store(fns.executor_fns.load().clone());
    }
    pub(super) fn store_executors(&self, fns: Vec<Box<ExecutorFn>>) {
        let mut f = self.executor_fns.swap(Arc::new(fns));
    }

    pub(super) fn store_setup(&self, fns: Vec<Box<SetupFn>>) {
        let mut f = self.setup_fns.swap(Arc::new(fns));
    }

    pub(super) fn executor_fns(&self) -> Arc<Vec<Box<ExecutorFn>>> {
        self.executor_fns.load().clone()
    }

    pub(super) fn setup_fns(&self) -> Arc<Vec<Box<SetupFn>>> {
        self.setup_fns.load().clone()
    }

    pub(super) fn publish_from(&self, functions: &Functions) {
        self.executor_fns.swap(functions.executor_fns());
        self.setup_fns.swap(functions.setup_fns());
    }
}
