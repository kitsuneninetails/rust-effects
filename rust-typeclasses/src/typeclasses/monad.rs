use super::{F, Effect};

/// The `Monad` typeclass.  This just ensures a `flat_map` operation is available for a context
/// of type `F<_>` which operates on a type `X` which can perform a new function returning
/// another context for the given type `X`.  This context is then "flattened" into the originating
/// context, essentially taking its place as the context holder for `X`.
pub trait Monad<'a, FX, FY>: Effect
    where FX: F<Self::In>,
          FY: F<Self::Out> {
    type In;
    type Out;
    fn flat_map(&self, f: FX, func: impl 'a + Fn(Self::In) -> FY + Send + Sync) -> FY;
}

pub trait MonadEffect<'a, FX, FY, X, Y>
    where
        FX: F<X>,
        FY: F<Y> {
    type Fct: Monad<'a, FX, FY, In=X, Out=Y> + Effect;
    fn monad(&self) -> Self::Fct;
}

pub fn flat_map<'a, FX, FY, X, Y>(f: FX, func: impl 'a + Fn(X) -> FY + Send + Sync) -> FY
    where FX: F<X> + MonadEffect<'a, FX, FY, X, Y>,
          FY: F<Y> {
    f.monad().flat_map(f, func)
}

/// A typeclass which can provide a folding feature, which "rolls" up a type into a new type.
/// This is accomplished via an initial value which is then iterated through the type, accumulating
/// a result value via the provided function (which takes the accumulated value and the item in
/// the iteration).  At the end, this accumulated value is returned.
///
/// Typically, the result of a fold is the same type as the initial value, due to init and function
/// both operating ont his value as the fold is accumulated.  However, a type `Z` is provided here
/// to differentiate the final return from the accumulation function.  This allows types like
/// `Future` to accumulate values inside, yet still return a `Future` for that accumulated value
/// rather than blocking for the Future's completion.
pub trait Foldable<'a, FX, X, Y, Z>: Effect
    where FX: F<X> {
    fn fold(&self, f: FX, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Z;
}

pub trait FoldableEffect<'a, FX, X, Y, Z>
    where
        FX: F<X> {
    type Fct: Foldable<'a, FX, X, Y, Z> + Effect;
    fn foldable(&self) -> Self::Fct;
}

pub fn fold<'a, FX, X, Y, Z>(f: FX,
                             init: Y,
                             func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Z
    where FX: F<X> + FoldableEffect<'a, FX, X, Y, Z> {
    f.foldable().fold(f, init, func)
}



