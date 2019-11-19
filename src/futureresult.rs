/// Future (Result-aware) Typeclass Behaviors
///
/// Note: Any type wrapped by Future must implement `Send` and `Sync` in order to be
/// dispatched to the execution context.

/// Semigroup
///     `combine(Future(X), Future(Y)) => Future(combine(X, Y))`
/// Monoid
///     `empty() => Future(T1::default())` // uses `ready` future
///     Note: This returns a valid future of the default value of the future's type.
/// Applicative
///     `pure(X) => Future(X)` // uses `ready` future
///     Note: This is greedy and will perform any function given to come up with a value before
///     creating the future!
/// Functor
///     `fmap(Future(X), fn T1 -> T2) => Future(fn(X))`
///     Note: This is lazy and will perform the function when the future.`await` is called
/// Functor2
///     `fmap2(Future(X), Future(Y), fn T1 T2 -> T3) => Future(fn(X, Y))`
///     Note: This is lazy and will perform the function when the future.`await` is called
/// Monad
///     `flat_map(Future(X), fn T1 -> Future<T2>) => Future(*fn(X))` if fn(X) returns Some(Y)
///     Note: This is lazy and will perform the function when the future.`await` is called.
///     Also, this can return a different future type (Ready vs. Lazy vs. AndThen vs. Map, etc.)
/// Foldable
///     `fold(Future(X), init, fn TI T1 -> TI) => Future(fn(init, X))`
///     Note: To preserve the 'future-ness' of the result, it is essentially the same as a `fmap.`
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
use std::fmt::Debug;

use crate::*;
use futures::executor::block_on;

pub struct ConcreteFutureResult<'a, X, E> {
    pub inner: BoxFuture<'a, Result<X, E>>
}

impl<'a, E, X> ConcreteFutureResult<'a, X, E> {
    pub fn new<F: 'a + Future<Output=Result<X, E>> + Send>(f: F) -> Self {
        ConcreteFutureResult {
            inner: f.boxed()
        }
    }
}

pub fn fut_res<'a, T, E>(f: impl 'a + Send + Future<Output=Result<T, E>>) -> ConcreteFutureResult<'a, T, E> {
    ConcreteFutureResult::new(f)
}

impl<X, E> F<X> for dyn Future<Output=Result<X, E>> {}
impl<'a, E, X> F<X> for ConcreteFutureResult<'a, X, E> {}

semigroup_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
monoid_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
applicative_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
functor_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
functor2_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
monad_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
monaderror_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
foldable_effect! { 2S, ConcreteFutureResult, FutureResultEffect }
productable_effect! { 2S, ConcreteFutureResult, FutureResultEffect }

impl<'a, E, X> Future for ConcreteFutureResult<'a, X, E> {
    type Output=Result<X, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

#[derive(Clone, Debug)]
pub struct FutureResultEffect<'a, E=(), X=(), Y=(), Z=()> {
    _p: PhantomData<&'a()>,
    _a: PhantomData<X>,
    _b: PhantomData<Y>,
    _c: PhantomData<Z>,
    _e: PhantomData<E>,
}

impl<'a, E, X, Y, Z> FutureResultEffect<'a, E, X, Y, Z> {
    pub fn apply(_: Z) -> Self {
        FutureResultEffect {
            _p: PhantomData,
            _a: PhantomData,
            _b: PhantomData,
            _c: PhantomData,
            _e: PhantomData,
        }
    }

    pub(crate) fn combine_futures<X1, X2, R, Fn>(a: ConcreteFutureResult<'a, X1, E>,
                                                 b: ConcreteFutureResult<'a, X2, E>,
                                                 func: Fn) -> ConcreteFutureResult<'a, R, E>
        where
            X1: 'a + Send + Sync,
            X2: 'a + Send + Sync,
            R: 'a + Send + Sync,
            Fn: 'a + FnOnce(X1, X2) -> R + Send + Sync,
            E: 'a + Sync + Send {
        let fr = a.then(
            move |a_fut| b.map(
                move |b_fut| a_fut.and_then(
                    |a_in| b_fut.map(
                        |b_in| func(a_in, b_in)
                    )
                )
            )
        );
        ConcreteFutureResult::new(fr)
    }
}

#[macro_export]
macro_rules! future_result_monad {
    () => (FutureResultEffect::apply(()))
}

impl<'a, E, X, Y, Z> Effect for FutureResultEffect<'a, E, X, Y, Z> {}

impl<'a, X, Y, Z, E> Monoid<ConcreteFutureResult<'a, X, E>> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + MonoidEffect<X> + Sync + Send,
        E: 'a + Sync + Send {
    fn empty() -> ConcreteFutureResult<'a, X, E> {
        ConcreteFutureResult::new(ready(Ok(X::Fct::empty())))
    }
}

impl<'a, X1, X2, R, E> Semigroup<
    ConcreteFutureResult<'a, X1, E>,
    ConcreteFutureResult<'a, X2, E>,
    ConcreteFutureResult<'a, R, E>> for FutureResultEffect<'a, E, X1, X2, R>
    where
        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug  {
    fn combine(a: ConcreteFutureResult<'a, X1, E>,
               b: ConcreteFutureResult<'a, X2, E>) -> ConcreteFutureResult<'a, R, E> {
        Self::combine_futures(a, b, combine)
    }
}

impl <'a, E, X> SemigroupInner<'a, ConcreteFutureResult<'a, X, E>, X> for FutureResultEffect<'a, E, X, X, X>
    where
        X: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn combine_inner<TO>(a: ConcreteFutureResult<'a, X, E>,
                         b: ConcreteFutureResult<'a, X, E>) -> ConcreteFutureResult<'a, X, E>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_futures(a, b, TO::combine)
    }
}

impl<'a, E, X, Y, Z> Functor<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug  {
    type X = X;
    type Y = Y;
    type FX = ConcreteFutureResult<'a, X, E>;
    type FY = ConcreteFutureResult<'a, Y, E>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send) -> Self::FY {
        ConcreteFutureResult::new(f.map(move |f_fut| f_fut.map(|x_in| func(x_in))))
    }
}

impl<'a, X, Y, Z, E> Applicative<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn pure(x: X) -> Self::FX {
        ConcreteFutureResult::new(ready(Ok(x)))
    }
}

impl<'a, E, X, Y, Z> Functor2<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type Z = Z;
    type FZ = ConcreteFutureResult<'a, Z, E>;
    fn fmap2(fa: Self::FX,
             fb: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        let fr = fa.then(move |a_fut| fb.map(move |b_fut| a_fut.and_then(|a_in| b_fut.map(|b_in| func(a_in, b_in)))));

        ConcreteFutureResult::<'a, Z, E>::new(fr)
    }
}

impl<'a, E, X, Y, Z> Monad<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        let res = ConcreteFutureResult::new(f.then(move |f_fut| match f_fut {
            Ok(f_in) => func(f_in),
            Err(e) => ConcreteFutureResult::new(ready(Err(e)))
        }));
        res
    }
}

impl<'a, E, X, Y, Z> Foldable<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type Z = ConcreteFutureResult<'a, Y, E>;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Z {
        ConcreteFutureResult::new(f.map(move |f_fut| f_fut.map(|f_in| func(init, f_in))))
    }
}

impl<'a, E, X, Y, Z> MonadError<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type E=E;
    fn raise_error(err: Self::E) -> Self::FX {
        fut_res(ready(Err(err)))
    }

    fn handle_error(f: Self::FX, recovery: impl 'a + Send + Sync + Fn(Self::E) -> Self::FX) -> Self::FX {
        fut_res(f.then(move |r| match r {
            Ok(o) => pure(o),
            Err(e) => recovery(e)
        }))
    }

    fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
        block_on(async {
            f.await
        })
    }
}

/// A specialized fold for vectors of Futures which generally have to map and chain the futures
/// together into one big `Future`, rather than accumulate and combine on the fly.
pub fn vfold<'a, X, Y, E>(f: Vec<ConcreteFutureResult<'a, X, E>>,
                          init: Y,
                          func: impl 'a + Fn(Y, X) -> Y + Send + Sync + Copy) -> ConcreteFutureResult<'a, Y, E>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    VecEffect::<ConcreteFutureResult<X, E>, ConcreteFutureResult<Y, E>>::fold(
        f,
        FutureResultEffect::<'a, E, Y>::pure(init),
        |a, i| ConcreteFutureResult::new(
            a.then(
                move|a_fut| i.map(
                    move |i_fut| a_fut.and_then(|a_in| i_fut.map(|i_in| func(a_in, i_in)))
                )
            )
        )
    )
}

impl<'a, X: Clone, Y: Clone, Z, E> Productable<'a> for FutureResultEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type FXY = ConcreteFutureResult<'a, (X, Y), E>;
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
            let f1: ConcreteFutureResult<u32, ()> = pure(1);
            let f2: ConcreteFutureResult<u32, ()> = pure(2);
            let fr = combine(f1, f2);
            assert_eq!(fr.await, Ok(3));

            let f1: ConcreteFutureResult<u32, ()> = pure(3);
            let f2: ConcreteFutureResult<u32, ()> = pure(5);
            let fr = FutureResultEffect::combine_inner::<IntMulSemigroup>(f1, f2);
            assert_eq!(fr.await, Ok(15));
        });
    }

    #[test]
    fn test_monoid() {
        block_on(async {
            let f: ConcreteFutureResult<u32, ()> = empty();
            assert_eq!(f.await, Ok(0));
        });
    }

    #[test]
    fn test_applicative() {
        block_on(async {
            let f: ConcreteFutureResult<u32, ()> = pure(3u32);
            assert_eq!(f.await, Ok(3));
            let f: ConcreteFutureResult<Result<&str, ()>, ()> = pure(Ok("test"));
            assert_eq!(f.await, Ok(Ok("test")));
        });
    }

    #[test]
    fn test_functor() {
        block_on(async {
            let f: ConcreteFutureResult<u32, ()> = pure(3u32);
            let f = fmap(f, |i| format!("{} strings", i));
            assert_eq!(f.await, Ok("3 strings".to_string()));
        });

        block_on(async {
            let f: ConcreteFutureResult<u32, ()> = empty();
            let f = fmap(f, |i| format!("{} strings", i));
            assert_eq!(f.await,  Ok("0 strings".to_string()));
        });
    }

    #[test]
    fn test_monad() {
        block_on(async {
            let f: ConcreteFutureResult<u32, ()> = pure(3u32);
            let f2 = flat_map(f, |i| {
                ConcreteFutureResult::new(lazy(move |_| Ok(format!("{} strings", i))))
            });
            assert_eq!(f2.await, Ok("3 strings".to_string()));
        });

        block_on(async {
            let f: ConcreteFutureResult<u32, ()> = pure(3u32);
            let fr = fold(f,
                          10u32,
                          |y, x| y + x);
            assert_eq!(fr.await, Ok(13));
        });

        block_on(async {
            let fs: Vec<ConcreteFutureResult<u32, ()>> = vec![
                pure(3),
                ConcreteFutureResult::new(ready(Ok(10u32))),
                ConcreteFutureResult::new(lazy(|_| Ok(4u32)))
            ];
            let fr = vfold(fs,
                           0u32,
                           |y, x| y + x);
            assert_eq!(fr.await, Ok(17));
        });

        block_on(async {
            let f: ConcreteFutureResult<u32, &'static str> = pure(3u32);
            let f: ConcreteFutureResult<u32, &'static str> = flat_map(f, |i| {
                ConcreteFutureResult::new(lazy(move |_| Err("Good error")))
            });
            let f = flat_map(f, |i| {
                ConcreteFutureResult::new(lazy(move |_| Ok(format!("Shouldn't run this"))))
            });
            assert_eq!(f.await, Err("Good error"));
        });
    }

    #[test]
    fn test_monad_error() {
        block_on(async {
            let f: ConcreteFutureResult<u32, u32> = pure(3u32);
            let f: ConcreteFutureResult<String, u32> = flat_map(f, |i| match i % 2 {
                0 => pure("Good".to_string()),
                _ => raise_error(i)
            });
            assert_eq!(f.await, Err(3));
        });

        block_on(async {
            let f: ConcreteFutureResult<u32, u32> = pure(3u32);
            let f: ConcreteFutureResult<String, u32> = flat_map(f, |i| match i % 2 {
                0 => pure("Good".to_string()),
                _ => raise_error(i)
            });
            let f: ConcreteFutureResult<String, u32> = handle_error(f, |e| pure(format!("{}", e)));

            assert_eq!(f.await, Ok("3".to_string()));
        });

        let f: ConcreteFutureResult<u32, u32> = pure(3u32);
        let f: ConcreteFutureResult<String, u32> = flat_map(f, |i| match i % 2 {
            0 => pure("Good".to_string()),
            _ => raise_error(i)
        });
        let r = attempt(f);

        assert_eq!(r, Err(3));
    }

    #[test]
    fn test_product() {
        block_on(async {
            let f1: ConcreteFutureResult<u32, ()> = pure(3u32);
            let f2: ConcreteFutureResult<&str, ()> = pure("strings");
            let f = product(f1, f2);
            assert_eq!(f.await, Ok((3, "strings")));
        });
    }

    #[test]
    fn test_traverse() {
        block_on(async {
            let fs: Vec<u32> = vec![3, 10, 4];
            let fr = traverse(fs,
                              |x| ConcreteFutureResult::<u32, ()>::new(ready(Ok(x + 5))));
            assert_eq!(fr.await, Ok(vec![8, 15, 9]));
        });

        block_on(async {
            let fs: Vec<u32> = vec![3, 10, 4];
            let fr = traverse(fs,
                              |x| ConcreteFutureResult::<u32, ()>::new(match x % 2 {
                                  0 => ready(Ok(x + 5)),
                                  1 => ready(Err(())),
                                  _ => unreachable!()
                              }));
            assert_eq!(fr.await, Err(()));
        });
    }
}
