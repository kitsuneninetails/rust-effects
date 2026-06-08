use crate::typeclasses::{free_effect::FreeEffect, monad::Monad};

pub struct FreeBind<T, U, In>
where
    In: Monad<U> + Send,
    T: Send,
    U: Send,
{
    func: Box<dyn Fn(T) -> In::MonadOut + Send>,
}

impl<T, U, In> FreeBind<T, U, In>
where
    In: Monad<U> + Send,
    T: Send,
    U: Send,
{
    pub fn new(func: impl Fn(T) -> In::MonadOut + Send + 'static) -> FreeBind<T, U, In> {
        FreeBind {
            func: Box::new(func),
        }
    }
}

impl<T, U, In> FreeEffect for FreeBind<T, U, In>
where
    In: Monad<U, MonadT = T> + Send + 'static,
    T: Send + 'static,
    U: Send + 'static,
{
    type InU = U;
    type OutU = U;
    type In = In;
    type Out = In::MonadOut;
    fn fold(self, source: Self::In) -> Self::Out {
        Self::In::bind(source, self.func)
    }
}
