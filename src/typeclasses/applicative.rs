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
