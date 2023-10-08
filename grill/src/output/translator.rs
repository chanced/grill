use crate::anymap::AnyMap;
use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Translations {
    lang: String,
    map: AnyMap,
}

impl Translations {
    #[must_use]
    pub fn new(lang: String) -> Self {
        Self {
            lang,
            map: AnyMap::new(),
        }
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

#[derive(Debug, Clone)]
pub struct Translator {
    /// hashmap of language codes to an anymap of translations
    translations: HashMap<String, Translations>,
}

impl Translator {
    /// Returns the `Lang` associated with the language code, if it exists.
    #[must_use]
    pub fn get(&self, lang: &str) -> Option<&Translations> {
        self.translations.get(lang)
    }

    /// Returns the `Lang` associated with the language code, if it exists.
    pub fn get_mut(&mut self, lang: &str) -> Option<&mut Translations> {
        self.translations.get_mut(lang)
    }
}
