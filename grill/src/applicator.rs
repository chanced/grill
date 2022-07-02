use crate::{Error, Evaluation, Interrogator, Next, Schema};
use serde_json::Value;
pub type ExecutorFn =
    dyn 'static + Send + Sync + Fn(&Value, Evaluation, Next) -> Result<Evaluation, Error>;

pub type SetupFn =
    dyn 'static + Send + Sync + Fn(Interrogator, Schema) -> Result<Option<Box<ExecutorFn>>, Error>;

/// Annotate an [`Evaluation`] with relevant data and validation state for a
/// given [`Value`] depending on the [`Schema`] and the specified
/// [`OutputFmt`](crate::OutputFmt).
///
/// `Applicator` is a three-stage process:
///
/// ## Initialization
/// When a [`Schema`](crate::schema::Schema) or
/// [`SubSchema`](crate::schema::SubSchema) is created or updated,
/// [`init`](crate::Applicator::init) is called for each
/// `Applicator` assigned to the [`Interrogator`].
///
/// The [`init`](crate::Applicator::init) method determines whether the
/// `Applicator` is applicable to the [`Schema`].
///
/// If relevant, [`init`](crate::Applicator::init) should update the [`Schema`]
/// with pertinent information for the `Applicator`, including creating and
/// associating [`SubSchema`](crate::schema::SubSchema) and inidicating
/// references. Finally, it should return a [`SetupFn`](SetupFn).
///
/// **Note**: The graph of [`Schema`] contained within the [`Interrogator`] will
/// not be fully populated during the initialization stage.
///
/// ## Setup
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
/// ## Execution
/// The [`ExecutorFn`] returned by [`SetupFn`](crate::applicator::SetupFn) is
/// invoked for each call to [`Schema::evaluate`](crate::Schema::evaluate). This
/// function should evaluate the [`Value`] annotate the [`Evaluation`] based
/// upon the specified [`OutputFmt`](crate::OutputFmt).
///
///
///
///

pub trait Applicator {
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
