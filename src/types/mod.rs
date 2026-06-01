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

impl Monad for () {
    type T = ();
    type MonadOut = ();
    fn bind(m: Self, _func: impl Fn(Self::T) -> Self::MonadOut + Send + 'static) -> Self::MonadOut {
        m
    }
}
