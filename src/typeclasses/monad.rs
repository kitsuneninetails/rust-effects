//use super::{F, Effect};
//use crate::typeclasses::applicative::Applicative;
//
///// The `Monad` typeclass.  This just ensures a `flat_map` operation is available for a context
///// of type `F<_>` which operates on a type `X` which can perform a new function returning
///// another context for the given type `X`.  This context is then "flattened" into the originating
///// context, essentially taking its place as the context holder for `X`.
//pub trait Monad<'a>: Applicative<'a> {
//    fn flat_map(f: Self::FctForX, func: impl 'a + Fn(Self::FnctX) -> Self::FctForY + Send + Sync) -> Self::FctForY;
//}
//
//pub trait MonadEffect<'a, Y> : F<<Self as MonadEffect<'a, Y>>::X> + Sized {
//    type X;
//    type FY: F<Y>;
//    type Fct: Monad<'a, FnctX=Self::X, FnctY=Y, FctForX=Self, FctForY=Self::FY> + Effect;
//}
//
//pub fn flat_map<'a, FX, FY, Y>(f: FX, func: impl 'a + Fn(FX::X) -> FY + Send + Sync) -> FY
//    where FX: MonadEffect<'a, Y, FY=FY>,
//          FY: F<Y> {
//    FX::Fct::flat_map(f, func)
//}
