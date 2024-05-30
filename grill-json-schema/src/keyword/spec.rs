use enum_dispatch::enum_dispatch;
use grill_core::Key;

pub mod schema;

#[enum_dispatch(Keyword<S, K>)]
pub enum Spec<S, K: Key> {
    Schema(schema::Schema<S, K>),
}
