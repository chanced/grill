// pub trait Injectable: Send + Sync + From<Self::Value> + 'static {
//     type Value: Clone + Send + Sync + 'static;
//     fn inject(&self) -> Self::Value;
// }
