use std::{hash::Hash, str::FromStr};

use serde::{Deserialize, Serialize};

pub trait Impl {
    type Output: for<'de> Deserialize<'de> + Serialize;
    type Annotation: for<'de> Deserialize<'de> + Serialize;
    type PartialId: Hash + for<'de> Deserialize<'de> + Serialize + FromStr + Clone;
    type Id: Hash + for<'de> Deserialize<'de> + Serialize + FromStr + Clone;
    type Scope;
    type Compile;
}
