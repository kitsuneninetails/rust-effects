//use super::{F, Effect};
//use crate::typeclasses::monad::*;
//
///// The `MonadError` typeclass.  This just ensures a `flat_map` operation is available for a context
///// of type `F<_>` which operates on a type `X` which can perform a new function returning
///// another context for the given type `X`.  This context is then "flattened" into the originating
///// context, essentially taking its place as the context holder for `X`.
//pub trait MonadError<'a>: Monad<'a> {
//    type E;
//    fn raise_error(err: Self::E) -> Self::FctForX;
//    fn handle_error(f: Self::FctForX, recovery: impl 'a + Send + Sync + Fn(Self::E) -> Self::FctForX) -> Self::FctForX;
//    fn attempt(f: Self::FctForX) -> Result<Self::FnctX, Self::E>;
//}
//
//pub trait MonadErrorEffect<'a>: F<<Self as MonadErrorEffect<'a>>::X> + Sized {
//    type X;
//    type E;
//    type Fct: MonadError<'a, FnctX=Self::X, FctForX=Self, E=Self::E> + Effect;
//}
//
//pub fn raise_error<'a, I: MonadErrorEffect<'a>>(x: I::E) -> I {
//    I::Fct::raise_error(x)
//}
//
//pub fn handle_error<'a, I: MonadErrorEffect<'a>>(f: I, recovery: impl 'a + Send + Sync + Fn(I::E) -> I) -> I {
//    I::Fct::handle_error(f, recovery)
//}
//
//pub fn attempt<'a, I: MonadErrorEffect<'a>>(f: I) -> Result<I::X, I::E> {
//    I::Fct::attempt(f)
//}
