//! Trait and common implementations for deserializing source data into schemas.

use std::{collections::HashMap, ops::Deref};

use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;

use crate::error::DeserializeError;

pub trait Deserializer: DynClone + Send + Sync + 'static {
    fn deserialize(&self, data: &str) -> Result<Value, erased_serde::Error>;
}
clone_trait_object!(Deserializer);

impl<F> Deserializer for F
where
    F: Fn(&str) -> Result<Value, erased_serde::Error> + Clone + Send + Sync + 'static,
{
    fn deserialize(&self, data: &str) -> Result<Value, erased_serde::Error> {
        self(data)
    }
}

#[derive(Clone)]
pub struct Deserializers {
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
}
impl std::fmt::Debug for Deserializers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.deserializers.iter().map(|(name, _)| name))
            .finish()
    }
}

impl Deserializers {
    pub fn new(mut deserializers: Vec<(&'static str, Box<dyn Deserializer>)>) -> Self {
        if deserializers.is_empty() {
            deserializers.push(("json", Box::new(deserialize_json)));
        }
        Self { deserializers }
    }
    pub fn deserialize(&self, data: &str) -> Result<Value, DeserializeError> {
        let mut errs = HashMap::new();
        for (name, deserializer) in &self.deserializers {
            match deserializer.deserialize(data) {
                Ok(value) => return Ok(value),
                Err(err) => {
                    errs.insert(*name, err);
                }
            }
        }
        Err(DeserializeError { formats: errs })
    }
}
impl Deref for Deserializers {
    type Target = [(&'static str, Box<dyn Deserializer>)];

    fn deref(&self) -> &Self::Target {
        &self.deserializers
    }
}

pub fn deserialize_json(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let mut json = serde_json::Deserializer::from_str(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(&mut json))
}

#[cfg(feature = "yaml")]
pub fn deserialize_yaml(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let yaml = serde_yaml::Deserializer::from_str(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(yaml))
}

#[cfg(feature = "toml")]
pub fn deserialize_toml(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let toml = toml::Deserializer::new(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(toml))
}
