use crate::prelude::*;
use crate::futures::future::lazy;
use crate::futures::FutureExt;
use futures::{Future, Poll};
use std::pin::Pin;
use futures::task::Context;
use futures::executor::block_on;

#[macro_use] use crate::*;
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
pub struct IoEffect<'a> {
    _p: PhantomData<&'a()>
}

impl<'a> IoEffect<'a> {
    pub fn apply() -> IoEffect<'a> {
        IoEffect {
            _p: PhantomData,
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
        IO::new(FutureEffect::combine_futures(a.fut, b.fut, func))
    }
}

impl<'a> Effect for IoEffect<'a> {}

impl<'a, X: 'a + Default + Send> Monoid<IO<'a, X>> for IoEffect<'a> {
    fn empty() -> IO<'a, X> {
        IO::new(FutureEffect::empty())
    }
}

impl<'a, X1, X2, R> Semigroup<IO<'a, X1>, IO<'a, X2>, IO<'a, R>> for IoEffect<'a>
    where
        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync {
    fn combine(a: IO<'a, X1>,
               b: IO<'a, X2>) -> IO<'a, R> {
        Self::combine_futures(a, b, combine)
    }
}

impl <'a, X> SemigroupInner<'a, IO<'a, X>, X> for IoEffect<'a>
    where
        X: 'a + Send + Sync {
    fn combine_inner<TO>(a: IO<'a, X>, b: IO<'a, X>) -> IO<'a, X>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_futures(a, b, TO::combine)
    }
}

impl<'a, X: 'a + Send + Sync> Applicative<X> for IoEffect<'a> {
    type FX = IO<'a, X>;
    fn pure(x: X) -> Self::FX {
        IO::new(FutureEffect::pure(x))
    }
}

impl<'a, X, Y> Functor<
    'a,
    X,
    Y> for IoEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = IO<'a, X>;
    type FY = IO<'a, Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
        IO::new(FutureEffect::fmap(f.fut, func))
    }
}

impl<'a, X, Y, Z> Functor2<'a, X, Y, Z> for IoEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync {
    type FX = IO<'a, X>;
    type FY = IO<'a, Y>;
    type FZ = IO<'a, Z>;
    fn fmap2(fa: Self::FX, fb: Self::FY, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Self::FZ {
        IO::new(FutureEffect::fmap2(fa.fut, fb.fut, func))
    }
}

impl<'a, X, Y> Monad<'a, X, Y> for IoEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = IO<'a, X>;
    type FY = IO<'a, Y>;

    fn flat_map(f: Self::FX, func: impl 'a + Fn(X) -> Self::FY + Send + Sync) -> Self::FY {
        IO::new(ConcreteFuture::new(f.map(move |x| func(x)).flatten()))
    }
}

impl<'a, X, Y> Foldable<'a, X, Y, IO<'a, Y>> for IoEffect<'a>
    where
        X: 'a + Send,
        Y: 'a + Send {
    type FX = IO<'a, X>;
    fn fold(f: Self::FX,
            init: Y,
            func: impl 'a + Fn(Y, X) -> Y + Send + Sync)
            -> IO<'a, Y> {
        IO::new(FutureEffect::fold(f.fut, init, func))
    }
}

impl<'a, X: Clone, Y: Clone> Productable<X, Y> for IoEffect<'a>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync {
    type FX = IO<'a, X>;
    type FY = IO<'a, Y>;
    type FXY = IO<'a, (X, Y)>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
        IO::new(FutureEffect::product(fa.fut, fb.fut))
    }
}

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
