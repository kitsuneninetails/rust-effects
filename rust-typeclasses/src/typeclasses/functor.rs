use super::{F, Effect};

pub trait Functor<'a, FX, FY, X, Y>: Effect
    where FX: F<X>,
          FY: F<Y> {
    fn fmap(&self, f: FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> FY;
}

pub trait FunctorEffect<'a, FX, FY, X, Y>
    where
        FX: F<X>,
        FY: F<Y> {
    type Fct: Functor<'a, FX, FY, X, Y> + Effect;
    fn functor(&self) -> Self::Fct;
}

pub fn fmap<'a, FX: F<X> + FunctorEffect<'a, FX, FY, X, Y>, FY: F<Y>, X, Y>(f: FX,
                                          func: impl 'a + Fn(X) -> Y + Send + Sync) -> FY {
    f.functor().fmap(f, func)
}

pub trait Functor2<'a, FX, FY, FZ, X, Y, Z>: Effect
    where FX: F<X>,
          FY: F<Y>,
          FZ: F<Z> {
    fn fmap2(&self, fa: FX, fb: FY, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> FZ;
}

pub trait Functor2Effect<'a, FX, FY, FZ, X, Y, Z>
    where
        FX: F<X>,
        FY: F<Y>,
        FZ: F<Z> {
    type Fct: Functor2<'a, FX, FY, FZ, X, Y, Z> + Effect;
    fn functor2(&self) -> Self::Fct;
}

pub fn fmap2<'a, FX: F<X>+ Functor2Effect<'a, FX, FY, FZ, X, Y, Z>, FY: F<Y>, FZ: F<Z>, X, Y, Z>(
    fa: FX,
    fb: FY,
    func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> FZ {
    fa.functor2().fmap2(fa, fb, func)
}
