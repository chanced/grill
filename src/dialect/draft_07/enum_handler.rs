/// The value of this keyword MUST be an array.  This array SHOULD have
/// at least one element.  Elements in the array SHOULD be unique.
///
/// An instance validates successfully against this keyword if its value
/// is equal to one of the elements in this keyword's array value.

/// Elements in the array might be of any value, including null.
///
/// - [JSON Schema Validation 07 # 6.1.2. `enum`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.1.2)
pub struct EnumHandler {}
