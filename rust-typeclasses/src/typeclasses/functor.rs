use super::F;

pub trait Functor<'a, FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn fmap(&self, f: FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> FY;
}

pub fn fmap<'a, FX: F<X>, FY: F<Y>, X, Y>(ev: &impl Functor<'a, FX, FY, X, Y>,
                                          f: FX,
                                          func: impl 'a + Fn(X) -> Y + Send + Sync) -> FY {
    ev.fmap(f, func)
}

pub trait Functor2<'a, FX, FY, FZ, X, Y, Z>
    where FX: F<X>,
          FY: F<Y>,
          FZ: F<Z> {
    fn fmap2(&self, fa: FX, fb: FY, func: impl 'a + Fn(&X, &Y) -> Z + Send + Sync) -> FZ;
}

pub fn fmap2<'a, FX: F<X>, FY: F<Y>, FZ: F<Z>, X, Y, Z>(ev: &impl Functor2<'a, FX, FY, FZ, X, Y, Z>,
                                                        fa: FX,
                                                        fb: FY,
                                                        func: impl 'a + Fn(&X, &Y) -> Z + Send + Sync) -> FZ {
    ev.fmap2(fa, fb, func)
}
