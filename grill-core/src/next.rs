use crate::{Result, Value, Eval};

pub struct Next<C> {
    func: Option<C>,
}
impl<C> Next<C>
where
    C: FnOnce(Value) -> Result<Eval>,
{
    pub fn call(self, value: Value) -> Result<Eval> {
        self.func.map_or(Ok(Eval::new()), |f| f(value))
    }
}
