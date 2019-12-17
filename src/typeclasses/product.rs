//use super::{F, Effect};
//use crate::typeclasses::monad::Monad;
//use crate::typeclasses::functor::{Functor, Functor2};
//
///// `Productable` is an extended typeclass that just specifies the ability to combine two
///// type constructors and map the internal type to a new, combined type, represented by a 2-tuple
///// of the original values.  This is usually equivalent to a `fmap2` from `Functor2`, using a
///// tuple output as the combining function.
//pub trait Productable<'a>: Monad<'a, Fnct2Z=(<Self as Functor<'a>>::FnctX, <Self as Functor<'a>>::FnctY)>
//    where Self::FctForZ: F<<Self as Functor2<'a>>::Fnct2Z> {
//    fn product(fa: Self::FctForX, fb: Self::FctForY) -> Self::FctForZ {
//        Self::fmap2(fa, fb, |a, b| (a, b))
//    }
//}
//
//pub trait ProductableEffect<'a, X, Y> : F<X> + Sized {
//    type FY: F<Y>;
//    type FZ: F<(X, Y)>;
//    type Fct: Productable<'a, FnctX=X, FnctY=Y, FctForX=Self, FctForY=Self::FY, FctForZ=Self::FZ> + Effect;
//}
//
//pub fn product<'a, FX, FY, FZ, X, Y>(fa: FX, fb: FY) -> FZ
//    where FX: F<X> + ProductableEffect<'a, X, Y, FY=FY, FZ=FZ>,
//          FY: F<Y>,
//          FZ: F<(X, Y)> {
//    FX::Fct::product(fa, fb)
//}
