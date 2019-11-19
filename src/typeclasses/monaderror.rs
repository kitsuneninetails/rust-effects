use super::{F, Effect};
use crate::typeclasses::monad::Monad;

/// The `MonadError` typeclass.  This just ensures a `flat_map` operation is available for a context
/// of type `F<_>` which operates on a type `X` which can perform a new function returning
/// another context for the given type `X`.  This context is then "flattened" into the originating
/// context, essentially taking its place as the context holder for `X`.
pub trait MonadError<'a>: Effect + Monad<'a> {
    type E;
    fn raise_error(err: Self::E) -> Self::FX;
    fn handle_error(f: Self::FX, recovery: impl 'a + Send + Sync + Fn(Self::E) -> Self::FX) -> Self::FX;
    fn attempt(f: Self::FX) -> Result<Self::X, Self::E>;
}

pub trait MonadErrorEffect<'a, X>: Sized where Self: F<<Self as MonadErrorEffect<'a, X>>::X> {
    type X;
    type E;
    type Fct: MonadError<'a, X=Self::X, FX=Self, E=Self::E> + Effect;
}

pub fn raise_error<'a, I: MonadErrorEffect<'a, X>, X>(x: I::E) -> I {
    I::Fct::raise_error(x)
}

pub fn handle_error<'a, I: MonadErrorEffect<'a, X>, X>(f: I, recovery: impl 'a + Send + Sync + Fn(I::E) -> I) -> I {
    I::Fct::handle_error(f, recovery)
}

pub fn attempt<'a, I: MonadErrorEffect<'a, X>, X>(f: I) -> Result<I::X, I::E> {
    I::Fct::attempt(f)
}
