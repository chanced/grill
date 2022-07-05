use crate::{Error, Evaluation, Interrogator, Next, Schema};
use serde_json::Value;

/// Annotates an [`Evaluation`] with relevant information pertinent to the
/// [`Schema`] for the given [`OutputFmt`].
pub type ExecutorFn =
    dyn 'static + Send + Sync + Fn(&Value, Evaluation, Next) -> Result<Evaluation, Error>;

/// Returned from [`Applicator::init`](Applicator::init) to setup the `Applicator` for use.
pub type SetupFn =
    dyn 'static + Send + Sync + Fn(Interrogator, Schema) -> Result<Option<Box<ExecutorFn>>, Error>;

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
pub trait Applicator {
    /// Initializes the `Applicator` with the [`Interrogator`] for the given [`Schema`].
    fn init(
        &self,
        interrogator: Interrogator,
        schema: Schema,
    ) -> Result<Option<Box<SetupFn>>, Error>;
}

impl<F> Applicator for F
where
    F: 'static + Send + Sync + Fn(Interrogator, Schema) -> Result<Option<Box<SetupFn>>, Error>,
{
    fn init(
        &self,
        interrogator: Interrogator,
        schema: Schema,
    ) -> Result<Option<Box<SetupFn>>, Error> {
        self(interrogator, schema)
    }
}
#[cfg(test)]
mod test {
    #[derive(Clone)]
    struct TestImpl {}
}
