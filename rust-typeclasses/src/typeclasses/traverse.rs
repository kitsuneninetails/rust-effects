use super::{F,
            Effect,
            applicative::*,
            functor::*};

// Basically take a Traversable typeclass T, wrapping a set of concrete types X and a function
// which maps the X into a compatible type constructor E (usually an effect, like Future,
// IO, Result, etc.) wrapping a type Y.
//   _____T_____
// /            \
// (* -> *) -> *
//     E       X
//
// Traverse returns a flipped structure, with the returned effect E holding the Traversable T,
// which itself wraps teh returned concrete type Y.
//
// e.g. T<X> => traverse (gets a T<E<Y>> as an interim value) => E<T<Y>>
pub trait Traverse<'a, T, E, TR, FR, X, Y>: Effect
    where T: F<X>, // The Traversable type (Vec, Option, etc.), wrapping the effect, T<E<X>>
          E: F<Y> + Functor2Effect<'a, Y, TR, TR, FX=E, FY=FR, FZ=FR>, // The effect returned from func, wrapping a concrete type Y, E<Y>
          TR: F<Y>, // The Traversable type to return, wrapping the effect's internal type, T<Y>
          FR: F<TR> + ApplicativeEffect // The full return, the effect wrapping the traversable, F<T<X>>
{
    fn traverse(f: T,
                func: impl 'a + Fn(X) -> E + Send + Sync) -> FR;
}

pub trait TraverseEffect<'a, T, E, TR, FR, X, Y>
    where
        T: F<X>,
        E: F<Y>+ Functor2Effect<'a, Y, TR, TR, FX=E, FY=FR, FZ=FR>,
        TR: F<Y>,
        FR: F<TR> + ApplicativeEffect {
    type Fct: Traverse<'a, T, E, TR, FR, X, Y> + Effect;
}

pub fn traverse<'a, T, E, TR, FR, X, Y>(f: T,
                                        func: impl 'a + Fn(X) -> E + Send + Sync) -> FR
where
    T: F<X> + TraverseEffect<'a, T, E, TR, FR, X, Y>,
    E: F<Y> + Functor2Effect<'a, Y, TR, TR, FX=E, FY=FR, FZ=FR>,
    TR: F<Y>,
    FR: F<TR> + ApplicativeEffect {
    T::Fct::traverse(f, func)
}
