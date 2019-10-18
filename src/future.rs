use super::prelude::*;
use futures::prelude::*;
use futures::future::{ready, BoxFuture, FutureExt};
use futures::Poll;
use futures::task::Context;
use std::pin::Pin;
use std::marker::PhantomData;

pub trait SemigroupFutureInner<'a, T, X>
    where
        X: 'a + Send + Sync {
    fn combine_future_inner<TO>(a: T, b: T) -> T
        where
            TO: 'a + Semigroup<X, X, X> + Send + Sync;
}

pub fn combine_future_inner<'a, T, X, TO>(a: T, b: T) -> T
    where
        X: 'a + Send + Sync,
        T: 'a + SemigroupEffect<T, T, T, Fct: SemigroupFutureInner<'a, T, X>> + Send + Sync,
        TO: 'a + Semigroup<X, X, X> + Send + Sync {
    T::Fct::combine_future_inner::<TO>(a, b)
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

pub fn fut<'a, T>(f: impl 'a + Send + Sync + Future<Output=T>) -> ConcreteFuture<'a, T> {
    ConcreteFuture::new(f)
}

impl<X> F<X> for dyn Future<Output=X> {}
impl<'a, X> F<X> for ConcreteFuture<'a, X> {}

impl<'a, X, X2, XR> SemigroupEffect<
    ConcreteFuture<'a, X>,
    ConcreteFuture<'a, X2>,
    ConcreteFuture<'a, XR>> for ConcreteFuture<'a, X>
    where
        X: 'a + SemigroupEffect<X, X2, XR> + Send + Sync,
        X2: 'a + Send + Sync,
        XR: 'a + Send + Sync {
    type Fct = FutureEffect<'a>;
}
impl<'a, X: 'a + Send + Sync + Default> MonoidEffect<ConcreteFuture<'a, X>> for ConcreteFuture<'a, X> {
    type Fct = FutureEffect<'a>;
}
impl<'a, X: 'a + Send + Sync> ApplicativeEffect for ConcreteFuture<'a, X> {
    type X = X;
    type Fct = FutureEffect<'a>;
}
impl<'a, X, Y> MonadEffect<'a, X, Y> for ConcreteFuture<'a, X>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    type Fct = FutureEffect<'a>;
}
impl<'a, X, Y: Clone> FoldableEffect<'a, X, Y, ConcreteFuture<'a, Y>> for ConcreteFuture<'a, X>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync{
    type FX = ConcreteFuture<'a, X>;
    type Fct = FutureEffect<'a>;
}
impl<'a, X, Y> FunctorEffect<'a, X, Y> for ConcreteFuture<'a, X>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    type Fct = FutureEffect<'a>;
}
impl<'a, X, Y, Z> Functor2Effect<'a, X, Y, Z> for ConcreteFuture<'a, X>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    type FZ = ConcreteFuture<'a, Z>;
    type Fct = FutureEffect<'a>;
}
impl<'a, X: Clone, Y: Clone> ProductableEffect<X, Y> for ConcreteFuture<'a, X>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    type FXY = ConcreteFuture<'a, (X, Y)>;
    type Fct = FutureEffect<'a>;
}

impl<'a, X> Future for ConcreteFuture<'a, X> {
    type Output=X;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

#[derive(Clone, Debug)]
pub struct FutureEffect<'a> {
    _p: PhantomData<&'a()>
}

impl<'a> FutureEffect<'a> {
    pub fn apply() -> FutureEffect<'a> {
        FutureEffect {
            _p: PhantomData,
        }
    }

    fn combine_futures<X1, X2, R, Fn>(a: ConcreteFuture<'a, X1>,
                                      b: ConcreteFuture<'a, X2>,
                                      func: Fn) -> ConcreteFuture<'a, R>
        where
            X1: 'a + Send + Sync,
            X2: 'a + Send + Sync,
            R: 'a + Send + Sync,
            Fn: 'a + FnOnce(X1, X2) -> R + Send + Sync {
        let fr = a.then(move |i| b.map(move |j| func(i, j)));
        ConcreteFuture::new(fr)
    }
}

impl<'a> Effect for FutureEffect<'a> {}

impl<'a, X: 'a + Default + Send> Monoid<ConcreteFuture<'a, X>> for FutureEffect<'a> {
    fn empty() -> ConcreteFuture<'a, X> {
        ConcreteFuture::new(ready(X::default()))
    }
}

impl<'a, X1, X2, R> Semigroup<
    ConcreteFuture<'a, X1>,
    ConcreteFuture<'a, X2>,
    ConcreteFuture<'a, R>> for FutureEffect<'a>
    where
        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync {
    fn combine(a: ConcreteFuture<'a, X1>,
               b: ConcreteFuture<'a, X2>) -> ConcreteFuture<'a, R> {
        Self::combine_futures(a, b, combine)
    }
}

impl <'a, X> SemigroupFutureInner<
    'a,
    ConcreteFuture<'a, X>,
    X> for FutureEffect<'a>
    where
        X: 'a + Send + Sync {
    fn combine_future_inner<TO>(a: ConcreteFuture<'a, X>, b: ConcreteFuture<'a, X>) -> ConcreteFuture<'a, X>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_futures(a, b, TO::combine)
    }
}

impl<'a, X: 'a + Send + Sync> Applicative<X> for FutureEffect<'a> {
    type FX = ConcreteFuture<'a, X>;
    fn pure(x: X) -> Self::FX {
        ConcreteFuture::new(ready(x))
    }
}

impl<'a, X, Y> Functor<
    'a,
    X,
    Y> for FutureEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send) -> Self::FY {
        ConcreteFuture::new(f.map(move |x| func(x)))
    }
}

impl<'a, X, Y, Z> Functor2<'a, X, Y, Z> for FutureEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    type FZ = ConcreteFuture<'a, Z>;
    fn fmap2(fa: Self::FX, fb: Self::FY, func: impl 'a + Fn(X, Y) -> Z + Send) -> Self::FZ {
        let fr = fa.then(move |x| fb.map(move |y| func(x, y)));

        ConcreteFuture::<'a, Z>::new(fr)
    }
}

impl<'a, X, Y> Monad<'a, X, Y> for FutureEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;

    fn flat_map(f: Self::FX, func: impl 'a + Fn(X) -> Self::FY + Send + Sync) -> Self::FY {
        ConcreteFuture::new(f.map(move |x| func(x)).flatten())
    }
}

impl<'a, X, Y> Foldable<'a, X, Y, ConcreteFuture<'a, Y>> for FutureEffect<'a>
    where
        X: 'a + Send,
        Y: 'a + Send {
    type FX = ConcreteFuture<'a, X>;
    fn fold(f: Self::FX,
            init: Y,
            func: impl 'a + Fn(Y, X) -> Y + Send)
        -> ConcreteFuture<'a, Y> {
        ConcreteFuture::new(f.map(move |x| func(init, x)))
    }
}

/// A specialized fold for vectors of Futures which generally have to map and chain the futures
/// together into one big `Future`, rather than accumulate and combine on the fly.
pub fn vfold<'a, X, Y>(f: Vec<ConcreteFuture<'a, X>>,
                   init: Y,
                   func: impl 'a + Fn(Y, X) -> Y + Send + Sync + Copy) -> ConcreteFuture<'a, Y>
    where
        X: 'a + Send,
        Y: 'a + Send {
    let mut accum = ConcreteFuture::new(ready(init));
    for i in f.into_iter() {
        accum = ConcreteFuture::new(accum.then(move |y| i.map(move |x| func(y, x))));
    }
    accum
}

impl<'a, X: Clone, Y: Clone> Productable<X, Y> for FutureEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    type FXY = ConcreteFuture<'a, (X, Y)>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
        fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::executor::block_on;
    use futures::future::lazy;

    #[test]
    fn test_semigroup() {
        block_on(async {
            let f1: ConcreteFuture<u32> = pure(1);
            let f2: ConcreteFuture<u32> = pure(2);
            let fr = combine(f1, f2);
            assert_eq!(fr.await, 3);

            let f1: ConcreteFuture<u32> = pure(3);
            let f2: ConcreteFuture<u32> = pure(5);
            let fr = FutureEffect::combine_future_inner::<IntMulSemigroup>(f1, f2);
            assert_eq!(fr.await, 15);
        });
    }

    #[test]
    fn test_monoid() {
        block_on(async {
            let f: ConcreteFuture<u32> = empty();
            assert_eq!(f.await, 0);
        });
    }

    #[test]
    fn test_applicative() {
        block_on(async {
            let f: ConcreteFuture<u32> = pure(3u32);
            assert_eq!(f.await, 3);
            let f: ConcreteFuture<Result<&str, ()>> = pure(Ok("test"));
            assert_eq!(f.await, Ok("test"));
        });
    }

    #[test]
    fn test_functor() {
        block_on(async {
            let f: ConcreteFuture<u32> = pure(3u32);
            let f = fmap(f, |i| format!("{} strings", i));
            assert_eq!(f.await, "3 strings".to_string());
        });
    }

    #[test]
    fn test_monad() {
        block_on(async {
            let f: ConcreteFuture<u32> = pure(3u32);
            let f2 = flat_map(f, |i| {
                ConcreteFuture::new(lazy(move |_| format!("{} strings", i)))
            });
            assert_eq!(f2.await, "3 strings".to_string());
        });

        block_on(async {
            let f: ConcreteFuture<u32> = pure(3u32);
            let fr = fold(f,
                          10u32,
                          |y, x| y + x);
            assert_eq!(fr.await, 13);
        });

        block_on(async {
            let fs = vec![
                pure(3),
                ConcreteFuture::new(ready(10u32)),
                ConcreteFuture::new(lazy(|_| 4u32))
            ];
            let fr = vfold(fs,
                           0u32,
                           |y, x| y + x);
            assert_eq!(fr.await, 17);
        });
    }

    #[test]
    fn test_product() {
        block_on(async {
            let f1: ConcreteFuture<u32> = pure(3u32);
            let f2: ConcreteFuture<&str> = pure("strings");
            let f = product(f1, f2);
            assert_eq!(f.await, (3, "strings"));
        });
    }

    #[test]
    fn test_traverse() {
        block_on(async {
            let fs: Vec<u32> = vec![3, 10, 4];
            let fr = traverse(fs,
                              |x| ConcreteFuture::new(ready(x + 5)));
            assert_eq!(fr.await, vec![8, 15, 9]);
        });
    }
}
