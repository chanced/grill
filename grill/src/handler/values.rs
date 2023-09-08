use lazy_static::lazy_static;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};

lazy_static! {
    static ref TRUE: Value = Value::Bool(true);
    static ref FALSE: Value = Value::Bool(false);
    static ref NULL: Value = Value::Null;
}

#[derive(Debug, Clone, Copy)]
pub struct ValueKey {
    inner: InnerKey,
}

impl ValueKey {
    fn null() -> Self {
        Self {
            inner: InnerKey::Null,
        }
    }
    fn bool(value: bool) -> Self {
        Self {
            inner: InnerKey::Bool(if value { BoolKey::True } else { BoolKey::False }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum InnerKey {
    Object(ObjectKey),
    Array(ArrayKey),
    String(StringKey),
    Bool(BoolKey),
    Number(NumberKey),
    Null,
}
macro_rules! impl_key {
    ($($t:ident),+) => {
		$(
			paste::paste! {
				impl From<[<$t Key>]> for ValueKey {
					fn from(key: [<$t Key>]) -> Self {
						Self {
							inner: InnerKey::$t(key),
						}
					}
				}
			}
		)+
	};
}

impl_key!(Object, Array, String, Bool, Number);

new_key_type! {
    struct StringKey;
    struct ObjectKey;
    struct ArrayKey;
    struct NumberKey;
}
#[derive(Debug, Clone, Copy)]
enum BoolKey {
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Values {
    strings: SlotMap<StringKey, Value>,
    objects: SlotMap<ObjectKey, Value>,
    arrays: SlotMap<ArrayKey, Value>,
    numbers: SlotMap<NumberKey, Value>,
}

impl Values {
    #[must_use]
    pub fn get(&self, key: ValueKey) -> Option<&Value> {
        match key.inner {
            InnerKey::Object(key) => self.objects.get(key),
            InnerKey::Array(key) => self.arrays.get(key),
            InnerKey::String(key) => self.strings.get(key),
            InnerKey::Number(key) => self.numbers.get(key),
            InnerKey::Bool(key) => Some(get_bool(key)),
            InnerKey::Null => Some(&NULL),
        }
    }
    #[must_use]
    pub fn insert(&mut self, value: &Value) -> ValueKey {
        match value {
            Value::Number(_) => self.insert_number(value).into(),
            Value::String(_) => self.insert_string(value).into(),
            Value::Array(_) => self.insert_array(value).into(),
            Value::Object(_) => self.insert_object(value).into(),
            Value::Bool(value) => ValueKey::bool(*value),
            Value::Null => ValueKey::null(),
        }
    }
    fn insert_object(&mut self, value: &Value) -> ObjectKey {
        let object = value.as_object().unwrap();
        for (key, value) in &self.objects {
            let obj = value.as_object().unwrap();
            if obj == object {
                return key;
            }
        }
        self.objects.insert(value.clone())
    }
    fn insert_array(&mut self, value: &Value) -> ArrayKey {
        let array = value.as_array().unwrap();
        for (key, value) in &self.arrays {
            let arr = value.as_array().unwrap();
            if arr == array {
                return key;
            }
        }
        self.arrays.insert(value.clone())
    }
    fn insert_string(&mut self, value: &Value) -> StringKey {
        let string = value.as_str().unwrap();
        for (key, value) in &self.strings {
            let s = value.as_str().unwrap();
            if s == string {
                return key;
            }
        }
        self.strings.insert(value.clone())
    }
    fn insert_number(&mut self, value: &Value) -> NumberKey {
        let number = value.as_f64().unwrap();
        for (key, value) in &self.numbers {
            let n = value.as_f64().unwrap();
            #[allow(clippy::float_cmp)]
            if n == number {
                return key;
            }
        }
        self.numbers.insert(value.clone())
    }
}

fn get_bool(key: BoolKey) -> &'static Value {
    match key {
        BoolKey::True => &TRUE,
        BoolKey::False => &FALSE,
    }
}
