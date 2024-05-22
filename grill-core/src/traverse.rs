pub struct Ancestors<'i, S, K> {
    schema: S,
    key: K,
    _marker: std::marker::PhantomData<&'i ()>,
}
