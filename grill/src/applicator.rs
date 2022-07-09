use crate::{Error, Evaluation, Interrogator, MetaSchema, Next, Schema};
use dyn_clone::{clone_trait_object, DynClone};
use parking_lot::{Mutex, MutexGuard};
use serde_json::Value;
use std::sync::Arc;
use uniresid::Uri;

pub trait ExecutorFnTrait:
    DynClone + Fn(&Value, Evaluation, Next) -> Result<Evaluation, Error>
{
}

impl<F> ExecutorFnTrait for F where
    F: Clone + Fn(&Value, Evaluation, Next) -> Result<Evaluation, Error>
{
}

clone_trait_object!(ExecutorFnTrait);

pub type InitFn = dyn Fn(Interrogator, Schema) -> Result<Option<Box<SetupFn>>, Error>;

/// Annotates an [`Evaluation`] with relevant information pertinent to the
/// [`Schema`] for the given [`OutputFmt`].
pub type ExecutorFn = dyn 'static + Send + Sync + ExecutorFnTrait;

pub trait SetupFnTrait:
    DynClone + Fn(Interrogator, Schema) -> Result<Box<ExecutorFn>, Error>
{
}

impl<F> SetupFnTrait for F where
    F: Clone + Fn(Interrogator, Schema) -> Result<Box<ExecutorFn>, Error>
{
}

clone_trait_object!(SetupFnTrait);

/// Returned from [`Applicator::init`](Applicator::init) to setup the `Applicator` for use.
pub type SetupFn = dyn 'static + Send + Sync + SetupFnTrait;

/// Annotates an [`Evaluation`] with relevant data and validation state for a
/// given [`Value`] depending on the [`Schema`] and the specified
/// [`OutputFmt`](crate::OutputFmt).
///
/// The `Applicator` is created in a 3 stage process of `init`, `setup`, and `execute`.
///
/// ## Stages
///
/// ### Initialization
/// [`init`](crate::Applicator::init) determines whether the `Applicator` is
/// applicable to the [`Schema`] or [`SubSchema`].
///
/// [`init`](crate::Applicator::init) is run upon creation of each [`Schema`]
/// and [`SubSchema`] and when a new `Applicator` is added to the
/// [`Interrogator`].
///
/// If relevant, [`init`](crate::Applicator::init) should update the [`Schema`]
/// with pertinent information for the `Applicator`, including creating and
/// associating [`SubSchema`](crate::schema::SubSchema) and inidicating
/// references. Finally, it should return a [`SetupFn`](SetupFn).
///
/// **Note**: The graph of [`Schema`] contained within the [`Interrogator`] will
/// not be fully populated during the initialization stage.
///
///
/// ### Setup
///
/// The [`SetupFn`](crate::applicator::SetupFn) returned by
/// [`init`](crate::Applicator::init) is invoked after the [`Interrogator`] has
/// initialized all `Applicator`s for all associated [`Schema`]s and their
/// [`SubSchema`](crate::schema::SubSchema)s.
///
/// The [`SetupFn`](crate::applicator::SetupFn) should load any relevant data
/// from the [`Schema`] and, further determine if the `Applicator` is applicable
/// to the [`Schema`]. If applicable, it should return an
/// [`ExecutorFn`](ExecutorFn).
///
/// **Note**: The [`Interrogator`]'s [`Schema`] graph will be fully populated at
/// this stage so references are safe to resolve.
///
/// ### Execution
/// The [`ExecutorFn`] returned by [`SetupFn`](crate::applicator::SetupFn) is
/// invoked for each call to [`Schema::evaluate`](crate::Schema::evaluate). This
/// function should evaluate the [`Value`] annotate the [`Evaluation`] based
/// upon the specified [`OutputFmt`](crate::OutputFmt).
///
///
/// ## Implementation
/// Each stage MUST be deterministic. Failing to do so could result in the [`Interrogator`]
///

pub trait Applicator: DynClone + Send + Sync {
    /// Initializes the `Applicator` with the [`Interrogator`] for the given [`Schema`].
    fn init(
        &self,
        interrogator: Interrogator,
        schema: Schema,
    ) -> Result<Option<Box<SetupFn>>, Error>;
}
clone_trait_object!(Applicator);

impl<F> Applicator for F
where
    F: 'static
        + DynClone
        + Send
        + Sync
        + Fn(Interrogator, Schema) -> Result<Option<Box<SetupFn>>, Error>,
{
    fn init(
        &self,
        interrogator: Interrogator,
        schema: Schema,
    ) -> Result<Option<Box<SetupFn>>, Error> {
        self(interrogator, schema)
    }
}

pub(crate) struct GuardedApplicators<'a> {
    current: MutexGuard<'a, Vec<Box<dyn Applicator>>>,
    pending: MutexGuard<'a, Vec<Box<dyn Applicator>>>,
}
impl GuardedApplicators<'_> {
    pub(crate) fn update(
        &mut self,
        current: Vec<Box<dyn Applicator>>,
        pending: Vec<Box<dyn Applicator>>,
    ) {
        *self.current = current;
        *self.pending = pending;
    }
}

#[derive(Clone)]
pub(crate) struct Applicators {
    current: Arc<Mutex<Vec<Box<dyn Applicator>>>>,
    pending: Arc<Mutex<Vec<Box<dyn Applicator>>>>,
}
impl Applicators {
    pub(crate) fn new() -> Self {
        Applicators {
            current: Arc::new(Mutex::new(Vec::new())),
            pending: Arc::new(Mutex::new(Vec::new())),
        }
    }
    /// Returns a cloned tuple of `(current, pending)` Applicators.
    pub(crate) fn clone_functions(&self) -> (Vec<Box<dyn Applicator>>, Vec<Box<dyn Applicator>>) {
        let current = self.current.lock();
        let pending = self.pending.lock();
        (current.clone(), pending.clone())
    }
    pub(crate) fn lock(&self) -> GuardedApplicators {
        GuardedApplicators {
            current: self.current.lock(),
            pending: self.pending.lock(),
        }
    }

    pub(crate) fn all(&self) -> Vec<Box<dyn Applicator>> {
        let current = self.current.lock();
        let pending = self.pending.lock();
        let mut res = Vec::with_capacity(current.len() + pending.len());
        res.extend(current.iter().chain(pending.iter()).cloned());
        res
    }
    pub(crate) fn push_to_current(&self, applicator: impl Applicator + 'static) {
        let mut current = self.current.lock();
        current.push(Box::new(applicator));
    }

    pub(crate) fn push(&self, applicator: impl Applicator + 'static) {
        let mut pending = self.pending.lock();
        pending.push(Box::new(applicator));
    }
    pub(crate) fn extend(&self, iter: impl Iterator<Item = Box<dyn Applicator>>) {
        let mut pending = self.pending.lock();
        pending.extend(iter);
    }
    pub(crate) fn commit(&self) {
        let mut pending = self.pending.lock();
        let mut current = self.current.lock();
        current.append(&mut pending);
        pending.clear();
    }
    pub(crate) fn rollback(&self) {
        let mut pending = self.pending.lock();
        *pending = Vec::new();
    }
}

pub(crate) fn assign_default_id(id: Uri) -> Box<InitFn> {
    // let id = Arc::new(id);
    Box::new(move |_, schema| {
        if schema.id().is_none() {
            schema.set_id(id.clone());
        }
        Ok(Some(Box::new(move |_, _| {
            Ok(Box::new(move |value, eval, next| next.call(value, eval)))
        })))
    })
}

pub(crate) fn assign_default_meta_schema(meta_schema: MetaSchema) -> Box<InitFn> {
    Box::new(move |_, schema| {
        if schema.id().is_none() {
            schema.set_meta_schema(meta_schema.clone());
        }
        Ok(Some(Box::new(move |i: Interrogator, s: Schema| {
            Ok(Box::new(move |value, eval, next| next.call(value, eval)))
        })))
    })
}

#[cfg(test)]
mod test {
    #[derive(Clone)]
    struct TestImpl {}
}
