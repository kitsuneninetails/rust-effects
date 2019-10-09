use super::F;

/// The `Monad` typeclass.  This just ensures a `flat_map` operation is available for a context
/// of type `F<_>` which operates on a type `X` which can perform a new function returning
/// another context for the given type `X`.  This context is then "flattened" into the originating
/// context, essentially taking its place as the context holder for `X`.
pub trait Monad<'a, FX, FY>
    where FX: F<Self::In>,
          FY: F<Self::Out> {
    type In;
    type Out;
    fn flat_map(&self, f: FX, func: impl 'a + Fn(Self::In) -> FY + Send + Sync) -> FY;
}

pub fn flat_map<'a, FX, FY, X, Y>(ev: &impl Monad<'a, FX, FY, In=X, Out=Y>,
                              f: FX, func: impl 'a + Fn(X) -> FY + Send + Sync) -> FY
    where FX: F<X>,
          FY: F<Y> {
    ev.flat_map(f, func)
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
pub trait Foldable<'a, FX, X, Y, Z>
    where FX: F<X> {
    fn fold(&self, f: FX, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Z;
}

/// A specialized fold for vectors of Foldables, especially in cases where the vector should roll
/// up into a different type and/or has some special roll-up mechanics (like `Future` types, which
/// generally have to map and chain the futures together into one big `Future`, rather than
/// accumulate and combine on the fly. As in the normal fold, For this reason, a type `Z` is
/// provided here to differentiate the final return from the accumulation function.
///
/// The type restrictions on the impl are particular because it is a vector operation.  Therefore,
/// the closure must be able to be shared amongst many elements in the vector, potentially being
/// stored in a `Future` or other type, meaning it has to be `move`d into each element's result
/// to prevent lifetime issues, which necessitates that the closure be `Copy`able.
pub trait VecFoldable<'a, FX, X, Y, Z, T>
    where FX: F<X>,
          T: Foldable<'a, FX, X, Y, Z>{
    fn fold(&self, f: Vec<FX>, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync + Copy) -> Z;
}

pub fn fold<'a, FX, X, Y, Z>(ev: &impl Foldable<'a, FX, X, Y, Z>, f: FX,
                             init: Y,
                             func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Z
    where FX: F<X> {
    ev.fold(f, init, func)
}

pub fn vfold<'a, FX, X, Y, Z, T: Foldable<'a, FX, X, Y, Z>>(ev: &impl VecFoldable<'a, FX, X, Y, Z, T>,
                                                            f: Vec<FX>,
                                                            init: Y,
                                                            func: impl 'a + Fn(Y, X) -> Y + Send + Sync + Copy) -> Z
    where FX: F<X> {
    ev.fold(f, init, func)
}
