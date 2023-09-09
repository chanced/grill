use std::collections::{BTreeMap, HashMap};

use num::{BigInt, BigRational};
use serde_json::Number;
use slotmap::{new_key_type, Key, SlotMap};

use crate::error::NumberError;

use crate::big::{parse_int, parse_rational};

pub type BigRationals = Numbers<RationalKey, BigRational>;
pub type BigInts = Numbers<IntKey, BigInt>;

new_key_type! {
    pub struct RationalKey;
}
new_key_type! {
    pub struct IntKey;
}

pub trait Parse: Sized {
    fn parse(value: &str) -> Result<Self, NumberError>;
}
impl Parse for BigInt {
    fn parse(value: &str) -> Result<Self, NumberError> {
        parse_int(value)
    }
}
impl Parse for BigRational {
    fn parse(value: &str) -> Result<Self, NumberError> {
        parse_rational(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Numbers<K: Key, V: Parse> {
    table: SlotMap<K, V>,
    lookup: HashMap<Number, K>,
}

impl<K, V> Numbers<K, V>
where
    K: Key,
    V: Parse + PartialEq<V>,
{
    pub fn insert(&mut self, num: &Number) -> Result<K, NumberError>
    where
        V: PartialEq,
    {
        if let Some(key) = self.lookup.get(num) {
            return Ok(*key);
        }

        // TODO use `as_str` once github.com/serde-rs/json/pull/1067 is merged
        let num_str = num.to_string();
        let value = V::parse(&num_str)?;
        for (key, val) in self.table.iter() {
            if val == &value {
                return Ok(key);
            }
        }
        let key = self.table.insert(value);
        self.lookup.insert(num.clone(), key);
        Ok(key)
    }
    #[must_use]
    pub fn get_by_number(&self, num: &Number) -> Option<&V> {
        self.lookup.get(num).and_then(|key| self.table.get(*key))
    }

    #[must_use]
    pub fn contains(&self, num: &Number) -> bool {
        self.lookup.contains_key(num)
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.table.get(key)
    }
}
