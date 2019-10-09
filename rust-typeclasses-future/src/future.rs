use rust_typeclasses::typeclasses::{F,
                                    applicative::*,
                                    functor::*,
                                    monad::*,
                                    monoid::*,
                                    product::*,
                                    semigroup::*};
use futures::prelude::*;
use futures::future::{ready, BoxFuture};
use futures::Poll;
use futures::task::Context;
use std::marker::PhantomData;
use std::pin::Pin;

pub struct FutureInnerSemigroup<'a, X, X2, XR, T: 'static> {
    pub t: T,
    _p1: PhantomData<X>,
    _p2: PhantomData<X2>,
    _p3: PhantomData<XR>,
    _p4: PhantomData<&'a()>
}

impl<'a, X, X2, XR, T: 'static> FutureInnerSemigroup<'a, X, X2, XR, T> {
    pub fn apply(t: T) -> Self {
        FutureInnerSemigroup {
            t,
            _p1: PhantomData,
            _p2: PhantomData,
            _p3: PhantomData,
            _p4: PhantomData,
        }
    }
}

pub struct ConcreteFuture<'a, X> {
    pub inner: BoxFuture<'a, X>
}

    impl<'a, X> ConcreteFuture<'a, X> {
    pub fn new<F: 'a + Future<Output=X> + Send>(f: F) -> Self {
        ConcreteFuture {
            inner: f.boxed()
        }
    }
}

impl<'a, X> F<X> for ConcreteFuture<'a, X> {}

impl<'a, X> Future for ConcreteFuture<'a, X> {
    type Output=X;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

pub struct FutureEffect;
impl FutureEffect {
    pub fn sg<X, X2, XR, T: Semigroup<X, X2, XR>>(&self, ev: T) -> FutureInnerSemigroup<X, X2, XR, T>{
        FutureInnerSemigroup::apply(ev)
    }
}

pub const FUT_EV: &FutureEffect = &FutureEffect;

impl<'a, X: 'a + Default + Send> Monoid<ConcreteFuture<'a, X>> for FutureEffect {
    fn empty(&self) -> ConcreteFuture<'a, X> {
        ConcreteFuture::new(ready(X::default()))
    }
}

impl<'a, X1, X2, R, T> Semigroup<
    ConcreteFuture<'a, X1>,
    ConcreteFuture<'a, X2>,
    ConcreteFuture<'a, R>> for FutureInnerSemigroup<'a, X1, X2, R, T>
    where
        X1: 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync,
        T: Semigroup<X1, X2, R> + Send + Sync {
    fn combine(self,
               a: ConcreteFuture<'a, X1>,
               b: ConcreteFuture<'a, X2>) -> ConcreteFuture<'a, R> {
        let fr = a.then(move |i| b.map(move |j| combine(self.t, i, j)));

        ConcreteFuture::new(fr)
    }
}

impl<'a, X> Applicative<ConcreteFuture<'a, X>, X> for FutureEffect
    where
        X:  'a + Send + Sync {
    fn pure(&self, x: X) -> ConcreteFuture<'a, X> {
        ConcreteFuture::new(ready(x))
    }
}

impl<'a, X, Y> Functor<
    ConcreteFuture<'a, X>,
    ConcreteFuture<'a, Y>,
    X,
    Y> for FutureEffect
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    fn fmap(&self, f: ConcreteFuture<'a, X>, func: fn(X) -> Y) -> ConcreteFuture<'a, Y> {
        ConcreteFuture::new(f.map(move |x| func(x)))
    }
}

impl<'a, X, Y, Z> Functor2<
    ConcreteFuture<'a, X>,
    ConcreteFuture<'a, Y>,
    ConcreteFuture<'a, Z>,
    X,
    Y,
    Z> for FutureEffect
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    fn fmap2(&self,
             fa: ConcreteFuture<'a, X>,
             fb: ConcreteFuture<'a, Y>,
             func: fn(&X, &Y) -> Z) -> ConcreteFuture<'a, Z> {
        let fr = fa.then(move |x| fb.map(move |y| func(&x,&y)));

        ConcreteFuture::<'a, Z>::new(fr)
    }
}

impl<'a, X, Y> Monad<
    ConcreteFuture<'a, X>,
    ConcreteFuture<'a, Y>,
    X,
    Y> for FutureEffect
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    fn flat_map(&self,
                f: ConcreteFuture<'a, X>,
                func: fn(X) -> ConcreteFuture<'a, Y>) -> ConcreteFuture<'a, Y> {
        ConcreteFuture::new(f.map(move |x| func(x)).flatten())
    }
}

impl<'a, X, Y> Foldable<
    ConcreteFuture<'a, X>,
    X,
    Y,
    ConcreteFuture<'a, Y>> for FutureEffect
    where
        X: 'a + Send,
        Y: 'a + Send {
    fn fold(&self,
            f: ConcreteFuture<'a, X>,
            init: Y,
            func: fn(Y, X) -> Y)
        -> ConcreteFuture<'a, Y> {
        ConcreteFuture::new(f.map(move |x| func(init, x)))
    }
}

impl<'a, X, Y> VecFoldable<
    ConcreteFuture<'a, X>,
    X,
    Y,
    ConcreteFuture<'a, Y>,
    FutureEffect> for FutureEffect
    where
        X: 'a + Send,
        Y: 'a + Send {
    fn fold(&self,
            f: Vec<ConcreteFuture<'a, X>>,
            init: Y,
            func: fn(Y, X) -> Y)
        -> ConcreteFuture<'a, Y> {
        f.into_iter()
            .fold(ConcreteFuture::new(ready(init)),
                  |y, x| {
                      ConcreteFuture::new(y.map(move |yval| x.map(move |xval| func(yval, xval))).flatten())
                  })
    }
}

impl<'a, X: Clone, Y: Clone> Productable<
    ConcreteFuture<'a, X>,
    ConcreteFuture<'a, Y>,
    ConcreteFuture<'a, (X, Y)>,
    X,
    Y> for FutureEffect
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    fn product(&self,
               fa: ConcreteFuture<'a, X>,
               fb: ConcreteFuture<'a, Y>) -> ConcreteFuture<'a, (X, Y)> {
        fmap2(FUT_EV, fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

//impl<'a, E, FR, X, Y> Traverse<
//    ConcreteFuture<'a, X>,
//    E,
//    ConcreteFuture<'a, Y>,
//    FR,
//    X,
//    Y> for FutureEffect
//    where
//        Y: Clone,
//        E: F<Y>,
//        FR: F<ConcreteFuture<'a, Y>> {
//    fn traverse(&self,
//                e_effect: &(impl Applicative<FR, Vec<Y>> + Functor2<E, FR, FR, Y, Vec<Y>, Vec<Y>>),
//                fa: ConcreteFuture<'a, X>,
//                func: fn(X) -> E) -> FR {
//
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_typeclasses::typeclasses::traverse::traverse;
    use rust_typeclasses::vec::*;

    use futures::executor::block_on;
    use futures::future::lazy;

    #[test]
    fn test_semigroup() {
        block_on(async {
            let f1 = pure(FUT_EV, 1u32);
            let f2 = pure(FUT_EV, 2u32);
            let fr = combine(FUT_EV.sg(IADD_SG), f1, f2);
            assert_eq!(fr.await, 3);
        });
    }

    #[test]
    fn test_monoid() {
        block_on(async {
            let f: ConcreteFuture<u32> = empty(FUT_EV);
            assert_eq!(f.await, 0);
        });
    }

    #[test]
    fn test_applicative() {
        block_on(async {
            let f = pure(FUT_EV, 3u32);
            assert_eq!(f.await, 3);
            let f: ConcreteFuture<Result<&str, ()>> = pure(FUT_EV, Ok("test"));
            assert_eq!(f.await, Ok("test"));
        });
    }

    #[test]
    fn test_functor() {
        block_on(async {
            let f = pure(FUT_EV, 3u32);
            let f = fmap(FUT_EV, f, |i| format!("{} strings", i));
            assert_eq!(f.await, "3 strings".to_string());
        });
    }

    #[test]
    fn test_monad() {
        block_on(async {
            let f = pure(FUT_EV, 3u32);
            let f2 = flat_map(FUT_EV, f, |i| {
                ConcreteFuture::new(lazy(move |_| format!("{} strings", i)))
            });
            assert_eq!(f2.await, "3 strings".to_string());
        });

        block_on(async {
            let f = pure(FUT_EV, 3u32);
            let fr = fold(FUT_EV,
                          f,
                          10u32,
                          |y, x| y + x);
            assert_eq!(fr.await, 13);
        });

        block_on(async {
            let fs = vec![
                pure(FUT_EV, 3u32),
                ConcreteFuture::new(ready(10u32)),
                ConcreteFuture::new(lazy(|_| 4u32))
            ];
            let fr = vfold(FUT_EV,
                           fs,
                           0u32,
                           |y, x| y + x);
            assert_eq!(fr.await, 17);
        });
    }

    #[test]
    fn test_product() {
        block_on(async {
            let f1 = pure(FUT_EV, 3u32);
            let f2 = pure(FUT_EV, "strings");
            let f = product(FUT_EV, f1, f2);
            assert_eq!(f.await, (3, "strings"));
        });
    }

    #[test]
    fn test_traverse() {
        block_on(async {
            let fs: Vec<u32> = vec![3, 10, 4];
            let fr = traverse(VEC_EV,
                              FUT_EV,
                              fs,
                              |x| ConcreteFuture::new(ready(x + 5)));
            assert_eq!(fr.await, vec![9, 15, 8]);
        });
    }
}
