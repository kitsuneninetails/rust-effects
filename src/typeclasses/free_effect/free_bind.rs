use crate::typeclasses::{free_effect::FreeEffect, monad::Monad};

trait CloneableFn<T, U, In>: Fn(T) -> In::MonadOut + Send
where
    In: Monad<U> + Send,
    T: Send,
    U: Send,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<T, U, In>>
    where
        Self: 'a;
}

impl<F, T, U, In> CloneableFn<T, U, In> for F
where
    In: Monad<U> + Send,
    T: Send,
    U: Send,
    F: Fn(T) -> In::MonadOut + Clone + Send,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<T, U, In>>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}
impl<'a, T, U, In> Clone for Box<dyn 'a + CloneableFn<T, U, In>>
where
    In: 'a + Monad<U> + Send,
    T: 'a + Send,
    U: 'a + Send,
{
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Clone)]
pub struct FreeBind<T, U, In>
where
    In: 'static + Monad<U> + Send,
    T: 'static + Send,
    U: 'static + Send,
{
    func: Box<dyn CloneableFn<T, U, In>>,
}

impl<T, U, In> FreeBind<T, U, In>
where
    In: Monad<U> + Send,
    T: Send,
    U: Send,
{
    pub fn new(func: impl Fn(T) -> In::MonadOut + Send + Clone + 'static) -> FreeBind<T, U, In> {
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
    fn fold(&self, source: Self::In) -> Self::Out {
        Self::In::bind(source, self.func.clone())
    }
}
