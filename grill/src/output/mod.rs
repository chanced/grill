mod output;
pub use output::{Basic, BasicNode, Detailed, Flag, Output, Verbose};

mod error;
pub use error::Error;

mod structure;
pub use structure::Structure;

mod translator;
pub use translator::{Translations, Translator};

const ERROR_MSG: &str = "one or more validation errors occurred";
const SUCCESS_MSG: &str = "validation passed";
