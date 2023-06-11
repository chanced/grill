use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;

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

pub fn json(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let mut json = serde_json::Deserializer::from_str(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(&mut json))
}

#[cfg(feature = "yaml")]
pub fn yaml(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let yaml = serde_yaml::Deserializer::from_str(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(yaml))
}

#[cfg(feature = "toml")]
pub fn toml(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let toml = toml::Deserializer::new(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(toml))
}
