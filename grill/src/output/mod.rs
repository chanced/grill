mod output;
pub use output::Output;

mod structure;
pub use structure::Structure;

mod detail;
pub use detail::Detail;

const ERROR_MSG: &str = "one or more validation errors occurred";
const SUCCESS_MSG: &str = "validation passed";
