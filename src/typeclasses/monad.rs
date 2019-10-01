use super::F;

pub trait Monad<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn flat_map(self, f: FX, func: fn(X) -> FY) -> FY;
}

pub fn flat_map <FX, FY, X, Y>(ev: impl Monad<FX, FY, X, Y>, f: FX, func: fn(X) -> FY) -> FY
    where FX: F<X>,
          FY: F<Y> {
    ev.flat_map(f, func)
}

pub trait Foldable<FX, X, Y>
    where FX: F<X> {
    fn fold(f: FX, init: Y, func: impl Fn(Y, X) -> Y) -> Y;
}

pub fn fold<FX, X, Y>(f: FX, init: Y, func: impl Fn(Y, X) -> Y) -> Y
    where FX: F<X> + Foldable<FX, X, Y> {
    FX::fold(f, init, func)
}
