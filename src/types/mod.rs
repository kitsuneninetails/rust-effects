pub mod cfuture;
pub mod option;
pub mod result;
pub mod vec;

use crate::typeclasses::{applicative::Applicative, functor::Functor, monad::Monad};

impl<U> Functor<U> for () {
    type FuncT = ();
    type FunctorOut = ();
    fn fmap(m: Self, _func: impl Fn(()) -> U + Send + 'static) -> Self::FunctorOut {
        m
    }
}
impl<U> Applicative<U> for () {
    type AppT = ();
    fn pure(_a: ()) -> Self {
        ()
    }
}

impl Monad for () {
    type MonadT = ();
    type MonadOut = ();
    fn bind(
        m: Self,
        _func: impl Fn(Self::AppT) -> Self::MonadOut + Send + 'static,
    ) -> Self::MonadOut {
        m
    }
}
