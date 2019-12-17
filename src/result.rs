///// Result Typeclass Behaviors
/////
///// Semigroup
/////     `combine(Ok(X), Ok(Y)) => Ok(combine(X, Y))`
/////     `combine(Ok(X), Err(E)) => Err(E)`
/////     `combine(Err(E), Ok(Y)) => Err(E)`
/////     `combine(Err(E1), Err(E2)) => Err(E1)`
///// Monoid
/////     `empty() => Ok(X::empty())`
///// Applicative
/////     `pure(X) => Ok(X)`
///// ApplicativeApply
/////     `fmap(Ok(fn X -> Y), Ok(X)) => Ok(fn(X)) => Ok(Y)`
/////     `fmap(Ok(fn X -> Y), Err(E)) => Err(E)`
/////     `fmap(Err(E), Ok(X)) => Err(E)`
/////     `fmap(Err(E1), Err(E2)) => Err(E2)`
///// Functor
/////     `fmap(Ok(X), fn X -> Y) => Ok(fn(X)) => Ok(Y)`
/////     `fmap(Err(E), fn X -> Y) => Err(E)`
///// Functor2
/////     `fmap2(Ok(X), Ok(Y), fn X, Y -> Z) => Ok(fn(X, Y))`
/////     `fmap2(Ok(X), Err(E2), fn X, Y -> Z) => Err(E2)`
/////     `fmap2(Err(E1), Ok(Y), fn X, Y -> Z) => Err(E1)`
/////     `fmap2(Err(E1), Err(E2), fn X, Y -> Z) => Err(E1)`
///// Monad
/////     `flat_map(Ok(X), fn X -> Ok(Y)) => fn(X) => Ok(Y)`
/////     `flat_map(Ok(X), fn X -> Err(E)) => fn(X) => Err(E)`
/////     `flat_map(Err(E), fn X -> Ok(Y)) => Err(E)`
/////     `flat_map(Err(E1), fn X -> Err(E)) => Err(E1)`
///// Foldable
/////     `fold(Ok(X), init, fn Y, X -> Y2) => fn(init, X) => Y2`
/////     `fold(Err(E), init, fn Y, X -> Y2) => init => Y`
/////     Note: Y and Y2 are the same type, just possibly two different values.
///// MonadError
/////     `raise_error(E) => Err(E)`
///// Productable -
/////     `product(Ok(X), Ok(Y)) => Ok((X, Y))`
/////     `product(Ok(X), Err(E)) => Err(E)`
/////     `product(Err(E), Ok(Y)) => Err(E)`
/////     `product(Err(E1), Err(E2)) => Err(E1)`
///// Traverse
/////     `Not implemented`
//
//use super::prelude::*;
//use std::marker::PhantomData;
//use std::fmt::Debug;
//
//use crate::*;
//
//impl<X, E: Debug> F<X> for Result<X, E> {}
//
//semigroup_effect! { 2, Result, ResultEffect }
//monoid_effect! { 2, Result, ResultEffect }
//applicative_effect! { 2, Result, ResultEffect }
//applicativeapply_effect! { 2, Result, ResultEffect }
//functor_effect! { 2, Result, ResultEffect }
//functor2_effect! { 2, Result, ResultEffect }
//monad_effect! { 2, Result, ResultEffect }
//foldable_effect! { 2, Result, ResultEffect }
//monaderror_effect! { 2, Result, ResultEffect }
//productable_effect! { 2, Result, ResultEffect }
//
//pub struct ResultEffect<E: Debug, X=(), Y=(), Z=()> {
//    _a: PhantomData<X>,
//    _b: PhantomData<Y>,
//    _c: PhantomData<Z>,
//    _p: PhantomData<E>
//}
//
//impl<E: Debug, X, Y, Z> ResultEffect<E, X, Y, Z> {
//    pub fn new(_: Z) -> Self {
//        ResultEffect {
//            _a: PhantomData,
//            _b: PhantomData,
//            _c: PhantomData,
//            _p: PhantomData
//        }
//    }
//
//    fn combine_results<X1, X2, XR, F>(a: Result<X1, E>,
//                                      b: Result<X2, E>,
//                                      func: F) -> Result<XR, E>
//        where
//            F: FnOnce(X1, X2) -> XR {
//        a.and_then(|i| b.map(|j| func(i, j)))
//    }
//}
//
//#[macro_export]
//macro_rules! result_monad {
//    () => (ResultEffect::new(()))
//}
//
//impl<E: Debug, X, Y, Z> Effect for ResultEffect<E, X, Y, Z>{}
//
//impl<X, X2, XR, E: Debug> Semigroup<
//    Result<X, E>,
//    Result<X2, E>,
//    Result<XR, E>> for ResultEffect<E, X, X2, XR>
//    where
//        X: SemigroupEffect<X, X2, XR> {
//    fn combine(a: Result<X, E>, b: Result<X2, E>) -> Result<XR, E> {
//        Self::combine_results(a, b, combine)
//    }
//}
//
//impl <'a, X, E: Debug> SemigroupInner<'a, Result<X, E>, X> for ResultEffect<E, X, X, X>  where X: 'a, E: 'a {
//    fn combine_inner<TO>(a: Result<X, E>, b: Result<X, E>) -> Result<X, E>
//        where
//            TO: 'a + Semigroup<X, X, X> {
//        Self::combine_results(a, b, TO::combine)
//    }
//}
//
//impl<E: Debug, X: MonoidEffect, Y, Z> Monoid for ResultEffect<E, X, Y, Z> {
//    type M = Result<X, E>;
//    fn empty() -> Self::M {
//        Ok(empty::<X>())
//    }
//}
//
//impl<'a, E: Debug, X, Y, Z> Functor<'a> for ResultEffect<E, X, Y, Z> {
//    type FnctX = X;
//    type FnctY = Y;
//    type FnctZ = Z;
//    type FctForX = Result<X, E>;
//    type FctForY = Result<Y, E>;
//    type FctForZ = Result<Z, E>;
//    fn fmap(f: Self::FctForX, func: impl 'a + Fn(Self::FnctX) -> Self::FnctY + Send + Sync) -> Self::FctForY {
//        f.map(func)
//    }
//    fn fmap2(r1: Self::FctForX,
//             r2: Self::FctForY,
//             func: impl 'a + Fn(Self::FnctX, Self::FnctY) -> Self::FnctZ + Send + Sync) -> Self::FctForZ {
//        r1.and_then(|i| r2.map(|j| func(i, j)))
//    }
//}
//
//impl<'a, E: Debug, X, Y, Z> Applicative<'a> for ResultEffect<E, X, Y, Z> {
//    fn pure(x: X) -> Self::FX {
//        Ok(x)
//    }
//}
//
//impl<'a, E: Debug, X, Y, Z, M> ApplicativeApply<'a, M> for ResultEffect<E, X, Y, Z>
//    where
//        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
//    type FMapper = Result<M, E>;
//    fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
//        x.and_then(|x_in| func.map(|f| f(x_in)))
//    }
//}
//
//impl<'a, E: Debug, X, Y, Z> Monad<'a> for ResultEffect<E, X, Y, Z> {
//    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//        f.and_then(func)
//    }
//}
//
//impl<'a, X, Y, Z, E: Debug> Foldable<'a> for ResultEffect<E, X, Y, Z> {
//    type Fld = Result<X, E>;
//    type FldInner = X;
//    type Folded = Y;
//    type Folded2 = Y;
//    fn fold(f: Self::Fld,
//            init: Self::Folded,
//            func: impl 'a + Fn(Self::Folded, Self::FldInner) -> Self::Folded2 + Send + Sync) -> Self::Folded2 {
//        match f {
//            Ok(i) => func(init, i),
//            Err(_e) => init
//        }
//    }
//}
//
//impl<'a, E: Debug, X, Y, Z> MonadError<'a> for ResultEffect<E, X, Y, Z> {
//    type E=E;
//    fn raise_error(err: Self::E) -> Self::FX {
//        Err(err)
//    }
//
//    fn handle_error(f: Self::FX, recovery: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
//        f.or_else(|e| recovery(e))
//    }
//
//    fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
//        f
//    }
//}
//
//impl<'a, E: Debug, X, Y> Productable<'a> for ResultEffect<E, X, Y, (X, Y)> {}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_semigroup() {
//        let a = Ok(3);
//        let b = Ok(5);
//
//        let out: Result<i32, ()> = combine(a, b);
//        assert_eq!(Ok(8), out);
//
//        let a = Ok(3);
//        let b = Err(());
//
//        let out = combine(a, b);
//        assert_eq!(Err(()), out);
//
//        let a = Ok("Hello");
//        let b = Ok(" World".to_string());
//
//        let out: Result<String, ()> = combine(a, b);
//        assert_eq!("Hello World", out.unwrap());
//
//        let a = Ok(3);
//        let b = Ok(5);
//
//        let out: Result<i32, ()> = ResultEffect::combine_inner::<IntMulSemigroup>(a, b);
//        assert_eq!(Ok(15), out);
//
//        let a = Ok(3);
//        let b = Ok(5);
//
//        let out: Result<i32, ()> = combine_inner::<_, _, IntMulSemigroup>(a, b);
//        assert_eq!(Ok(15), out);
//    }
//
//    #[test]
//    fn test_monoid() {
//        let out: Result<u32, ()> = empty();
//        assert_eq!(Ok(0), out);
//    }
//
//    #[test]
//    fn test_applicative() {
//        let out = pure::<Result::<u32, ()>>(3);
//        assert_eq!(Ok(3), out);
//
//        let out: Result<&str, ()> = pure("test");
//        assert_eq!(Ok("test"), out);
//    }
//
//    #[test]
//    fn test_apply() {
//        let input: Result<u32, ()> = pure(3);
//        let out: Result<String, ()> = apply(Ok(|i| format!("{} beans", i)), input);
//        assert_eq!(Ok("3 beans".to_string()), out);
//    }
//
//    #[test]
//    fn test_functor() {
//        let out: Result<u32, ()> = pure(3);
//        let res = fmap(out, |i| i + 4);
//        assert_eq!(Ok(7), res);
//
//        let out: Result<String, ()> = pure(format!("Hello"));
//        let res = fmap(out, |i| format!("{} World", i));
//        assert_eq!("Hello World", res.unwrap());
//
//        let out: Result<String, ()> = empty();
//        let res = fmap(out, |i| format!("{} World", i));
//        assert_eq!(Ok(" World".to_string()), res);
//
//        let out1: Result<u32, ()> = pure(3);
//        let out2 = pure::<Result<String, ()>>(format!("Bowls"));
//        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i+4, j));
//        assert_eq!("7 Bowls of salad", res.unwrap());
//
//        let out1: Result<u32, ()> = pure(3);
//        let out2: Result<String, ()> = Err(());
//        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i+4, j));
//        assert_eq!(Err(()), res);
//    }
//
//    #[test]
//    fn test_monad() {
//        let out: Result<u32, ()> = pure(3);
//        let res = flat_map(out, |i| Ok(i + 4));
//        assert_eq!(Ok(7), res);
//
//        let out: Result<String, ()> = empty();
//        let res = flat_map(out, |i| Ok(format!("{} World", i)));
//        assert_eq!(Ok(" World".to_string()), res);
//
//    }
//
//    #[test]
//    fn test_monad_error() {
//        let out: Result<u32, u32> = pure(3);
//        let res: Result<String, u32> = flat_map(out, |i| match i % 2 {
//            0 => pure("Good".to_string()),
//            _ => raise_error(i)
//        });
//        assert_eq!(Err(3), res);
//    }
//
//    #[test]
//    fn test_product() {
//        let out1: Result<u32, ()> = pure(3);
//        let out2: Result<u32, ()> = pure(5);
//        let res = product(out1, out2);
//        assert_eq!(Ok((3, 5)), res);
//
//        let out1: Result<u32, ()> = pure(3);
//        let out2: Result<u32, ()> = empty();
//        let res = product(out1, out2);
//        assert_eq!(Ok((3, 0)), res);
//    }
//}
