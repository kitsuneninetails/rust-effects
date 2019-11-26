/// The IO Monad
///
/// The IO Monad in Rust encapsulates a future, and is very similar to a future, except that it
/// implements `SyncT`, which allows an IO to be created with a function which will have delayed
/// execution (via a `lazy` future).  Any type wrapped by an IO must implement `Send` and `Sync`
/// in order to be dispatched to the execution context for the future.
///
/// IO Behaviors:
///
/// Semigroup
///     `combine(IO(X), IO(Y)) => IO(combine(X, Y))`
/// Monoid
///     `empty() => IO(T1::default())` // uses `ready` Future
///     Note: This returns a valid IO of the default value of the IO's type.
/// Applicative
///     `pure(X) => IO(X)` // uses `ready` Future
///     Note: This is greedy and will perform any function given to come up with a value before
///     creating the IO!
/// Functor
///     `fmap(IO(X), fn T1 -> T2) => IO(fn(X))`
///     Note: This is lazy and will perform the function when the IO.`await` is called
/// Functor2
///     `fmap2(IO(X), IO(Y), fn T1 T2 -> T3) => IO(fn(X, Y))`
///     Note: This is lazy and will perform the function when the IO.`await` is called
/// Monad
///     `flat_map(IO(X), fn T1 -> IO<T2>) => IO(*fn(X))` if fn(X) returns Some(Y)
///     Note: This is lazy and will perform the function when the IO.`await` is called.
///     Also, this can return a different IO type (Ready vs. Lazy vs. AndThen vs. Map, etc.)
/// Foldable
///     `fold(IO(X), init, fn TI T1 -> TI) => IO(fn(init, X))`
///     Note: To preserve the 'IO-ness' of the result, it is essentially the same as a `fmap.`
/// Productable -
///     `product(IO(X), IO(Y)) => IO((X, Y))`
/// Traverse
///     `Not implemented`
/// SyncT
///     `suspend(fn () -> T1) => IO(fn())`
///     Note: This is lazy and will perform the function when the IO.`await` is called.

use crate::prelude::*;
use crate::futures::FutureExt;
use futures::{Future, Poll, future::lazy};
use std::pin::Pin;
use std::fmt::Debug;

use futures::task::Context;
use futures::executor::block_on;

use crate::*;
use std::marker::PhantomData;
//use std::{fs, io};

pub struct IO<'a, X, E: Debug + Send + Sync> {
    pub fut: ConcreteFutureResult<'a, X, E>,
}
impl<'a, X, E> IO<'a, X, E>
    where
        X: 'a + Send + Sync,
        E: 'a + Debug + Send + Sync {
    pub fn apply(func: impl 'a + FnOnce() -> X + Send + Sync) -> IO<'a, X, E> {
        IO {
            fut: fut_res(lazy(move |_| Ok(func())))
        }
    }

    pub fn lazy(func: impl 'a + FnOnce() -> IO<'a, X, E> + Send + Sync) -> IO<'a, X, E> {
        IO {
            fut: func().fut
        }
    }

    pub fn new(fut: ConcreteFutureResult<'a, X, E>) -> IO<'a, X, E> {
        IO {
            fut
        }
    }

//    pub fn get_file(path: String) -> IO<'a, io::Result<String>> {
//        delay(move || fs::read_to_string(path.clone()))
//    }
//
//    pub fn get_line() -> IO<'a, io::Result<String>> {
//        delay(move || {
//            let mut output = String::new();
//            io::stdin().read_line(&mut output)
//                .map(|_| output)
//        })
//    }
//
//    pub fn put_to_file(path: String, contents: String) -> IO<'a, io::Result<()>> {
//        delay(move || {
//            fs::write(path.clone(), contents.clone())
//        })
//    }

    pub fn run_sync(self) -> X {
        block_on(async {
            self.await.unwrap()
        })
    }
//
//    pub fn run_async(self) -> X {
//        let ex = match ThreadPool::new() {
//            Ok(tp) => {
//                tp.run()
//            },
//            Err(e) => raise_error(e)
//        }
//
//    }

}

impl<'a, X, E> F<X> for IO<'a, X, E>
    where
        X: 'a + Send + Sync,
        E: 'a + Debug + Send + Sync {}

semigroup_effect! { 2S, IO, IoEffect }
monoid_effect! { 2S, IO, IoEffect }
applicative_effect! { 2S, IO, IoEffect }
functor_effect! { 2S, IO, IoEffect }
functor2_effect! { 2S, IO, IoEffect }
monad_effect! { 2S, IO, IoEffect }
monaderror_effect! { 2S, IO, IoEffect }
foldable_effect! { 2S, IO, IoEffect }
productable_effect! { 2S, IO, IoEffect }
synct_effect! { 2S, IO, IoEffect }

impl<'a, X, E> Future for IO<'a, X, E>
    where
        X: 'a + Send + Sync,
        E: 'a + Debug + Send + Sync {
    type Output = Result<X, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.inner.poll_unpin(cx)
    }
}

#[derive(Clone, Debug)]
pub struct IoEffect<'a, E, X=(), Y=(), Z=()>
    where
        X: 'a + Send + Sync,
        E: 'a + Debug + Send + Sync {
    _p: PhantomData<&'a()>,
    _a: PhantomData<X>,
    _b: PhantomData<Y>,
    _c: PhantomData<Z>,
    _e: PhantomData<E>
}

impl<'a, E, X, Y, Z> IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    pub fn apply(_: Z) -> IoEffect<'a, E, X, Y, Z> {
        IoEffect {
            _p: PhantomData,
            _a: PhantomData,
            _b: PhantomData,
            _c: PhantomData,
            _e: PhantomData
        }
    }

    fn combine_futures<X1, X2, R, Fn>(a: IO<'a, X1, E>,
                                      b: IO<'a, X2, E>,
                                      func: Fn) -> IO<'a, R, E>
        where
            X1: 'a + Send + Sync,
            X2: 'a + Send + Sync,
            R: 'a + Send + Sync,
            Fn: 'a + FnOnce(X1, X2) -> R + Send + Sync {
        IO::new(FutureResultEffect::<E, R, Y, Z>::combine_futures(a.fut, b.fut, func))
    }
}

#[macro_export]
macro_rules! io_monad {
    () => (IoEffect::apply(()))
}

impl<'a, E, X, Y, Z> Effect for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        E: 'a + Debug + Send + Sync {}

impl<'a, E: Debug + Send + Sync, X, Y, Z> Monoid<IO<'a, X, E>> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + MonoidEffect<X> + Sync + Send,
        E: 'a + Sync + Send + Debug {
    fn empty() -> IO<'a, X, E> {
        IO::new(FutureResultEffect::<E, X, Y, Z>::empty())
    }
}

impl<'a, X1, X2, R, E: Debug + Send + Sync> Semigroup<IO<'a, X1, E>, IO<'a, X2, E>, IO<'a, R, E>> for IoEffect<'a, E, X1, X2, R>
    where
        X1: SemigroupEffect<X1, X2, R> + 'a + Send + Sync,
        X2: 'a + Send + Sync,
        R: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn combine(a: IO<'a, X1, E>,
               b: IO<'a, X2, E>) -> IO<'a, R, E> {
        Self::combine_futures(a, b, combine)
    }
}

impl <'a, E: Debug + Send + Sync, X, Y, Z> SemigroupInner<'a, IO<'a, X, E>, X> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn combine_inner<TO>(a: IO<'a, X, E>, b: IO<'a, X, E>) -> IO<'a, X, E>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_futures(a, b, TO::combine)
    }
}

impl<'a, E: Debug + Send + Sync, X, Y, Z> Functor<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type X = X;
    type Y = Y;
    type FX = IO<'a, Self::X, E>;
    type FY = IO<'a, Self::Y, E>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
        IO::new(FutureResultEffect::<E, X, Y, Z>::fmap(f.fut, func))
    }
}

impl<'a, E: Debug + Send + Sync, X, Y, Z> Applicative<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn pure(x: X) -> Self::FX {
        IO::new(FutureResultEffect::<E, X, Y, Z>::pure(x))
    }
}

impl<'a, E: Debug + Send + Sync, X, Y, Z> Functor2<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        Z: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type Z = Z;
    type FZ = IO<'a, Z, E>;
    fn fmap2(fa: Self::FX,
             fb: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        IO::new(FutureResultEffect::fmap2(fa.fut, fb.fut, func))
    }
}

impl<'a, E: Debug + Send + Sync, X, Y, Z> Monad<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        IO::new(
            ConcreteFutureResult::new(
                f.then(move |x| match x {
                    Ok(x_in) => func(x_in),
                    Err(e) => raise_error::<IO<Y, E>, Y>(e)
                })
            )
        )
    }
}

impl<'a, E: Debug + Send + Sync, X, Y, Z> Foldable<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type Z = IO<'a, Y, E>;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Z {
        IO::new(FutureResultEffect::<E, X, Y, Z>::fold(f.fut, init, func))
    }
}

impl<'a, E: Debug + Send + Sync, X, Y, Z> MonadError<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type E=E;
    fn raise_error(err: Self::E) -> Self::FX {
        IO::new(FutureResultEffect::<E, X, Y, Z>::raise_error(err))
    }

    fn handle_error(f: Self::FX, recovery: impl 'a + Send + Sync + Fn(Self::E) -> Self::FX) -> Self::FX {
        IO::new(
            ConcreteFutureResult::new(
                f.then(move |x| match x {
                    Ok(x_in) => pure(x_in),
                    Err(e) => recovery(e)
                })
            )
        )
    }

    fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
        block_on(async {
            f.await
        })
    }

}

impl<'a, E: Debug + Send + Sync, X: Clone, Y: Clone, Z> Productable<'a> for IoEffect<'a, E, X, Y, Z>
    where
        X: 'a + Send + Sync,
        Y: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    type FXY = IO<'a, (X, Y), E>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
        IO::new(FutureResultEffect::<E, X, Y, Z>::product(fa.fut, fb.fut))
    }
}


impl<'a, E: Debug + Send + Sync, X, Z> SyncT<'a> for IoEffect<'a, E, X, X, Z>
    where
        X: 'a + Send + Sync,
        E: 'a + Send + Sync + Debug {
    fn suspend(thunk: impl 'a + Fn() -> Self::FX + Send + Sync) -> Self::FX {
       flat_map(IO::apply(|| ()), move |_| thunk())
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
        let t: IO<i32, ()> = io! ({
            println!("Hello");
            println!("World");
            4
        });
        assert_eq!(4, t.run_sync());
    }

    #[test]
    fn test_sync() {
        let func = || {
            println!("Hello");
            println!("World");
            4u32
        };
        let t: IO<u32, ()> = delay(func);
        assert_eq!(4, t.run_sync());

        let func = || {
            println!("Hello");
            println!("World");
            pure(4)
        };
        let t: IO<i32, ()> = suspend(func);
        assert_eq!(4, t.run_sync());
    }

    #[test]
    fn test_errors() {
        let t: IO<i32, u32> = io! ({
            println!("Hello");
            println!("World");
            4
        });
        let t: IO<i32, u32> = flat_map(t, |_| raise_error(2u32));
        let t: IO<i32, u32> = handle_error(t, |_| pure(200));
        assert_eq!(attempt(t), Ok(200));
    }
}
