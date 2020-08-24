// /// Future (Result-aware) Typeclass Behaviors
// ///
// /// The Future type is a `Result` which allows for error types to be propagated, and for
// /// `MonadError` to be implemented.
// ///
// /// Note: Any type wrapped by Future must implement `Send` and `Sync` in order to be
// /// dispatched to the execution context.
// ///
// /// Note: Most functions can return a different future type (Ready vs. Lazy vs. AndThen vs. Map,
// /// etc.).  They should all qualify as implementations of Future for the given type, however.
// /// For all rules below the "Future" type is a ConcreteFuture implementaiton, which wraps a
// /// `Future` trait object (but is needed for its `Sized` implementation).
// ///
// /// Semigroup
// ///     `combine(Future(R1), Future(R2)) => Future(combine(R1, R2))`
// ///     Note: R1 and R2 are results, so they combine as results would combine.
// /// Monoid
// ///     `empty() => Future(Ok(X::empty()))`
// ///     Note: This returns a valid future of the empty value of the future's type.  The type
// ///     must have an associated Monoid.
// /// Applicative
// ///     `pure(X) => Future(Ok(X))` // uses `ready` future
// ///     Note: This is greedy and will perform any function given to come up with a value before
// ///     creating the future!
// /// ApplicativeApply
// ///     `apply(Future(Ok(fn X -> X2)), Future(Ok(X))) => Future(Ok(fn(X))) => Future(Ok(X2))`
// ///     `apply(Future(Ok(fn X -> X2)), Future(Err(E))) => Future(Err(E))`
// ///     `apply(Future(Err(E)), Future(Ok(X))) => Future(Err(E))`
// ///     `apply(Future(Err1(E1)), Future(Err(E2))) => Future(Err(E2))`
// ///     Note: This is lazy and will perform the function when the future.`await` is called
// /// Functor
// ///     `fmap(Future(Ok(X)), fn X1 -> X2) => Future(Ok(fn(X))) => Future(Ok(X2))`
// ///     `fmap(Future(Err(E)), fn X1 -> X2) => Future(Err(E))`
// ///     Note: This is lazy and will perform the function when the future.`await` is called
// /// Functor2
// ///     `fmap2(Future(Ok(X)), Future(Ok(Y)), fn X, Y -> Z) => Future(fn(Ok(X, Y))) => Future(Ok(Z))`
// ///     `fmap2(Future(Err(E)), Future(Ok(Y)), fn X, Y -> Z) => Future(fn(Err(E)))`
// ///     `fmap2(Future(Ok(X)), Future(Err(E)), fn X, Y -> Z) => Future(fn(Err(E)))`
// ///     `fmap2(Future(Err(E1)), Future(Err(E2)), fn X, Y -> Z) => Future(fn(Err(E)))`
// ///     Note: This is lazy and will perform the function when the future.`await` is called
// /// Monad
// ///     `flat_map(Future(Ok(X)), fn X -> Future(Ok(Y))) => fn(X) => Future(Ok(Y))`
// ///     `flat_map(Future(Err(E)), fn X -> Future(Ok(Y))) => Future(Err(E))`
// ///     `flat_map(Future(Ok(X)), fn X -> Future(Err(E))) => Future(Err(E))`
// ///     `flat_map(Future(Err(E1)), fn X -> Future(Err(E2))) => Future(Err(E1))`
// ///     Note: This is lazy and will perform the function when the future.`await` is called.
// /// MonadError
// ///     `raise_error(E) => Future(Err(E))`
// ///     `handle_error(Future(Ok(X1)), fn E -> Future(Ok(X2))) -> fn(E) => Future(Ok(X2))`
// ///     `handle_error(Future(Ok(X1)), fn E -> Future(Err(E))) -> Future(Err(E))`
// ///     `handle_error(Future(Err(E)), fn E -> Future(Ok(X2))) -> Future(Err(E))`
// ///     `handle_error(Future(Err(E)), fn E -> Future(Err(E2))) -> Future(Err(E1))`
// ///     `attempt(Future(Ok(X))) -> Ok(X)`
// ///     `attempt(Future(Err(E))) -> Err(E)`
// /// Foldable
// ///     `fold(Future(Ok(X)), Y, fn Y, X -> Y2) => Future(Ok(fn(Y, X))) => Future(Ok(Y2))`
// ///     `fold(Future(Err(E)), Y, fn Y, X -> Y2) => Future(Ok(Y))`
// ///     Note: Y and Y2 are the same type, just possibly two different values. To preserve the
// ///     'future-ness' of the result, it is essentially the same as a `fmap.`
// /// Productable -
// ///     `product(Future(Ok(X)), Future(Ok(Y))) => Future(Ok(X, Y))`
// ///     `product(Future(Err(E)), Future(Ok(Y))) => Future(Err(E))`
// ///     `product(Future(Ok(X)), Future(Err(E)))) => Future(Err(E))`
// ///     `product(Future(Err(E1)), Future(Err(E2))) => Future(Err(E1))`
// /// Traverse
// ///     `Not implemented`
// use super::prelude::*;
// use futures::prelude::*;
// use futures::future::{ready, BoxFuture, FutureExt};
// use futures::Poll;
// use futures::task::Context;
// use std::pin::Pin;
// use std::marker::PhantomData;
// use std::fmt::Debug;
//
// use crate::*;
// use futures::executor::block_on;
//
// pub struct ConcreteFutureResult<'a, X, E> {
//     pub inner: BoxFuture<'a, Result<X, E>>
// }
//
// impl<'a, E, X> ConcreteFutureResult<'a, X, E> {
//     pub fn new<F: 'a + Future<Output=Result<X, E>> + Send>(f: F) -> Self {
//         ConcreteFutureResult {
//             inner: f.boxed()
//         }
//     }
// }
//
// pub fn fut_res<'a, T, E>(f: impl 'a + Send + Future<Output=Result<T, E>>) -> ConcreteFutureResult<'a, T, E> {
//     ConcreteFutureResult::new(f)
// }
//
// impl<X, E> F<X> for dyn Future<Output=Result<X, E>> {}
// impl<'a, E, X> F<X> for ConcreteFutureResult<'a, X, E> {}
//
// impl<'a, E, X> Future for ConcreteFutureResult<'a, X, E> {
//     type Output=Result<X, E>;
//
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         self.inner.poll_unpin(cx)
//     }
// }
//
// #[derive(Clone, Debug)]
// pub struct FutureResultSemigroup {}
//
// #[derive(Clone, Debug)]
// pub struct FutureResultMonad<'a, E=(), X=(), Y=(), Z=()> {
//     _p: PhantomData<&'a()>,
//     _a: PhantomData<X>,
//     _b: PhantomData<Y>,
//     _c: PhantomData<Z>,
//     _e: PhantomData<E>,
// }
//
// impl<'a, E, X, Y, Z> FutureResultMonad<'a, E, X, Y, Z> {
//     pub fn new(_: Z) -> Self {
//         FutureResultMonad {
//             _p: PhantomData,
//             _a: PhantomData,
//             _b: PhantomData,
//             _c: PhantomData,
//             _e: PhantomData,
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! future_result_semigroup {
//     () => (FutureResultSemigroup {})
// }
// #[macro_export]
// macro_rules! future_result_monad {
//     () => (FutureResultMonad::new(()))
// }
//
// impl Effect for FutureResultSemigroup {}
// impl<'a, E, X, Y, Z> Effect for FutureResultMonad<'a, E, X, Y, Z> {}
//
// impl<'a, E, X> Monoid<ConcreteFutureResult<'a, X, E>> for FutureResultSemigroup
//     where
//         E: 'a + Sync + Send {
//     fn empty<MX>() -> ConcreteFutureResult<'a, X, E>
//         where
//             MX: 'a + Monoid<X> + Sync + Send {
//         ConcreteFutureResult::new(ready(Ok(MX::empty())))
//     }
// }
//
// impl<'a, X1, X2, R, E> Semigroup<
//     ConcreteFutureResult<'a, X1, E>,
//     ConcreteFutureResult<'a, X2, E>,
//     ConcreteFutureResult<'a, R, E>> for FutureResultSemigroup
//     where
//         X1: 'a + Send + Sync,
//         X2: 'a + Send + Sync,
//         R: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug  {
//     fn combine<MX>(
//         a: ConcreteFutureResult<'a, X1, E>,
//         b: ConcreteFutureResult<'a, X2, E>
//     ) -> ConcreteFutureResult<'a, R, E>
//         where
//             MX: Semigroup<X1, X2, R> {
//         let fr = a.then(
//             move |a_fut| b.map(
//                 move |b_fut| a_fut.and_then(
//                     |a_in| b_fut.map(
//                         |b_in| MX::combine(a_in, b_in)
//                     )
//                 )
//             )
//         );
//         ConcreteFutureResult::new(fr)
//     }
// }
//
// impl<'a, E, X, Y, Z> Functor<'a> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug  {
//     type X = X;
//     type Y = Y;
//     type FX = ConcreteFutureResult<'a, X, E>;
//     type FY = ConcreteFutureResult<'a, Y, E>;
//     fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send) -> Self::FY {
//         ConcreteFutureResult::new(f.map(move |f_fut| f_fut.map(|x_in| func(x_in))))
//     }
// }
//
// impl<'a, X, Y, Z, E> Applicative<'a> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         Z: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug {
//     fn pure(x: X) -> Self::FX {
//         ConcreteFutureResult::new(ready(Ok(x)))
//     }
// }
//
// impl<'a, E, X, Y, Z, M> ApplicativeApply<'a, M> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         Z: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug,
//         M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
//     type FMapper = ConcreteFutureResult<'a, M, E>;
//     fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
//         ConcreteFutureResult::new(x.map(move |x_fut| func.map(|f_in| x_fut.and_then(|x_in| f_in.map(|f| f(x_in))))).flatten())
//     }
// }
//
// impl<'a, E, X, Y, Z> Functor2<'a> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         Z: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug {
//     type Z = Z;
//     type FZ = ConcreteFutureResult<'a, Z, E>;
//     fn fmap2(fa: Self::FX,
//              fb: Self::FY,
//              func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
//         let fr = fa.then(move |a_fut| fb.map(move |b_fut| a_fut.and_then(|a_in| b_fut.map(|b_in| func(a_in, b_in)))));
//
//         ConcreteFutureResult::<'a, Z, E>::new(fr)
//     }
// }
//
// impl<'a, E, X, Y, Z> Monad<'a> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         Z: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug {
//     fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//         let res = ConcreteFutureResult::new(f.then(move |f_fut| match f_fut {
//             Ok(f_in) => func(f_in),
//             Err(e) => ConcreteFutureResult::new(ready(Err(e)))
//         }));
//         res
//     }
// }
//
// impl<'a, E, X, Y, Z> Foldable<'a> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         Z: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug {
//     type Y2 = ConcreteFutureResult<'a, Y, E>;
//     fn fold(f: Self::FX,
//             init: Self::Y,
//             func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Y2 {
//         ConcreteFutureResult::new(f.map(move |f_fut| f_fut.map(|f_in| func(init, f_in))))
//     }
// }
//
// impl<'a, E, X, Y, Z> MonadError<'a> for FutureResultMonad<'a, E, X, Y, Z>
//     where
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync,
//         Z: 'a + Send + Sync,
//         E: 'a + Send + Sync + Debug {
//     type E=E;
//     fn raise_error(err: Self::E) -> Self::FX {
//         fut_res(ready(Err(err)))
//     }
//
//     fn handle_error(f: Self::FX, recovery: impl 'a + Send + Sync + Fn(Self::E) -> Self::FX) -> Self::FX {
//         fut_res(f.then(move |r| match r {
//             Ok(o) => FutureResultMonad::pure(o),
//             Err(e) => recovery(e)
//         }))
//     }
//
//     fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
//         block_on(async {
//             f.await
//         })
//     }
// }
//
// /// A specialized fold for vectors of Futures which generally have to map and chain the futures
// /// together into one big `Future`, rather than accumulate and combine on the fly.
// //pub fn vfold<'a, X, Y, E>(f: Vec<ConcreteFutureResult<'a, X, E>>,
// //                          init: Y,
// //                          func: impl 'a + Fn(Y, X) -> Y + Send + Sync + Copy) -> ConcreteFutureResult<'a, Y, E>
// //    where
// //        X: 'a + Send + Sync,
// //        Y: 'a + Send + Sync,
// //        E: 'a + Send + Sync + Debug {
// //    VecEffect::<ConcreteFutureResult<X, E>, ConcreteFutureResult<Y, E>>::fold(
// //        f,
// //        FutureResultEffect::<'a, E, Y>::pure(init),
// //        |a, i| ConcreteFutureResult::new(
// //            a.then(
// //                move|a_fut| i.map(
// //                    move |i_fut| a_fut.and_then(|a_in| i_fut.map(|i_in| func(a_in, i_in)))
// //                )
// //            )
// //        )
// //    )
// //}
//
// impl<'a, E, X, Y> Productable<'a> for FutureResultMonad<'a, E, X, Y, (X, Y)>
//     where
//         E: 'a + Debug + Send + Sync,
//         X: 'a + Send + Sync,
//         Y: 'a + Send + Sync {}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use futures::executor::block_on;
//     use futures::future::lazy;
//
//     #[test]
//     fn test_semigroup() {
//         block_on(async {
//             let f1: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(1);
//             let f2: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(2);
//             let fr = FutureResultMonad::combine(f1, f2);
//             assert_eq!(fr.await, Ok(3));
//
//             let f1: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(3);
//             let f2: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(5);
//             let fr = FutureResultMonad::combine_inner::<IntMulMonoid>(f1, f2);
//             assert_eq!(fr.await, Ok(15));
//         });
//     }
//
//     #[test]
//     fn test_monoid() {
//         block_on(async {
//             let f: ConcreteFutureResult<u32, ()> = FutureResultMonad::empty();
//             assert_eq!(f.await, Ok(0));
//         });
//     }
//
//     #[test]
//     fn test_applicative() {
//         block_on(async {
//             let f: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(3u32);
//             assert_eq!(f.await, Ok(3));
//             let f: ConcreteFutureResult<Result<&str, ()>, ()> = FutureResultMonad::pure(Ok("test"));
//             assert_eq!(f.await, Ok(Ok("test")));
//         });
//     }
//
//     #[test]
//     fn test_apply() {
//         block_on(async {
//             let f: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(3u32);
//             let func: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(move |x| format!("{} strings", x));
//             let f = FutureResultMonad::apply(func, f);
//             assert_eq!(f.await, Ok("3 strings".to_string()));
//
//             let f1: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(3u32);
//             let f2: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(6);
//             let func: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(|x| move |y| x + y);
//             let f = FutureResultMonad::apply(func, f1);
//             let f: ConcreteFutureResult<_, ()> = FutureResultMonad::apply(f, f2);
//             assert_eq!(f.await, Ok(9));
//
//             let f1: ConcreteFutureResult<i32, ()> = FutureResultMonad::empty();
//             let f2: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(6);
//             let func: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(|x| move |y| x + y);
//             let f = FutureResultMonad::apply(func, f1);
//             let f: ConcreteFutureResult<_, ()> = FutureResultMonad::apply(f, f2);
//             assert_eq!(f.await, Ok(6));
//
//             let f1: ConcreteFutureResult<i32, ()> = FutureResultMonad::raise_error(());
//             let f2: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(6);
//             let func: ConcreteFutureResult<_, ()> = FutureResultMonad::pure(|x| move |y| x + y);
//             let f = FutureResultMonad::apply(func, f1);
//             let f: ConcreteFutureResult<_, ()> = FutureResultMonad::apply(f, f2);
//             assert_eq!(f.await, Err(()));
//         });
//     }
//
//     #[test]
//     fn test_functor() {
//         block_on(async {
//             let f: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(3u32);
//             let f = FutureResultMonad::fmap(f, |i| format!("{} strings", i));
//             assert_eq!(f.await, Ok("3 strings".to_string()));
//         });
//
//         block_on(async {
//             let f: ConcreteFutureResult<u32, ()> = FutureResultMonad::empty();
//             let f = FutureResultMonad::map(f, |i| format!("{} strings", i));
//             assert_eq!(f.await,  Ok("0 strings".to_string()));
//         });
//     }
//
//     #[test]
//     fn test_monad() {
//         block_on(async {
//             let f: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(3u32);
//             let f2 = FutureResultMonad::flat_map(f, |i| {
//                 ConcreteFutureResult::new(lazy(move |_| Ok(format!("{} strings", i))))
//             });
//             assert_eq!(f2.await, Ok("3 strings".to_string()));
//         });
//
//         block_on(async {
//             let f: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(3u32);
//             let fr = FutureResultMonad::fold(f,
//                                              10u32,
//                                              |y, x| y + x);
//             assert_eq!(fr.await, Ok(13));
//         });
//
// //        block_on(async {
// //            let fs: Vec<ConcreteFutureResult<u32, ()>> = vec![
// //                pure(3),
// //                ConcreteFutureResult::new(ready(Ok(10u32))),
// //                ConcreteFutureResult::new(lazy(|_| Ok(4u32)))
// //            ];
// //            let fr = vfold(fs,
// //                           0u32,
// //                           |y, x| y + x);
// //            assert_eq!(fr.await, Ok(17));
// //        });
//
//         block_on(async {
//             let f: ConcreteFutureResult<u32, &'static str> = FutureResultMonad::pure(3u32);
//             let f: ConcreteFutureResult<u32, &'static str> = FutureResultMonad::flat_map(f, |_| {
//                 ConcreteFutureResult::new(lazy(move |_| Err("Good error")))
//             });
//             let f = FutureResultMonad::flat_map(f, |_| {
//                 ConcreteFutureResult::new(lazy(move |_| Ok(format!("Shouldn't run this"))))
//             });
//             assert_eq!(f.await, Err("Good error"));
//         });
//     }
//
//     #[test]
//     fn test_monad_error() {
//         block_on(async {
//             let f: ConcreteFutureResult<u32, u32> = FutureResultMonad::pure(3u32);
//             let f: ConcreteFutureResult<String, u32> = FutureResultMonad::flat_map(f, |i| match i % 2 {
//                 0 => FutureResultMonad::pure("Good".to_string()),
//                 _ => FutureResultMonad::raise_error(i)
//             });
//             assert_eq!(f.await, Err(3));
//         });
//
//         block_on(async {
//             let f: ConcreteFutureResult<u32, u32> = FutureResultMonad::pure(3u32);
//             let f: ConcreteFutureResult<String, u32> = FutureResultMonad::flat_map(f, |i| match i % 2 {
//                 0 => FutureResultMonad::pure("Good".to_string()),
//                 _ => FutureResultMonad::raise_error(i)
//             });
//             let f: ConcreteFutureResult<String, u32> = FutureResultMonad::handle_error(
//                 f,
//                 |e| FutureResultMonad::pure(format!("{}", e))
//             );
//
//             assert_eq!(f.await, Ok("3".to_string()));
//         });
//
//         let f: ConcreteFutureResult<u32, u32> = FutureResultMonad::pure(3u32);
//         let f: ConcreteFutureResult<String, u32> = FutureResultMonad::flat_map(f, |i| match i % 2 {
//             0 => FutureResultMonad::pure("Good".to_string()),
//             _ => FutureResultMonad::raise_error(i)
//         });
//         let r = FutureResultMonad::attempt(f);
//
//         assert_eq!(r, Err(3));
//     }
//
//     #[test]
//     fn test_product() {
//         block_on(async {
//             let f1: ConcreteFutureResult<u32, ()> = FutureResultMonad::pure(3u32);
//             let f2: ConcreteFutureResult<&str, ()> = FutureResultMonad::pure("strings");
//             let f = FutureResultMonad::product(f1, f2);
//             assert_eq!(f.await, Ok((3, "strings")));
//         });
//     }
//
// //    #[test]
// //    fn test_traverse() {
// //        block_on(async {
// //            let fs: Vec<u32> = vec![3, 10, 4];
// //            let fr = traverse(fs,
// //                              |x| ConcreteFutureResult::<u32, ()>::new(ready(Ok(x + 5))));
// //            assert_eq!(fr.await, Ok(vec![8, 15, 9]));
// //        });
// //
// //        block_on(async {
// //            let fs: Vec<u32> = vec![3, 10, 4];
// //            let fr = traverse(fs,
// //                              |x| ConcreteFutureResult::<u32, ()>::new(match x % 2 {
// //                                  0 => ready(Ok(x + 5)),
// //                                  1 => ready(Err(())),
// //                                  _ => unreachable!()
// //                              }));
// //            assert_eq!(fr.await, Err(()));
// //        });
// //    }
// }
