use super::{Effect};
use crate::typeclasses::functor::Functor;
use crate::typeclasses::F;

pub struct ConcreteMapper<'a, X, Y> {
    _func: Box<dyn 'a + Fn(X) -> Y + Send + Sync>
}
impl<'a, X, Y> ConcreteMapper<'a, X, Y> {
    pub fn new(f: impl 'a + Fn(X) -> Y + Send + Sync) -> Self {
        ConcreteMapper {
            _func: Box::new(f)
        }
    }

    pub fn call(&self, arg: X) -> Y {
        self._func(arg)
    }
}

/// An applicative is a `Functor` which defines a `pure` method to generate a concrete type
/// constructor from a concrete value.  In general, `pure` should be seen as a greedy
/// function, consuming its values and generating a type constructor upon execution (not
/// deferred like a lazy evaluator might be).
pub trait Applicative<'a>: Effect + Functor<'a> {
    type FMap: F<ConcreteMapper<'a, Self::X, Self::Y>>;
    fn ap(func: Self::FMap, x: Self::FX) -> Self::FY;
    fn pure(x: Self::X) -> Self::FX;
    fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
        Self::ap(Self::pure(ConcreteMapper::new(func)), f)
    }
}

pub trait ApplicativeEffect<'a, Y>: Sized
    where
        Self: F<<Self as ApplicativeEffect<'a, Y>>::X>,
        Self::FMap: F<ConcreteMapper<'a, Self::X, <Self::Fct as Functor<'a>>::Y>> {
    type X;
    type FMap: F<ConcreteMapper<'a, Self::X, Y>>;
    type Fct: Applicative<'a, X=Self::X, FX=Self, FMap=Self::FMap>;
}

pub fn pure<'a, I: ApplicativeEffect<'a, ()>>(x: I::X) -> I {
    I::Fct::pure(x)
}

pub fn ap<'a, I: ApplicativeEffect<'a, Y>, Y>(func: I::FMap, x: I::X) -> I {
    I::Fct::ap(func, x)
}
