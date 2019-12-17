use super::{F, Effect};
use crate::Empty;

/// The `Functor` type class.  This represents a mapping from one type to another, which takes
/// place inside the given context.  For a Functor for context `C` containing members of type `X`,
/// a function `fmap` must defined `X -> Y` where `Y` is the target type.  Applying the Functor
/// for context `C` will result in the context `C'` which contains the same number of members as
/// the original `C`, but containing the members `{ fmap(x1), fmap(x2), ..., fmap(xn) } ` for all
/// members` x1, x2, ..., xn` in `C`.
pub trait Functor<'a>: Effect {
    type FnctX;
    type FnctY;
    type FnctZ;
    type FctForX: F<Self::FnctX>;
    type FctForY: F<Self::FnctY>;
    type FctForZ: F<Self::FnctZ>;
    fn fmap(f: Self::FctForX, func: impl 'a + Fn(Self::FnctX) -> Self::FnctY + Send + Sync) -> Self::FctForY;
    fn fmap2(fa: Self::FctForX,
             fb: Self::FctForY,
             func: impl 'a + Fn(Self::FnctX, Self::FnctY) -> Self::FnctZ + Send + Sync) -> Self::FctForZ;
}

pub trait FunctorEffect<'a, Y, Z = ()>: F<<Self as FunctorEffect<'a, Y, Z>>::FnctX> + Sized {
    type FnctX;
    type FnctY = Y;
    type FnctZ = Z;
    type FctForY: F<Self::FnctY>;
    type FctForZ: F<Self::FnctZ> = Empty;
    type FunctorFct: Functor<'a, FnctX=Self::FnctX, FnctY=Self::FnctY, FnctZ=Self::FnctZ, FctForX=Self, FctForY=Self::FctForY, FctForZ=Self::FctForZ> + Effect;
}

pub fn fmap<'a, FctForX, FctForY, X, Y>(
    f: FctForX,
    func: impl 'a + Fn(FctForX::FnctX) -> FctForX::FnctY + Send + Sync) -> FctForY
    where
        FctForX: F<X> + FunctorEffect<'a, Y, FctForY=FctForY>,
        FctForY: F<FctForX::FnctY> {
    FctForX::FunctorFct::fmap(f, func)
}

pub fn fmap2<'a, FctForX, FctForY, FctForZ, X, Y, Z>(
    fa: FctForX,
    fb: FctForY,
    func: impl 'a + Fn(FctForX::FnctX, FctForX::FnctY) -> FctForX::FnctZ + Send + Sync) -> FctForZ
    where
        FctForX: F<X> + FunctorEffect<'a, Y, Z, FctForY=FctForY, FctForZ=FctForZ>,
        FctForY: F<FctForX::FnctY>,
        FctForZ: F<FctForX::FnctZ> {
    FctForX::FunctorFct::fmap2(fa, fb, func)
}
