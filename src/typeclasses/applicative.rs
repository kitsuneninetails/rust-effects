use super::{Effect};
use crate::typeclasses::functor::Functor2;
use crate::typeclasses::F;

/// An applicative is a `Functor` which defines a `pure` method to generate a concrete type
/// constructor from a concrete value.  In general, `pure` should be seen as a greedy
/// function, consuming its values and generating a type constructor upon execution (not
/// deferred like a lazy evaluator might be).
pub trait Applicative<'a>: Functor2<'a> {
    fn pure(x: Self::X) -> Self::FX;
}

pub trait ApplicativeApply<'a, M>: Effect + Applicative<'a>
    where
        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
    type FMapper: F<M>;
    fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY;
}

