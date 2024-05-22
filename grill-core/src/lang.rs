use slotmap::Key;
use std::fmt::Debug;

use crate::schema::{Schema, Schemas};

pub struct Compile<'i, S, K: Key> {
    pub schemas: &'i mut Schemas<S, K>,
}

pub trait Language<K>: Sized + Clone + Debug
where
    K: 'static + Key,
{
    type Schema<'i>: Schema<'i, K>;
}
