mod enum_handler;
mod type_handler;
pub mod handler {
    pub use super::{
        enum_handler::{EnumHandler, EnumInvalid},
        type_handler::{TypeHandler, TypeInvalid},
    };
}
