use super::F;

pub trait Functor<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn fmap(&self, f: FX, func: fn(X) -> Y) -> FY;
}

pub fn fmap<FX: F<X>, FY: F<Y>, X, Y>(ev: &impl Functor<FX, FY, X, Y>, f: FX, func: fn(X) -> Y) -> FY {
    ev.fmap(f, func)
}

pub trait Functor2<FX, FY, FZ, X, Y, Z>
    where FX: F<X>,
          FY: F<Y>,
          FZ: F<Z> {
    fn fmap2(&self, fa: FX, fb: FY, func: fn(&X, &Y) -> Z) -> FZ;
}

pub fn fmap2<FX: F<X>, FY: F<Y>, FZ: F<Z>, X, Y, Z>(ev: &impl Functor2<FX, FY, FZ, X, Y, Z>,
                                                    fa: FX, fb: FY, func: fn(&X, &Y) -> Z) -> FZ {
    ev.fmap2(fa, fb, func)
}
