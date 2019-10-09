use super::F;

pub trait Monad<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn flat_map(&self, f: FX, func: fn(X) -> FY) -> FY;
}

pub fn flat_map <FX, FY, X, Y>(ev: &impl Monad<FX, FY, X, Y>, f: FX, func: fn(X) -> FY) -> FY
    where FX: F<X>,
          FY: F<Y> {
    ev.flat_map(f, func)
}

/// A typeclass which can provide a folding feature, which "rolls" up a type into a new type.
/// This is accomplished via an initial value which is then iterated through the type, accumulating
/// a result value via the provided function (which takes the accumulated value and the item in
/// the iteration).  At the end, this accumulated value is returned.
/// Typically, the result of a fold is the same type as the initial value, due to init and function
/// both operating ont his value as the fold is accumulated.  However, a type `Z` is provided here
/// to differentiate the final return from the accumulation function.  This allows types like
/// `Future` to accumulate values inside, yet still return a `Future` for that accumulated value
/// rather than blocking for the Future's completion.
pub trait Foldable<FX, X, Y, Z>
    where FX: F<X> {
    fn fold(&self, f: FX, init: Y, func: fn(Y, X) -> Y) -> Z;
}

/// A specialized fold for vectors of Foldables, especially in cases where the vector should roll
/// up into a different type and/or has some special roll-up mechanics (like `Future` types, which
/// generally have to map and chain the futures together into one big `Future`, rather than
/// accumulate and combine on the fly. As in the normal fold, For this reason, a type `Z` is
/// provided here to differentiate the final return from the accumulation function.
pub trait VecFoldable<FX, X, Y, Z, T>
    where FX: F<X>,
          T: Foldable<FX, X, Y, Z>{
    fn fold(&self, f: Vec<FX>, init: Y, func: fn(Y, X) -> Y) -> Z;
}

pub fn fold<FX, X, Y, Z>(ev: &impl Foldable<FX, X, Y, Z>, f: FX, init: Y, func: fn(Y, X) -> Y) -> Z
    where FX: F<X> {
    ev.fold(f, init, func)
}

pub fn vfold<FX, X, Y, Z, T: Foldable<FX, X, Y, Z>>(ev: &impl VecFoldable<FX, X, Y, Z, T>,
                                                    f: Vec<FX>,
                                                    init: Y,
                                                    func: fn(Y, X) -> Y) -> Z
    where FX: F<X> {
    ev.fold(f, init, func)
}
