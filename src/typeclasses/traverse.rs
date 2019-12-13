use super::{F,
            Effect,
            applicative::*,
            functor::*};

/// Basically take a Traversable type constructor T, wrapping a set of concrete types X, and a
/// function which maps the X into a compatible type constructor E (usually an effect, like Future,
/// IO, Result, etc.) which wraps a concrete type Y.
///   _____T_____
/// /            \
/// (* -> *) -> *
///     E       X
///
/// Traverse calls the function on each item in the traversable, to generate an interim structure
/// with the Traversable holding the results of the computation.  Then, Traverse will flip the
/// structure, with a single effect E wrapping the Traversable of concrete Y values.
///
/// e.g. T<X> => traverse (gets a T<E<Y>> as an interim value) => E<T<Y>>
pub trait Traverse<'a, T, E, TR, FR, X, Y>: Effect
    where T: F<X>, // The Traversable type (Vec, Option, etc.), wrapping the input(s): F<X>
          E: F<Y> + Functor2Effect<'a, Y, TR, TR, FY=FR, FZ=FR>, // The effect returned from func, wrapping a concrete type Y, E<Y>
          TR: F<Y>, // The Traversable type to return, wrapping the effect's internal type, T<Y>
          FR: F<TR> + ApplicativeEffect<'a> // The full return, the effect wrapping the traversable, E<T<Y>>
{
    fn traverse(f: T,
                func: impl 'a + Fn(X) -> E + Send + Sync) -> FR;
}

pub trait TraverseEffect<'a, E, TR, FR, X, Y>: F<X> + Sized
    where
        E: F<Y>+ Functor2Effect<'a, Y, TR, TR, FY=FR, FZ=FR>,
        TR: F<Y>,
        FR: F<TR> + ApplicativeEffect<'a> {
    type Fct: Traverse<'a, Self, E, TR, FR, X, Y> + Effect;
}

pub fn traverse<'a, T, E, TR, FR, X, Y>(f: T,
                                        func: impl 'a + Fn(X) -> E + Send + Sync) -> FR
where
    T: TraverseEffect<'a, E, TR, FR, X, Y>,
    E: F<Y> + Functor2Effect<'a, Y, TR, TR, FY=FR, FZ=FR>,
    TR: F<Y>,
    FR: F<TR> + ApplicativeEffect<'a> {
    T::Fct::traverse(f, func)
}
