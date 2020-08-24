use super::{
    F,
    Effect
};

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
pub trait Traverse<'a, TIn, EffIn, TRet, EffRet, X, Y>: Effect
    where TIn: F<X>, // The input Traversable type (Vec, Option, etc.), carrying data of type X.
          EffIn: F<Y>, // The effect returned from func, wrapping a concrete type Y
          TRet: F<Y>, // The Traversable type to return, carrying data of type Y.
          EffRet: F<TRet> // The full return, the effect wrapping the traversable, TRet
{
    fn traverse(f: TIn,
                func: impl 'a + Fn(X) -> EffIn + Send + Sync) -> EffRet;
}
