pub mod cfuture;
pub mod option;
pub mod result;
pub mod vec;

use crate::typeclasses::{applicative::Applicative, functor::Functor, monad::Monad};

impl<T, U> Functor<T, U> for () {
    type FunctorOut = ();
    fn fmap(m: Self, _func: impl Fn(T) -> U + Send + 'static) -> Self::FunctorOut {
        m
    }
}
impl<T, U> Applicative<T, U> for () {
    fn pure(_a: T) -> Self {
        ()
    }
}

impl<T, U> Monad<T, U> for () {
    type MonadOut = ();
    fn bind(m: Self, _func: impl Fn(T) -> Self::MonadOut + Send + 'static) -> Self::MonadOut {
        m
    }
}
