//use super::{Effect};
//use crate::typeclasses::functor::*;
//use crate::typeclasses::F;
//
///// An applicative is a `Functor` which defines a `pure` method to generate a concrete type
///// constructor from a concrete value.  In general, `pure` should be seen as a greedy
///// function, consuming its values and generating a type constructor upon execution (not
///// deferred like a lazy evaluator might be).
//pub trait Applicative<'a>: Functor<'a> {
//    fn pure(x: Self::FnctX) -> Self::FctForX;
//}
//
//pub trait ApplicativeEffect<'a, Y>: FunctorEffect<
//    'a,
//    Y,
//    (<<Self as ApplicativeEffect<'a, Y>>::Fct as Functor<'a>>::FnctX, Y)> + Sized {
//    type Fct: Applicative<'a, FnctX=Self::FnctX, FctForX=Self>;
//}
//
//pub fn pure<'a, Y, I: ApplicativeEffect<'a, Y>>(x: I::X) -> I {
//    I::Fct::pure(x)
//}
//
//pub trait ApplicativeApply<'a, M>: Effect + Applicative<'a>
//    where
//        M: 'a + Fn(Self::FnctX) -> Self::FnctY + Send + Sync {
//    type FMapper: F<M>;
//    fn apply(func: Self::FMapper, x: Self::FctForX) -> Self::FctForY;
//}
//
//pub trait ApplicativeApplyEffect<'a, M, X, Y> : F<X> + Sized
//    where
//        M: 'a + Fn(X) -> Y + Send + Sync {
//    type FM: F<M>;
//    type FY: F<Y>;
//    type Fct: ApplicativeApply<'a, M, FnctX=X, FnctY=Y, FMapper=Self::FM, FctForX=Self, FctForY=Self::FY> + Effect;
//}
//
//pub fn apply<'a, FM, M, FX, FY, X, Y>(
//    func: FM,
//    f: FX) -> FY
//    where
//        FX: ApplicativeApplyEffect<'a, M, X, Y, FM=FM, FY=FY>,
//        FY: F<Y>,
//        FM: F<M>,
//        M: 'a + Fn(X) -> Y + Send + Sync {
//    FX::Fct::apply(func, f)
//}
//
