mod error;
mod schema;
pub use error::*;
pub use schema::*;

extern crate strum;
#[macro_use]
extern crate strum_macros;
// extern crate alloc;

pub mod value {
    pub use serde_json::Value;
}
pub mod map {
    pub use serde_json::Map;
}
pub mod number {
    pub use serde_json::Number;
}
pub use map::Map;
pub use number::Number;
pub use value::Value;
