use crate::typeclasses::applicative::Applicative;

/// The `Monad` typeclass.  This just ensures a `flat_map` operation is available for a context
/// of type `F<_>` which operates on a type `X` which can perform a new function returning
/// another context for the given type `X`.  This context is then "flattened" into the originating
/// context, essentially taking its place as the context holder for `X`.
pub trait Monad<'a>: Applicative<'a> {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY;
}

/// Foldable is a typeclass which can provide a folding feature, which "rolls" up a type into a
/// new type.  This is accomplished via an initial value which is then iterated through the type,
/// accumulating a result value via the provided function (which takes the accumulated value and
/// the item in the iteration).  At the end, this accumulated value is returned.
///
/// Typically, the result of a fold is the same type as the initial value, due to init and function
/// both operating on this value as the fold is accumulated.  However, a type `Y2` is provided here
/// to differentiate the final return from the accumulation function.  This allows types like
/// `Future` to accumulate values inside, yet still return a `Future` for that accumulated value
/// rather than blocking for the Future's completion.
pub trait Foldable<'a>: Monad<'a> {
    type Y2;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Y2;
}



