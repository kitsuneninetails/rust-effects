// /// Result Typeclass Behaviors
// ///
// /// Semigroup
// ///     `combine(Ok(X), Ok(Y)) => Ok(combine(X, Y))`
// ///     `combine(Ok(X), Err(E)) => Err(E)`
// ///     `combine(Err(E), Ok(Y)) => Err(E)`
// ///     `combine(Err(E1), Err(E2)) => Err(E1)`
// /// Monoid
// ///     `empty() => Ok(X::empty())`
// /// Applicative
// ///     `pure(X) => Ok(X)`
// /// ApplicativeApply
// ///     `fmap(Ok(fn X -> Y), Ok(X)) => Ok(fn(X)) => Ok(Y)`
// ///     `fmap(Ok(fn X -> Y), Err(E)) => Err(E)`
// ///     `fmap(Err(E), Ok(X)) => Err(E)`
// ///     `fmap(Err(E1), Err(E2)) => Err(E2)`
// /// Functor
// ///     `fmap(Ok(X), fn X -> Y) => Ok(fn(X)) => Ok(Y)`
// ///     `fmap(Err(E), fn X -> Y) => Err(E)`
// /// Functor2
// ///     `fmap2(Ok(X), Ok(Y), fn X, Y -> Z) => Ok(fn(X, Y))`
// ///     `fmap2(Ok(X), Err(E2), fn X, Y -> Z) => Err(E2)`
// ///     `fmap2(Err(E1), Ok(Y), fn X, Y -> Z) => Err(E1)`
// ///     `fmap2(Err(E1), Err(E2), fn X, Y -> Z) => Err(E1)`
// /// Monad
// ///     `flat_map(Ok(X), fn X -> Ok(Y)) => fn(X) => Ok(Y)`
// ///     `flat_map(Ok(X), fn X -> Err(E)) => fn(X) => Err(E)`
// ///     `flat_map(Err(E), fn X -> Ok(Y)) => Err(E)`
// ///     `flat_map(Err(E1), fn X -> Err(E)) => Err(E1)`
// /// Foldable
// ///     `fold(Ok(X), init, fn Y, X -> Y2) => fn(init, X) => Y2`
// ///     `fold(Err(E), init, fn Y, X -> Y2) => init => Y`
// ///     Note: Y and Y2 are the same type, just possibly two different values.
// /// MonadError
// ///     `raise_error(E) => Err(E)`
// /// Productable -
// ///     `product(Ok(X), Ok(Y)) => Ok((X, Y))`
// ///     `product(Ok(X), Err(E)) => Err(E)`
// ///     `product(Err(E), Ok(Y)) => Err(E)`
// ///     `product(Err(E1), Err(E2)) => Err(E1)`
// /// Traverse
// ///     `Not implemented`
//
// use super::prelude::*;
// use std::marker::PhantomData;
// use std::fmt::Debug;
//
// impl<X, E: Debug> F<X> for Result<X, E> {}
//
// pub struct ResultSemigroup {}
//
// pub struct ResultMonad<E: Debug, X=(), Y=(), Z=()> {
//     _a: PhantomData<X>,
//     _b: PhantomData<Y>,
//     _c: PhantomData<Z>,
//     _p: PhantomData<E>
// }
//
// impl<E: Debug, X, Y, Z> ResultMonad<E, X, Y, Z> {
//     pub fn new(_: Z) -> Self {
//         ResultMonad {
//             _a: PhantomData,
//             _b: PhantomData,
//             _c: PhantomData,
//             _p: PhantomData
//         }
//     }
//
//     fn combine_results<X1, X2, XR, F>(a: Result<X1, E>,
//                                       b: Result<X2, E>,
//                                       func: F) -> Result<XR, E>
//         where
//             F: FnOnce(X1, X2) -> XR {
//         a.and_then(|i| b.map(|j| func(i, j)))
//     }
// }
//
// #[macro_export]
// macro_rules! result_semigroup {
//     () => (ResultSemigroup {})
// }
// macro_rules! result_monad {
//     () => (ResultMonad::new(()))
// }
//
// impl Effect for ResultSemigroup {}
// impl<E: Debug, X, Y, Z> Effect for ResultMonad<E, X, Y, Z>{}
//
// impl<E: Debug, X> Monoid<Result<X, E>> for ResultSemigroup {
//     fn empty<MX: Monoid<X>>() -> Result<X, E> {
//         Ok(MX::empty())
//     }
// }
//
// impl<X, X2, XR, E: Debug> Semigroup<
//     Result<X, E>,
//     Result<X2, E>,
//     Result<XR, E>> for ResultSemigroup {
//     fn combine<MX>(a: Result<X, E>, b: Result<X2, E>) -> Result<XR, E>
//         where
//             MX: Semigroup<X, X2, XR> {
//         Self::combine_results(a, b, MX::combine)
//     }
// }
//
// impl<'a, E: Debug, X, Y, Z> Functor<'a> for ResultMonad<E, X, Y, Z> {
//     type X = X;
//     type Y = Y;
//     type FX = Result<X, E>;
//     type FY = Result<Y, E>;
//     fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
//         f.map(func)
//     }
// }
//
// impl<'a, E: Debug, X, Y, Z> Applicative<'a> for ResultMonad<E, X, Y, Z> {
//     fn pure(x: X) -> Self::FX {
//         Ok(x)
//     }
// }
//
// impl<'a, E: Debug, X, Y, Z, M> ApplicativeApply<'a, M> for ResultMonad<E, X, Y, Z>
//     where
//         M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
//     type FMapper = Result<M, E>;
//     fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
//         x.and_then(|x_in| func.map(|f| f(x_in)))
//     }
// }
//
// impl<'a, E: Debug, X, Y, Z> Functor2<'a> for ResultMonad<E, X, Y, Z> {
//     type Z = Z;
//     type FZ = Result<Z, E>;
//     fn fmap2(r1: Self::FX,
//              r2: Self::FY,
//              func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
//         r1.and_then(|i| r2.map(|j| func(i, j)))
//     }
// }
//
// impl<'a, E: Debug, X, Y, Z> Monad<'a> for ResultMonad<E, X, Y, Z> {
//     fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//         f.and_then(func)
//     }
// }
//
// impl<'a, X, Y, Z, E: Debug> Foldable<'a> for ResultMonad<E, X, Y, Z> {
//     type Y2 = Y;
//     fn fold(f: Self::FX,
//             init: Self::Y,
//             func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Y2 {
//         match f {
//             Ok(i) => func(init, i),
//             Err(_e) => init
//         }
//     }
// }
//
// impl<'a, E: Debug, X, Y, Z> MonadError<'a> for ResultMonad<E, X, Y, Z> {
//     type E=E;
//     fn raise_error(err: Self::E) -> Self::FX {
//         Err(err)
//     }
//
//     fn handle_error(f: Self::FX, recovery: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
//         f.or_else(|e| recovery(e))
//     }
//
//     fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
//         f
//     }
// }
//
// impl<'a, E: Debug, X, Y> Productable<'a> for ResultMonad<E, X, Y, (X, Y)> {}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_semigroup() {
//         let a = Ok(3);
//         let b = Ok(5);
//
//         let out: Result<i32, ()> = ResultSemigroup::combine(a, b);
//         assert_eq!(Ok(8), out);
//
//         let a = Ok(3);
//         let b = Err(());
//
//         let out = ResultSemigroup::combine(a, b);
//         assert_eq!(Err(()), out);
//
//         let a = Ok("Hello");
//         let b = Ok(" World".to_string());
//
//         let out: Result<String, ()> = ResultSemigroup::combine(a, b);
//         assert_eq!("Hello World", out.unwrap());
//
//         let a = Ok(3);
//         let b = Ok(5);
//
//         let out: Result<i32, ()> = ResultSemigroup::combine_inner::<IntMulMonoid>(a, b);
//         assert_eq!(Ok(15), out);
//
//         let a = Ok(3);
//         let b = Ok(5);
//
//         let out: Result<i32, ()> = ResultSemigroup::combine_inner::<IntMulMonoid>(a, b);
//         assert_eq!(Ok(15), out);
//     }
//
//     #[test]
//     fn test_monoid() {
//         let out: Result<u32, ()> = ResultSemigroup::empty();
//         assert_eq!(Ok(0), out);
//     }
//
//     #[test]
//     fn test_applicative() {
//         let out = ResultMonad::<(), u32>::pure(3);
//         assert_eq!(Ok(3), out);
//
//         let out: Result<&str, ()> = ResultMonad::pure("test");
//         assert_eq!(Ok("test"), out);
//     }
//
//     #[test]
//     fn test_apply() {
//         let input: Result<u32, ()> = ResultMonad::pure(3);
//         let out: Result<String, ()> = ResultMonad::apply(Ok(|i| format!("{} beans", i)), input);
//         assert_eq!(Ok("3 beans".to_string()), out);
//     }
//
//     #[test]
//     fn test_functor() {
//         let out: Result<u32, ()> = ResultMonad::pure(3);
//         let res = ResultMonad::fmap(out, |i| i + 4);
//         assert_eq!(Ok(7), res);
//
//         let out: Result<String, ()> = ResultMonad::pure(format!("Hello"));
//         let res = ResultMonad::fmap(out, |i| format!("{} World", i));
//         assert_eq!("Hello World", res.unwrap());
//
//         let out: Result<String, ()> = ResultSemigroup::empty();
//         let res = ResultMonad::fmap(out, |i| format!("{} World", i));
//         assert_eq!(Ok(" World".to_string()), res);
//
//         let out1: Result<u32, ()> = ResultMonad::pure(3);
//         let out2 = ResultMonad::<(), String>::pure(format!("Bowls"));
//         let res = ResultMonad::fmap2(out1, out2, |i, j| format!("{} {} of salad", i+4, j));
//         assert_eq!("7 Bowls of salad", res.unwrap());
//
//         let out1: Result<u32, ()> = ResultMonad::pure(3);
//         let out2: Result<String, ()> = Err(());
//         let res = ResultMonad::fmap2(out1, out2, |i, j| format!("{} {} of salad", i+4, j));
//         assert_eq!(Err(()), res);
//     }
//
//     #[test]
//     fn test_monad() {
//         let out: Result<u32, ()> = ResultMonad::pure(3);
//         let res = ResultMonad::flat_map(out, |i| Ok(i + 4));
//         assert_eq!(Ok(7), res);
//
//         let out: Result<String, ()> = ResultSemigroup::empty();
//         let res = ResultMonad::flat_map(out, |i| Ok(format!("{} World", i)));
//         assert_eq!(Ok(" World".to_string()), res);
//
//     }
//
//     #[test]
//     fn test_monad_error() {
//         let out: Result<u32, u32> = ResultMonad::pure(3);
//         let res: Result<String, u32> = ResultMonad::flat_map(out, |i| match i % 2 {
//             0 => ResultMonad::pure("Good".to_string()),
//             _ => ResultMonad::raise_error(i)
//         });
//         assert_eq!(Err(3), res);
//     }
//
//     #[test]
//     fn test_product() {
//         let out1: Result<u32, ()> = ResultMonad::pure(3);
//         let out2: Result<u32, ()> = ResultMonad::pure(5);
//         let res = ResultMonad::product(out1, out2);
//         assert_eq!(Ok((3, 5)), res);
//
//         let out1: Result<u32, ()> = ResultMonad::pure(3);
//         let out2: Result<u32, ()> = ResultSemigroup::empty();
//         let res = ResultMonad::product(out1, out2);
//         assert_eq!(Ok((3, 0)), res);
//     }
// }
