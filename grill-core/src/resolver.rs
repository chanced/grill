use serde_json::Value;

use self::error::Error;

pub trait Resolver {
    fn resolve(&self, id: String) -> Result<Value, Error>;
}

pub mod error {
    use std::error::Error as StdError;
    pub enum Error {
        /// The schema was not found
        NotFound(String),
        ///
        Internal(Box<dyn StdError + Send + Sync + 'static>),
    }
}
