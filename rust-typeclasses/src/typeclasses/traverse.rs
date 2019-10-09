use super::{F,
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
pub trait Traverse<'a, T, E, TR, FR, X, Y>
    where T: F<X>, // The Traversable type (Vec, Option, etc.), wrapping the effect, T<E<X>>
          E: F<Y>, // The effect returned from func, wrapping a concrete type Y, E<Y>
          TR: F<Y>, // The Traversable type to return, wrapping the effect's internal type, T<Y>
          FR: F<TR> // The full return, the effect wrapping the traversable, F<T<X>>
{
    fn traverse(&self,
                e_effect: &(impl Applicative<FR, TR> + Functor2<'a, E, FR, FR, Y, TR, TR> + Send + Sync),
                f: T,
                func: impl 'a + Fn(X) -> E + Send + Sync) -> FR;
}

pub fn traverse<'a, T: F<X>, E: F<Y>, TR: F<Y>, FR: F<TR>, X, Y>(
    ev: &impl Traverse<'a, T, E, TR, FR, X, Y>,
    e_effect: &(impl Applicative<FR, TR> + Functor2<'a, E, FR, FR, Y, TR, TR> + Send + Sync),
    f: T,
    func: impl 'a + Fn(X) -> E + Send + Sync) -> FR {
    ev.traverse(e_effect, f, func)
}
