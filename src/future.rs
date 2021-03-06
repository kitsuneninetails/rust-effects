/// Future Typeclass Behaviors
///
/// The type stored by this future is a simple type X.  Any mapping functions must return
/// successful types.
///
/// Note: Any type wrapped by Future must implement `Send` and `Sync` in order to be
/// dispatched to the execution context.
///
///
/// Note: Most functions can return a different future type (Ready vs. Lazy vs. AndThen vs. Map,
/// etc.).  They should all qualify as implementations of Future for the given type, however.
/// For all rules below the "Future" type is a ConcreteFuture implementaiton, which wraps a
/// `Future` trait object (but is needed for its `Sized` implementation).
///
/// Semigroup
///     `combine(Future(X1), Future(X2)) => Future(combine(X1, X2))`
///     Note: R1 and R2 are results, so they combine as results would combine.
/// Monoid
///     `empty() => Future(X::empty())`
///     Note: This returns a valid future of the empty value of the future's type.  The type
///     must have an associated Monoid.
/// Applicative
///     `pure(X) => Future(X)` // uses `ready` future
///     Note: This is greedy and will perform any function given to come up with a value before
///     creating the future!
/// ApplicativeApply
///     `apply(Future(fn X -> X2)), Future(X)) => Future(fn(X)) => Future(X2)`
///     Note: This is lazy and will perform the function when the future.`await` is called
/// Functor
///     `fmap(Future(X), fn X -> X2) => Future(fn(X)) => Future(X2)`
///     Note: This is lazy and will perform the function when the future.`await` is called
/// Functor2
///     `fmap2(Future(X), Future(Y), fn X, Y -> Z) => Future(fn(X, Y)) => Future(Z)`
///     Note: This is lazy and will perform the function when the future.`await` is called
/// Monad
///     `flat_map(Future(X), fn X -> Future(Y)) => fn(X) => Future(Y)`
///     Note: This is lazy and will perform the function when the future.`await` is called.
/// MonadError
///     `Not implemented`
/// Foldable
///     `fold(Future(X), Y, fn Y, X -> Y2) => Future(fn(Y, X)) => Future(Y2)`
///     Note: Y and Y2 are the same type, just possibly two different values. To preserve the
///     'future-ness' of the result, it is essentially the same as a `fmap.`
/// Productable -
///     `product(Future(X), Future(Y)) => Future((X, Y))`
/// Traverse
///     `Not implemented`
use super::prelude::*;
use futures::prelude::*;
use futures::future::{ready, BoxFuture, FutureExt};
use futures::Poll;
use futures::task::Context;
use std::pin::Pin;
use std::marker::PhantomData;

use crate::*;

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

semigroup_effect! { S, ConcreteFuture, FutureEffect }
monoid_effect! { S, ConcreteFuture, FutureEffect }
applicative_effect! { S, ConcreteFuture, FutureEffect }
functor_effect! { S, ConcreteFuture, FutureEffect }
functor2_effect! { S, ConcreteFuture, FutureEffect }
monad_effect! { S, ConcreteFuture, FutureEffect }
foldable_effect! { S, ConcreteFuture, FutureEffect }
productable_effect! { S, ConcreteFuture, FutureEffect }

impl<'a, X> Future for ConcreteFuture<'a, X> {
    type Output=X;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

#[derive(Clone, Debug)]
pub struct FutureEffect<'a, X=(), Y=(), Z=()> {
    _p: PhantomData<&'a()>,
    _a: PhantomData<X>,
    _b: PhantomData<Y>,
    _c: PhantomData<Z>
}

impl<'a, X, Y, Z> FutureEffect<'a, X, Y, Z> {
    pub fn new(_: Z) -> Self {
        FutureEffect {
            _p: PhantomData,
            _a: PhantomData,
            _b: PhantomData,
            _c: PhantomData
        }
    }

    pub(crate) fn combine_futures<X1, X2, R, Fn>(a: ConcreteFuture<'a, X1>,
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

#[macro_export]
macro_rules! future_monad {
    () => (FutureEffect::new(()))
}

impl<'a, X, Y, Z> Effect for FutureEffect<'a, X, Y, Z> {}

impl<'a, X, Y, Z> Monoid<ConcreteFuture<'a, X>> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + MonoidEffect<X> + Sync + Send {
    fn empty() -> ConcreteFuture<'a, X> {
        ConcreteFuture::new(ready(empty::<X>()))
    }
}

impl<'a, X1, X2, R> Semigroup<
    ConcreteFuture<'a, X1>,
    ConcreteFuture<'a, X2>,
    ConcreteFuture<'a, R>> for FutureEffect<'a, X1, X2, R>
    where
        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync {
    fn combine(a: ConcreteFuture<'a, X1>,
               b: ConcreteFuture<'a, X2>) -> ConcreteFuture<'a, R> {
        Self::combine_futures(a, b, combine)
    }
}

impl <'a, X> SemigroupInner<'a, ConcreteFuture<'a, X>, X> for FutureEffect<'a, X, X, X>
    where
        X: 'a + Send + Sync {
    fn combine_inner<TO>(a: ConcreteFuture<'a, X>, b: ConcreteFuture<'a, X>) -> ConcreteFuture<'a, X>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_futures(a, b, TO::combine)
    }
}

impl<'a, X, Y, Z> Functor<'a> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type X = X;
    type Y = Y;
    type FX = ConcreteFuture<'a, X>;
    type FY = ConcreteFuture<'a, Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send) -> Self::FY {
        ConcreteFuture::new(f.map(move |x| func(x)))
    }
}

impl<'a, X, Y, Z> Applicative<'a> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    fn pure(x: X) -> Self::FX {
        ConcreteFuture::new(ready(x))
    }
}

impl<'a, X, Y, Z, M> ApplicativeApply<'a, M> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync,
        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
    type FMapper = ConcreteFuture<'a, M>;
    fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
        ConcreteFuture::new(func.map(move |f| x.map(move |x_in| f(x_in))).flatten())
    }
}

impl<'a, X, Y, Z> Functor2<'a> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    type Z = Z;
    type FZ = ConcreteFuture<'a, Z>;
    fn fmap2(fa: Self::FX,
             fb: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        let fr = fa.then(move |x| fb.map(move |y| func(x, y)));

        ConcreteFuture::<'a, Z>::new(fr)
    }
}

impl<'a, X, Y, Z> Monad<'a> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        ConcreteFuture::new(f.map(move |x| func(x)).flatten())
    }
}

impl<'a, X, Y, Z> Foldable<'a> for FutureEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    type Y2 = ConcreteFuture<'a, Y>;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Y2 {
        ConcreteFuture::new(f.map(move |x| func(init, x)))
    }
}

/// A specialized fold for vectors of Futures which generally have to map and chain the futures
/// together into one big `Future`, rather than accumulate and combine on the fly.
//pub fn vfold<'a, X, Y>(f: Vec<ConcreteFuture<'a, X>>,
//                          init: Y,
//                          func: impl 'a + Fn(Y, X) -> Y + Send + Sync + Copy) -> ConcreteFuture<'a, Y>
//    where
//        X: 'a + Send + Sync,
//        Y: 'a + Send + Sync {
//    VecEffect::<ConcreteFuture<X>, ConcreteFuture<Y>>::fold(
//        f,
//        FutureEffect::<'a, Y>::pure(init), |a, i|
//            ConcreteFuture::new(a.then(move|y| i.map(move |x| func(y, x)))))
//}

impl<'a, X, Y> Productable<'a> for FutureEffect<'a, X, Y, (X, Y)>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {}

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
            let fr = FutureEffect::combine_inner::<IntMulSemigroup>(f1, f2);
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

//        block_on(async {
//            let fs = vec![
//                pure(3),
//                ConcreteFuture::new(ready(10u32)),
//                ConcreteFuture::new(lazy(|_| 4u32))
//            ];
//            let fr = vfold(fs,
//                           0u32,
//                           |y, x| y + x);
//            assert_eq!(fr.await, 17);
//        });
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

//    #[test]
//    fn test_traverse() {
//        block_on(async {
//            let fs: Vec<u32> = vec![3, 10, 4];
//            let fr = traverse(fs,
//                              |x| ConcreteFuture::new(ready(x + 5)));
//            assert_eq!(fr.await, vec![8, 15, 9]);
//        });
//    }
}
