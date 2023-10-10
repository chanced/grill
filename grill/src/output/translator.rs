use crate::anymap::AnyMap;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Translator {
    lang: String,
    map: AnyMap,
}

impl Translator {
    #[must_use]
    pub fn new(lang: String) -> Self {
        Self {
            lang,
            map: AnyMap::new(),
        }
    }

    #[must_use]
    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn insert<T>(&mut self, translate: T)
    where
        T: 'static + Clone + std::fmt::Debug + Send + Sync,
    {
        self.map.insert(translate);
    }
    #[must_use]
    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any + std::fmt::Debug + Clone + Send + Sync,
    {
        self.map.get()
    }
}
