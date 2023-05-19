use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Compiler<'s> {
	marker: PhantomData<&'s str>
}
