use std::marker::PhantomData;

use crate::typeclasses::{free_effect::FreeEffect, monad::Monad};

pub struct FreeMap<T, U, In>
where
    T: Send,
    U: Send,
    In: Monad<U> + Send,
{
    func: Box<dyn Fn(T) -> U + Send>,
    _ph: PhantomData<In>,
}

impl<T, U, In> FreeMap<T, U, In>
where
    T: Send,
    U: Send,
    In: Monad<U> + Send,
{
    pub fn new(func: impl Fn(T) -> U + Send + 'static) -> Self {
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
    fn fold(self, source: Self::In) -> Self::Out {
        Self::In::fmap(source, self.func)
    }
}
