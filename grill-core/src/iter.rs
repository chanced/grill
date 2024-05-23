//! Various [`Iterator`]s.

/// TODO: Implement this
pub struct Ancestors<'i, S, K> {
    _schema: S,
    _key: K,
    _marker: std::marker::PhantomData<&'i ()>,
}
