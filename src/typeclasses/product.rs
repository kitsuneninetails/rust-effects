use super::{F, Effect};
use crate::typeclasses::monad::Monad;

/// `Productable` is an extended typeclass that just specifies the ability to combine two
/// type constructors and map the internal type to a new, combined type, represented by a 2-tuple
/// of the original values.  This is usually equivalent to a `fmap2` from `Functor2`, using a
/// tuple output as the combining function.
pub trait Productable<'a>: Effect + Monad<'a> {
    type FXY: F<(Self::X, Self::Y)>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY;
}

pub trait ProductableEffect<'a, X, Y> {
    type FX: F<X>;
    type FY: F<Y>;
    type FXY: F<(X, Y)>;
    type Fct: Productable<'a, X=X, Y=Y, FX=Self::FX, FY=Self::FY, FXY=Self::FXY> + Effect;
}

pub fn product<'a, FX, FY, FXY, X, Y>(fa: FX, fb: FY) -> FXY
    where FX: F<X> + ProductableEffect<'a, X, Y, FX=FX, FY=FY, FXY=FXY>,
          FY: F<Y>,
          FXY: F<(X, Y)>{
    FX::Fct::product(fa, fb)
}
