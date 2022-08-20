use std::marker::PhantomData;

pub struct Shape;
pub struct Id<'a>(PhantomData<&'a ()>);
