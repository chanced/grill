use std::{
    collections::HashMap,
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use num_rational::BigRational;
use once_cell::sync::Lazy;
use serde_json::{Number, Value};

use crate::big::{self, parse_rational};

fn boolean(value: bool) -> Arc<Value> {
    static TRUE: Lazy<Arc<Value>> = Lazy::new(|| Arc::new(Value::Bool(true)));
    static FALSE: Lazy<Arc<Value>> = Lazy::new(|| Arc::new(Value::Bool(false)));
    if value {
        TRUE.clone()
    } else {
        FALSE.clone()
    }
}

fn null() -> Arc<Value> {
    static NULL: Lazy<Arc<Value>> = Lazy::new(|| Arc::new(Value::Null));
    NULL.clone()
}

type Map<K, V> = HashMap<K, V, LenHasher>;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Values                                 ║
║                                ¯¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A cache store of [`Value`]s.
#[derive(Clone, Debug, Default)]
pub struct Values {
    strings: Vec<Arc<Value>>,
    numbers: Vec<Arc<Value>>,
    objects: Map<usize, Vec<Arc<Value>>>,
    arrays: Map<usize, Vec<Arc<Value>>>,
}

impl Values {
    /// Returns an `Arc<Value>` representation of `value`, either by returning
    /// an existing cached instance or inserts and returns a new instance.
    #[must_use]
    pub fn get_or_insert(&mut self, value: &Value) -> Arc<Value> {
        match value {
            Value::Number(_) => self.resolve_number(value),
            Value::String(_) => self.resolve_string(value),
            Value::Array(_) => self.resolve_array(value),
            Value::Object(_) => self.resolve_object(value),
            Value::Bool(value) => boolean(*value),
            Value::Null => null(),
        }
    }

    fn resolve_object(&mut self, value: &Value) -> Arc<Value> {
        let object = value.as_object().unwrap();
        let len = object.len();
        let objects = self.objects.entry(len).or_default();
        if let Some(object) = objects.iter().find(|o| o.as_object().unwrap() == object) {
            return object.clone();
        }
        let value = Arc::new(value.clone());
        objects.push(value.clone());
        value
    }

    fn resolve_array(&mut self, value: &Value) -> Arc<Value> {
        let array = value.as_array().unwrap();
        let len = array.len();
        let arrays = self.arrays.entry(len).or_default();

        if let Some(object) = arrays.iter().find(|o| o.as_array().unwrap() == array) {
            return object.clone();
        }
        let value = Arc::new(value.clone());
        arrays.push(value.clone());
        value
    }

    fn resolve_string(&mut self, value: &Value) -> Arc<Value> {
        let string = value.as_str().unwrap();
        #[allow(clippy::map_unwrap_or)]
        self.strings
            .binary_search_by_key(&string, |v| v.as_str().unwrap())
            .map(|index| self.strings[index].clone())
            .unwrap_or_else(|index| {
                self.strings.insert(index, Arc::new(value.clone()));
                self.strings[index].clone()
            })
    }

    fn resolve_number(&mut self, value: &Value) -> Arc<Value> {
        let number = value.as_number().unwrap();
        let number = number.as_str();
        #[allow(clippy::map_unwrap_or)]
        self.numbers
            .binary_search_by_key(&number, |v| {
                let number = v.as_number().unwrap();
                number.as_str()
            })
            .map(|index| self.numbers[index].clone())
            .unwrap_or_else(|index| {
                self.numbers.insert(index, Arc::new(value.clone()));
                self.numbers[index].clone()
            })
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Numbers                                ║
║                                ¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A cache of numbers parsed as `BigRational`.
#[derive(Debug, Default, Clone)]
pub struct Numbers {
    rationals: HashMap<String, Arc<BigRational>>,
}

impl Numbers {
    /// Creates a new [`Numbers`] cache, seeded with `seed`
    ///
    /// # Errors
    /// Returns [`big::ParseError`](crate::big::ParseError) if any of the numbers fail to parse
    pub fn new<'n>(seed: impl IntoIterator<Item = &'n Number>) -> Result<Self, big::ParseError> {
        let mut numbers = Self::default();
        for number in seed {
            numbers.get_or_insert_arc(number)?;
        }
        Ok(numbers)
    }

    /// Either returns an [`Arc`] to a previously parsed [`BigRational`]
    /// or parses and
    /// returns a reference to the [`BigRational`].
    ///
    /// # Errors
    /// Returns [`big::ParseError`] if the number fails to parse
    pub fn get_or_insert_arc(
        &mut self,
        number: &Number,
    ) -> Result<Arc<BigRational>, big::ParseError> {
        use std::collections::hash_map::Entry::{Occupied, Vacant};
        match self.rationals.entry(number.to_string()) {
            Occupied(entry) => Ok(entry.get().clone()),
            Vacant(entry) => Ok(entry
                .insert(Arc::new(parse_rational(number.as_str())?))
                .clone()),
        }
    }
    /// Either returns a reference to a previously parsed [`BigRational`] or parses and
    /// returns a reference to the [`BigRational`].
    ///
    /// # Errors
    /// Returns [`big::ParseError`] if the number fails to parse
    pub fn get_or_insert_ref(&mut self, number: &Number) -> Result<&BigRational, big::ParseError> {
        if self.rationals.contains_key(number.as_str()) {
            return Ok(self.rationals.get(number.as_str()).unwrap().as_ref());
        }
        let n = parse_rational(number.as_str())?;
        self.rationals.insert(number.to_string(), Arc::new(n));
        Ok(self.rationals.get(number.as_str()).unwrap().as_ref())
    }

    /// Returns an [`Arc`] to the [`BigRational`] associated with `value` if it
    /// exists.
    #[must_use]
    pub fn get_arc(&self, number: &Number) -> Option<Arc<BigRational>> {
        self.rationals.get(number.as_str()).cloned()
    }

    /// Returns a reference to the [`BigRational`] associated with `value` if it exists.
    #[must_use]
    pub fn get_ref(&self, number: &Number) -> Option<&BigRational> {
        self.rationals.get(number.as_str()).map(AsRef::as_ref)
    }

    /// Creates an empty [`Numbers`] with at least the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Numbers {
        Self {
            rationals: HashMap::with_capacity(capacity),
        }
    }

    // /// Either returns a previously parsed [`Arc<BigInt>`](`num::BigInt`) or
    // /// parses, caches, and returns a new [`Arc<BigInt>`](`num::BigInt`).
    // ///
    // /// # Example
    // /// ```
    // /// # use grill_core::big::BigInt;
    // /// use serde_json::Number;
    // /// # use grill_core::keyword::NumberCache;
    // ///
    // /// let mut cache = NumberCache::default();
    // /// let value = Number::from(34);
    // /// let int = cache.int(&value).unwrap();
    // /// assert_eq!(&*int, &BigInt::from(34));
    // /// ```
    // /// # Errors
    // /// Returns `big::ParseError` if the number fails to parse
    // pub fn int(&mut self, value: &Number) -> Result<Arc<BigInt>, big::ParseError> {
    //     use std::collections::hash_map::Entry::{Occupied, Vacant};

    //     match self.ints.entry(value.to_string()) {
    //         Occupied(entry) => Ok(entry.get().clone()),
    //         Vacant(entry) => Ok(entry.insert(Arc::new(parse_int(value.as_str())?)).clone()),
    //     }
    // }
    // /// Either returns a previously parsed [`Arc<BigRational>`](`num::BigRational`) or
    // /// parses, caches, and returns a new [`Arc<BigRational>`](`num::BigRational`).
    // ///
    // /// # Example
    // /// ```
    // /// # use grill_core::big::{parse_rational, BigRational, num::FromPrimitive};
    // /// use serde_json::Number;
    // /// # use grill_core::keyword::NumberCache;
    // /// use std::str::FromStr;
    // ///
    // /// let mut cache = NumberCache::default();
    // /// let value = Number::from_str("34.3434").unwrap();
    // /// let rat = cache.rational(&value).unwrap();
    // /// assert_eq!(&*rat, &parse_rational("34.3434").unwrap());
    // /// ```
    // /// # Errors
    // /// Returns `big::ParseError` if the number fails to parse
    // pub fn rational(&mut self, value: &Number) -> Result<Arc<BigRational>, big::ParseError> {
    //     use std::collections::hash_map::Entry::{Occupied, Vacant};

    //     match self.rationals.entry(value.to_string()) {
    //         Occupied(entry) => Ok(entry.get().clone()),
    //         Vacant(entry) => Ok(entry
    //             .insert(Arc::new(parse_rational(value.as_str())?))
    //             .clone()),
    //     }
    // }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Hasher                                 ║
║                                ¯¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Clone, Default)]
struct LenHasher(u64);
impl Hasher for LenHasher {
    fn write(&mut self, _bytes: &[u8]) {
        unreachable!();
    }

    fn write_usize(&mut self, i: usize) {
        self.0 = i as u64;
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}
impl BuildHasher for LenHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher {
        Self(self.0)
    }
}
