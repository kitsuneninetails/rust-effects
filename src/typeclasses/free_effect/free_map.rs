use std::marker::PhantomData;

use crate::typeclasses::{free_effect::FreeEffect, monad::Monad};

trait CloneableFn<T, U>: Fn(T) -> U + Send
where
    T: Send,
    U: Send,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<T, U>>
    where
        Self: 'a;
}

impl<F, T, U> CloneableFn<T, U> for F
where
    T: Send,
    U: Send,
    F: Fn(T) -> U + Clone + Send,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<T, U>>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}
impl<'a, T, U> Clone for Box<dyn 'a + CloneableFn<T, U>>
where
    T: 'a + Send,
    U: 'a + Send,
{
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
pub struct FreeMap<T, U, In>
where
    T: Send,
    U: Send,
    In: Monad<U> + Send,
{
    func: Box<dyn CloneableFn<T, U>>,
    _ph: PhantomData<In>,
}

impl<T, U, In> FreeMap<T, U, In>
where
    T: Send,
    U: Send,
    In: Monad<U> + Send,
{
    pub fn new(func: impl Fn(T) -> U + Send + Clone + 'static) -> Self {
        FreeMap {
            func: Box::new(func),
            _ph: PhantomData,
        }
    }
}

impl<T, U, In> FreeEffect for FreeMap<T, U, In>
where
    T: Send + 'static,
    U: Send + 'static,
    In: Monad<U, MonadT = T> + Send,
{
    type InU = U;
    type OutU = U;
    type In = In;
    type Out = In::MonadOut;
    fn fold(&self, source: Self::In) -> Self::Out {
        Self::In::fmap(source, self.func.clone())
    }
}
