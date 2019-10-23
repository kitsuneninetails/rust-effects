use crate::prelude::*;
use crate::futures::future::lazy;
use crate::futures::FutureExt;
use futures::{Future, Poll};
use std::pin::Pin;
use futures::task::Context;
use futures::executor::block_on;

use crate::*;
use std::marker::PhantomData;

pub struct IO<'a, X> {
    fut: ConcreteFuture<'a, X>,
}
impl<'a, X> IO<'a, X> {
    pub fn apply(func: impl 'a + FnOnce() -> X + Send + Sync) -> IO<'a, X> {
        IO {
            fut: fut(lazy(|_| func()))
        }
    }

    pub fn new(fut: ConcreteFuture<'a, X>) -> IO<'a, X> {
        IO {
            fut
        }
    }

//    pub fn read_line() -> IO<'a, String> {
//
//    }

    pub fn run_sync(self) -> X {
        block_on(async {
            self.await
        })
    }
}

impl<'a, X> F<X> for IO<'a, X> {}

semigroup_effect! { S, IO, IoEffect }
monoid_effect! { S, IO, IoEffect }
applicative_effect! { S, IO, IoEffect }
functor_effect! { S, IO, IoEffect }
functor2_effect! { S, IO, IoEffect }
monad_effect! { S, IO, IoEffect }
foldable_effect! { S, IO, IoEffect }
productable_effect! { S, IO, IoEffect }

impl<'a, X> Future for IO<'a, X> {
    type Output = X;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.inner.poll_unpin(cx)
    }
}

#[derive(Clone, Debug)]
pub struct IoEffect<'a, X=(), Y=(), Z=()> {
    _p: PhantomData<&'a()>,
    _a: PhantomData<X>,
    _b: PhantomData<Y>,
    _c: PhantomData<Z>
}

impl<'a, X, Y, Z> IoEffect<'a, X, Y, Z> {
    pub fn apply(_: Z) -> IoEffect<'a, X, Y, Z> {
        IoEffect {
            _p: PhantomData,
            _a: PhantomData,
            _b: PhantomData,
            _c: PhantomData
        }
    }

    fn combine_futures<X1, X2, R, Fn>(a: IO<'a, X1>,
                                      b: IO<'a, X2>,
                                      func: Fn) -> IO<'a, R>
        where
            X1: 'a + Send + Sync,
            X2: 'a + Send + Sync,
            R: 'a + Send + Sync,
            Fn: 'a + FnOnce(X1, X2) -> R + Send + Sync {
        IO::new(FutureEffect::<X, Y, Z>::combine_futures(a.fut, b.fut, func))
    }
}

impl<'a, X, Y, Z> Effect for IoEffect<'a, X, Y, Z> {}

impl<'a, X: 'a + Default + Send, Y, Z> Monoid<IO<'a, X>> for IoEffect<'a, X, Y, Z> {
    fn empty() -> IO<'a, X> {
        IO::new(FutureEffect::<X, Y, Z>::empty())
    }
}

impl<'a, X1, X2, R> Semigroup<IO<'a, X1>, IO<'a, X2>, IO<'a, R>> for IoEffect<'a, X1, X2, R>
    where
        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync {
    fn combine(a: IO<'a, X1>,
               b: IO<'a, X2>) -> IO<'a, R> {
        Self::combine_futures(a, b, combine)
    }
}

impl <'a, X, Y, Z> SemigroupInner<'a, IO<'a, X>, X> for IoEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync {
    fn combine_inner<TO>(a: IO<'a, X>, b: IO<'a, X>) -> IO<'a, X>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_futures(a, b, TO::combine)
    }
}

impl<'a, X, Y, Z> Functor<'a> for IoEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type X = X;
    type Y = Y;
    type FX = IO<'a, Self::X>;
    type FY = IO<'a, Self::Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
        IO::new(FutureEffect::<X, Y, Z>::fmap(f.fut, func))
    }
}

impl<'a, X: 'a + Send + Sync, Y: 'a + Send + Sync, Z> Applicative<'a> for IoEffect<'a, X, Y, Z> {
    fn pure(x: X) -> Self::FX {
        IO::new(FutureEffect::<X, Y, Z>::pure(x))
    }
}

impl<'a, X, Y, Z> Functor2<'a> for IoEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    type Z = Z;
    type FZ = IO<'a, Z>;
    fn fmap2(fa: Self::FX,
             fb: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        IO::new(FutureEffect::fmap2(fa.fut, fb.fut, func))
    }
}

impl<'a, X, Y, Z> Monad<'a> for IoEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        IO::new(ConcreteFuture::new(f.map(move |x| func(x)).flatten()))
    }
}

impl<'a, X, Y, Z> Foldable<'a> for IoEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type Z = IO<'a, Y>;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Z {
        IO::new(FutureEffect::<X, Y, Z>::fold(f.fut, init, func))
    }
}

impl<'a, X: Clone, Y: Clone, Z> Productable<'a> for IoEffect<'a, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FXY = IO<'a, (X, Y)>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
        IO::new(FutureEffect::<X, Y, Z>::product(fa.fut, fb.fut))
    }
}

//impl<'a, X, Y, Z> SyncT<'a> for IO<'a, X, Y, Z> {
//
//}

#[macro_export]
macro_rules! io {
    ($m:expr) => (IO::apply(move || $m ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_io() {
        let t = io! ({
            println!("Hello");
            println!("World");
            4
        });
        assert_eq!(4, t.run_sync());
    }
}
