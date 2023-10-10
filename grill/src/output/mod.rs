mod output;
pub use output::{AnnotationOrError, Basic, BasicNode, Detailed, Flag, Output, Verbose};

mod annotation;
pub use annotation::Annotation;

mod error;
pub use error::{BoxedError, Error};

mod structure;
pub use structure::Structure;

mod translator;
pub use translator::Translator;

const ERROR_MSG: &str = "one or more validation errors occurred";
const SUCCESS_MSG: &str = "validation passed";
