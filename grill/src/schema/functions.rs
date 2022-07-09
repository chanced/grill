use crate::applicator::{ExecutorFn, SetupFn};
use parking_lot::{RwLock, RwLockWriteGuard};
use std::sync::Arc;

#[derive(Clone)]
pub(super) struct Functions {
    setup_fns: Arc<RwLock<Arc<Vec<Box<SetupFn>>>>>,
    executor_fns: Arc<RwLock<Arc<Vec<Box<ExecutorFn>>>>>,
}

impl Functions {
    pub(super) fn new() -> Self {
        Self {
            setup_fns: Arc::new(RwLock::new(Arc::new(Vec::new()))),
            executor_fns: Arc::new(RwLock::new(Arc::new(Vec::new()))),
        }
    }

    pub(super) fn set_executors(&self, fns: Vec<Box<ExecutorFn>>) {
        let mut exec = self.executor_fns.write();
        *exec = Arc::new(fns);
    }

    pub(super) fn set_setup(&self, fns: Vec<Box<SetupFn>>) {
        let mut setup = self.setup_fns.write();
        *setup = Arc::new(fns);
    }

    pub(super) fn executor_fns(&self) -> Vec<Box<ExecutorFn>> {
        let v = {
            let guard = self.executor_fns.read();
            guard.clone()
        };
        v.to_vec()
    }

    pub(super) fn setup_fns(&self) -> Vec<Box<SetupFn>> {
        let v = {
            let guard = self.setup_fns.read();
            guard.clone()
        };
        v.to_vec()
    }
    pub(super) fn write(&self) -> GuardedFunctions {
        GuardedFunctions {
            setup_fns: self.setup_fns.write(),
            exeec_fns: self.executor_fns.write(),
        }
    }
}

pub(super) struct GuardedFunctions<'a> {
    setup_fns: RwLockWriteGuard<'a, Arc<Vec<Box<SetupFn>>>>,
    exeec_fns: RwLockWriteGuard<'a, Arc<Vec<Box<ExecutorFn>>>>,
}
impl GuardedFunctions<'_> {
    pub(super) fn update(
        &mut self,
        setup_fns: Vec<Box<SetupFn>>,
        executor_fns: Vec<Box<ExecutorFn>>,
    ) {
        *self.setup_fns = Arc::new(setup_fns);
        *self.exeec_fns = Arc::new(executor_fns);
    }
}
