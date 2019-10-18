use super::{F, Effect};

/// The `Functor` type class.  This represents a mapping from one type to another, which takes
/// place inside the given context.  For a Functor for context `C` containing members of type `X`,
/// a function `fmap` must defined `X -> Y` where `Y` is the target type.  Applying the Functor
/// for context `C` will result in the context `C'` which contains the same number of members as
/// the original `C`, but containing the members `{ fmap(x1), fmap(x2), ..., fmap(xn) } ` for all
/// members` x1, x2, ..., xn` in `C`.
pub trait Functor<'a, X, Y>: Effect {
    type FX: F<X>;
    type FY: F<Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY;
}

pub trait FunctorEffect<'a, X, Y>
    where {
    type FX: F<X>;
    type FY: F<Y>;
    type Fct: Functor<'a, X, Y, FX=Self::FX, FY=Self::FY> + Effect;
}

pub fn fmap<'a, FX: F<X> + FunctorEffect<'a, X, Y, FX=FX, FY=FY>, FY: F<Y>, X, Y>(
    f: FX,
    func: impl 'a + Fn(X) -> Y + Send + Sync) -> FY {
    FX::Fct::fmap(f, func)
}

pub trait Functor2<'a, X, Y, Z>: Effect {
    type FX: F<X>;
    type FY: F<Y>;
    type FZ: F<Z>;
    fn fmap2(fa: Self::FX, fb: Self::FY, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Self::FZ;
}

pub trait Functor2Effect<'a, X, Y, Z> {
    type FX: F<X>;
    type FY: F<Y>;
    type FZ: F<Z>;
    type Fct: Functor2<'a, X, Y, Z, FX=Self::FX, FY=Self::FY, FZ=Self::FZ> + Effect;
}

pub fn fmap2<'a, FX: F<X>+ Functor2Effect<'a, X, Y, Z, FX=FX, FY=FY, FZ=FZ>, FY: F<Y>, FZ: F<Z>, X, Y, Z>(
    fa: FX,
    fb: FY,
    func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> FZ {
    FX::Fct::fmap2(fa, fb, func)
}
