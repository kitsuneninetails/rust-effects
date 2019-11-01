///// The IO Monad
/////
///// The IO Monad in Rust encapsulates a future, and is very similar to a future, except that it
///// implements `SyncT`, which allows an IO to be created with a function which will have delayed
///// execution (via a `lazy` future).  Any type wrapped by an IO must implement `Send` and `Sync`
///// in order to be dispatched to the execution context for the future.
/////
///// IO Behaviors:
/////
///// Semigroup
/////     `combine(IO(X), IO(Y)) => IO(combine(X, Y))`
///// Monoid
/////     `empty() => IO(T1::default())` // uses `ready` Future
/////     Note: This returns a valid IO of the default value of the IO's type.
///// Applicative
/////     `pure(X) => IO(X)` // uses `ready` Future
/////     Note: This is greedy and will perform any function given to come up with a value before
/////     creating the IO!
///// Functor
/////     `fmap(IO(X), fn T1 -> T2) => IO(fn(X))`
/////     Note: This is lazy and will perform the function when the IO.`await` is called
///// Functor2
/////     `fmap2(IO(X), IO(Y), fn T1 T2 -> T3) => IO(fn(X, Y))`
/////     Note: This is lazy and will perform the function when the IO.`await` is called
///// Monad
/////     `flat_map(IO(X), fn T1 -> IO<T2>) => IO(*fn(X))` if fn(X) returns Some(Y)
/////     Note: This is lazy and will perform the function when the IO.`await` is called.
/////     Also, this can return a different IO type (Ready vs. Lazy vs. AndThen vs. Map, etc.)
///// Foldable
/////     `fold(IO(X), init, fn TI T1 -> TI) => IO(fn(init, X))`
/////     Note: To preserve the 'IO-ness' of the result, it is essentially the same as a `fmap.`
///// Productable -
/////     `product(IO(X), IO(Y)) => IO((X, Y))`
///// Traverse
/////     `Not implemented`
///// SyncT
/////     `suspend(fn () -> T1) => IO(fn())`
/////     Note: This is lazy and will perform the function when the IO.`await` is called.
//
//use crate::prelude::*;
//use crate::futures::future::lazy;
//use crate::futures::FutureExt;
//use futures::{Future, Poll};
//use std::pin::Pin;
//use futures::task::Context;
//use futures::executor::block_on;
//
//use crate::*;
//use std::marker::PhantomData;
//use std::{fs, io};
//
//pub struct IOResult<'a, X>(IO<'a, io::Result<X>>);
//
//impl<'a, X> F<X> for IOResult<'a, X> {}
//
//semigroup_effect! { S, IOResult, IoResultEffect }
//monoid_effect! { S, IOResult, IoResultEffect }
//applicative_effect! { S, IOResult, IoResultEffect }
//functor_effect! { S, IOResult, IoResultEffect }
//functor2_effect! { S, IOResult, IoResultEffect }
//monad_effect! { S, IOResult, IoResultEffect }
//foldable_effect! { S, IOResult, IoResultEffect }
//productable_effect! { S, IOResult, IoResultEffect }
//synct_effect! { S, IOResult, IoResultEffect }
//
//impl<'a, X> Future for IOResult<'a, X> {
//    type Output = io::Result<X>;
//
//    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//        self.0.fut.inner.poll_unpin(cx)
//    }
//}
//
//#[derive(Clone, Debug)]
//pub struct IoResultEffect<'a, X=(), Y=(), Z=()> {
//    _p: PhantomData<&'a()>,
//    _a: PhantomData<X>,
//    _b: PhantomData<Y>,
//    _c: PhantomData<Z>
//}
//
//impl<'a, X, Y, Z> IoResultEffect<'a, X, Y, Z> {
//    pub fn apply(_: Z) -> IoResultEffect<'a, X, Y, Z> {
//        IoResultEffect {
//            _p: PhantomData,
//            _a: PhantomData,
//            _b: PhantomData,
//            _c: PhantomData
//        }
//    }
//
//    fn combine_futures<X1, X2, R, Fn>(a: IOResult<'a, X1>,
//                                      b: IOResult<'a, X2>,
//                                      func: Fn) -> IOResult<'a, R>
//        where
//            X1: 'a + Send + Sync,
//            X2: 'a + Send + Sync,
//            R: 'a + Send + Sync,
//            Fn: 'a + FnOnce(X1, X2) -> R + Send + Sync {
//        let fr = a.0.then(move |i| match i {
//            Ok(b) => {
//                map(move |j| match j {
//                    Ok(c) => {
//                        func(b, c)
//                    },
//                    Err(e_i) => {
//
//                    }
//                })
//            },
//            Err(e) => {
//
//            }
//        });
//        IoResult(IO::new(fr))
//    }
//}
//
//impl<'a, X, Y, Z> Effect for IoResultEffect<'a, X, Y, Z> {}
//
//impl<'a, X: 'a + Default + Sync + Send, Y, Z> Monoid<IO<'a, X>> for IoResultEffect<'a, X, Y, Z> {
//    fn empty() -> IO<'a, X> {
//        IO::new(ready(io::Error::new(io::ErrorKind::Other, "empty IO error")))
//    }
//}
//
//impl<'a, X1, X2, R> Semigroup<IO<'a, X1>, IO<'a, X2>, IO<'a, R>> for IoResultEffect<'a, X1, X2, R>
//    where
//        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
//        X2: 'a + Send + Sync,
//        R: 'a + Send + Sync {
//    fn combine(a: IO<'a, X1>,
//               b: IO<'a, X2>) -> IO<'a, R> {
//        Self::combine_futures(a, b, combine)
//    }
//}
//
//impl <'a, X, Y, Z> SemigroupInner<'a, IO<'a, X>, X> for IoResultEffect<'a, X, Y, Z>
//    where
//        X: 'a + Send + Sync {
//    fn combine_inner<TO>(a: IO<'a, X>, b: IO<'a, X>) -> IO<'a, X>
//        where
//            TO: 'a + Semigroup<X, X, X> {
//        Self::combine_futures(a, b, TO::combine)
//    }
//}
//
//impl<'a, X, Y, Z> Functor<'a> for IoResultEffect<'a, X, Y, Z>
//    where
//        X: 'a + Send + Sync,
//        Y: 'a + Send + Sync {
//    type X = X;
//    type Y = Y;
//    type FX = IO<'a, Self::X>;
//    type FY = IO<'a, Self::Y>;
//    fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
//        IO::new(FutureEffect::<X, Y, Z>::fmap(f.fut, func))
//    }
//}
//
//impl<'a, X: 'a + Send + Sync, Y: 'a + Send + Sync, Z> Applicative<'a> for IoResultEffect<'a, X, Y, Z> {
//    fn pure(x: X) -> Self::FX {
//        IO::new(FutureEffect::<X, Y, Z>::pure(x))
//    }
//}
//
//impl<'a, X, Y, Z> Functor2<'a> for IoResultEffect<'a, X, Y, Z>
//    where
//        X: 'a + Send + Sync,
//        Y: 'a + Send + Sync,
//        Z: 'a + Send + Sync {
//    type Z = Z;
//    type FZ = IO<'a, Z>;
//    fn fmap2(fa: Self::FX,
//             fb: Self::FY,
//             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
//        IO::new(FutureEffect::fmap2(fa.fut, fb.fut, func))
//    }
//}
//
//impl<'a, X, Y, Z> Monad<'a> for IoResultEffect<'a, X, Y, Z>
//    where
//        X: 'a + Send + Sync,
//        Y: 'a + Send + Sync {
//    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//        IO::new(ConcreteFuture::new(f.map(move |x| func(x)).flatten()))
//    }
//}
//
//impl<'a, X, Y, Z> Foldable<'a> for IoResultEffect<'a, X, Y, Z>
//    where
//        X: 'a + Send + Sync,
//        Y: 'a + Send + Sync {
//    type Z = IO<'a, Y>;
//    fn fold(f: Self::FX,
//            init: Self::Y,
//            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Z {
//        IO::new(FutureEffect::<X, Y, Z>::fold(f.fut, init, func))
//    }
//}
//
//impl<'a, X: Clone, Y: Clone, Z> Productable<'a> for IoResultEffect<'a, X, Y, Z>
//    where
//        X: 'a + Send + Sync,
//        Y: 'a + Send + Sync {
//    type FXY = IO<'a, (X, Y)>;
//    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
//        IO::new(FutureEffect::<X, Y, Z>::product(fa.fut, fb.fut))
//    }
//}
//
//
//impl<'a, X, Z> SyncT<'a> for IoResultEffect<'a, X, X, Z>
//    where
//        X: 'a + Send + Sync {
//    fn suspend(thunk: impl Fn() -> Self::FX + 'a + Send + Sync) -> Self::FX {
//        let x = IO::apply(|| ());
//        IoResultEffect::<(), X, ()>::flat_map(x, move |_| thunk())
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_io() {
//        let t = io! ({
//            println!("Hello");
//            println!("World");
//            4
//        });
//        assert_eq!(4, t.run_sync());
//    }
//
//    #[test]
//    fn test_sync() {
//        let func = || {
//            println!("Hello");
//            println!("World");
//            4
//        };
//        let t: IO<i32> = delay(func);
//        assert_eq!(4, t.run_sync());
//
//        let func = || {
//            println!("Hello");
//            println!("World");
//            pure(4)
//        };
//        let t: IO<i32> = suspend(func);
//        assert_eq!(4, t.run_sync());
//    }
//}
