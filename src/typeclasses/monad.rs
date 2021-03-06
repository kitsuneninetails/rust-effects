use super::{F, Effect};
use crate::typeclasses::applicative::Applicative;
use crate::typeclasses::functor::Functor;

/// The `Monad` typeclass.  This just ensures a `flat_map` operation is available for a context
/// of type `F<_>` which operates on a type `X` which can perform a new function returning
/// another context for the given type `X`.  This context is then "flattened" into the originating
/// context, essentially taking its place as the context holder for `X`.
pub trait Monad<'a>: Effect + Applicative<'a> {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY;
}

pub trait MonadEffect<'a, X, Y> {
    type FX: F<X>;
    type FY: F<Y>;
    type Fct: Monad<'a, X=X, Y=Y, FX=Self::FX, FY=Self::FY> + Effect;
}

pub fn flat_map<'a, FX, FY, X, Y>(f: FX, func: impl 'a + Fn(X) -> FY + Send + Sync) -> FY
    where FX: F<X> + MonadEffect<'a, X, Y, FX=FX, FY=FY>,
          FY: F<Y> {
    FX::Fct::flat_map(f, func)
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
pub trait Foldable<'a>: Effect + Monad<'a> {
    type Y2;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Y2;
}

pub trait FoldableEffect<'a, X, Y, Y2>
    where <<Self as FoldableEffect<'a, X, Y, Y2>>::Fct as Functor<'a>>::FY: F<Y> {
    type FX: F<X>;
    type Fct: Foldable<'a, X=X, Y=Y, Y2=Y2, FX=Self::FX> + Effect;
}

pub fn fold<'a, FX, X, Y, Y2>(f: FX,
                              init: Y,
                              func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Y2
    where FX: F<X> + FoldableEffect<'a, X, Y, Y2, FX=FX>,
          <<FX as FoldableEffect<'a, X, Y, Y2>>::Fct as Functor<'a>>::FY: F<Y>{
    FX::Fct::fold(f, init, func)
}



