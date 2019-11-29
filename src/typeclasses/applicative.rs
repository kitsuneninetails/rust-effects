use super::{Effect};
use crate::typeclasses::functor::Functor;
use crate::typeclasses::F;

/// An applicative is a `Functor` which defines a `pure` method to generate a concrete type
/// constructor from a concrete value.  In general, `pure` should be seen as a greedy
/// function, consuming its values and generating a type constructor upon execution (not
/// deferred like a lazy evaluator might be).
pub trait Applicative<'a>: Effect + Functor<'a> {
    fn pure(x: Self::X) -> Self::FX;
}

pub trait ApplicativeEffect<'a>: Sized where Self: F<<Self as ApplicativeEffect<'a>>::X> {
    type X;
    type Fct: Applicative<'a, X=Self::X, FX=Self>;
}

pub fn pure<'a, I: ApplicativeEffect<'a>>(x: I::X) -> I {
    I::Fct::pure(x)
}

pub trait ApplicativeApply<'a, M>: Effect + Applicative<'a>
    where
        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
    type FMapper: F<M>;
    fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY;
}

pub trait ApplicativeApplyEffect<'a, M, X, Y>
    where
        M: 'a + Fn(X) -> Y + Send + Sync {
    type FM: F<M>;
    type FX: F<X>;
    type FY: F<Y>;
    type Fct: ApplicativeApply<'a, M, X=X, Y=Y, FMapper=Self::FM, FX=Self::FX, FY=Self::FY> + Effect;
}

pub fn apply<'a, FM, M, FX, FY, X, Y>(
    func: FM,
    f: FX) -> FY
    where
        FX: F<X> + ApplicativeApplyEffect<'a, M, X, Y, FM=FM, FX=FX, FY=FY>,
        FY: F<Y>,
        FM: F<M>,
        M: 'a + Fn(X) -> Y + Send + Sync {
    FX::Fct::apply(func, f)
}

