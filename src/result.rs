/// Result Typeclass Behaviors
///
/// Semigroup
///     `combine(Ok(X), Ok(Y)) => Ok(combine(X, Y))`
///     `combine(Ok(X), Err(E2)) => Err(E2)`
///     `combine(Err(E1), Ok(Y)) => Err(E1)`
///     `combine(Err(E1), Err(E2)) => Err(E1)`
/// Monoid
///     `empty() => Ok(X::empty())`
/// Applicative
///     `pure(X) => Ok(X)`
/// Functor
///     `fmap(Ok(X), fn T1 -> T2) => Ok(fn(X))`
///     `fmap(Err(E), fn T1 -> T2) => Err(E)`
/// Functor2
///     `fmap2(Ok(X), Ok(Y), fn T1 T2 -> T3) => Ok(fn(X, Y))`
///     `fmap2(Ok(X), Err(E2), fn T1 T2 -> T3) => Err(E2)`
///     `fmap2(Err(E1), Ok(Y), fn T1 T2 -> T3) => Err(E1)`
///     `fmap2(Err(E1), Err(E2), fn T1 T2 -> T3) => Err(E1)`
/// Monad
///     `flat_map(Ok(X), fn T1 -> Option<T2>) => Ok(Y)` if fn(X) returns Ok(Y)
///     `flat_map(Ok(X), fn T1 -> Option<T2>) => Err(E2)` if fn(X) returns Err(E2)
///     `flat_map(Err(E), fn T1 -> Option<T2>) => Err(E)`
/// Foldable
///     `fold(Ok(X), init, fn TI T1 -> TI) => fn(init, X)`
///     `fold(Err(E), init, fn TI T1 -> TI) => init`
/// MonadError
///     `raise_error(E) => Err(E)`
/// Productable -
///     `product(Ok(X), Ok(Y)) => Ok((X, Y))`
///     `product(Ok(X), Err(E2)) => Err(2E)`
///     `product(Err(E1), Ok(Y)) => Err(E1)`
///     `product(Err(E1), Err(E2)) => Err(E1)`
/// Traverse
///     `Not implemented`

use super::prelude::*;
use std::marker::PhantomData;
use std::fmt::Debug;

use crate::*;

impl<X, E: Debug> F<X> for Result<X, E> {}

semigroup_effect! { 2, Result, ResultEffect }
monoid_effect! { 2, Result, ResultEffect }
applicative_effect! { 2, Result, ResultEffect }
applicativeapply_effect! { 2, Result, ResultEffect }
functor_effect! { 2, Result, ResultEffect }
functor2_effect! { 2, Result, ResultEffect }
monad_effect! { 2, Result, ResultEffect }
foldable_effect! { 2, Result, ResultEffect }
monaderror_effect! { 2, Result, ResultEffect }
productable_effect! { 2, Result, ResultEffect }

pub struct ResultEffect<E: Debug, X=(), Y=(), Z=()> {
    _a: PhantomData<X>,
    _b: PhantomData<Y>,
    _c: PhantomData<Z>,
    _p: PhantomData<E>
}

impl<E: Debug, X, Y, Z> ResultEffect<E, X, Y, Z> {
    pub fn new(_: Z) -> Self {
        ResultEffect {
            _a: PhantomData,
            _b: PhantomData,
            _c: PhantomData,
            _p: PhantomData
        }
    }

    fn combine_results<X1, X2, XR, F>(a: Result<X1, E>,
                                      b: Result<X2, E>,
                                      func: F) -> Result<XR, E>
        where
            F: FnOnce(X1, X2) -> XR {
        a.and_then(|i| b.map(|j| func(i, j)))
    }
}

#[macro_export]
macro_rules! result_monad {
    () => (ResultEffect::new(()))
}

impl<E: Debug, X, Y, Z> Effect for ResultEffect<E, X, Y, Z>{}

impl<X, X2, XR, E: Debug> Semigroup<
    Result<X, E>,
    Result<X2, E>,
    Result<XR, E>> for ResultEffect<E, X, X2, XR>
    where
        X: SemigroupEffect<X, X2, XR> {
    fn combine(a: Result<X, E>, b: Result<X2, E>) -> Result<XR, E> {
        Self::combine_results(a, b, combine)
    }
}

impl <'a, X, E: Debug> SemigroupInner<'a, Result<X, E>, X> for ResultEffect<E, X, X, X>  where X: 'a, E: 'a {
    fn combine_inner<TO>(a: Result<X, E>, b: Result<X, E>) -> Result<X, E>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_results(a, b, TO::combine)
    }
}

impl<E: Debug, X: MonoidEffect<X>, Y, Z> Monoid<Result<X, E>> for ResultEffect<E, X, Y, Z> {
    fn empty() -> Result<X, E> {
        Ok(empty::<X>())
    }
}

impl<'a, E: Debug, X, Y, Z> Functor<'a> for ResultEffect<E, X, Y, Z> {
    type X = X;
    type Y = Y;
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
        f.map(func)
    }
}

impl<'a, E: Debug, X, Y, Z> Applicative<'a> for ResultEffect<E, X, Y, Z> {
    fn pure(x: X) -> Self::FX {
        Ok(x)
    }
}

impl<'a, E: Debug, X, Y, Z, M> ApplicativeApply<'a, M> for ResultEffect<E, X, Y, Z>
    where
        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
    type FMapper = Result<M, E>;
    fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
        x.and_then(|x_in| func.map(|f| f(x_in)))
    }
}

impl<'a, E: Debug, X, Y, Z> Functor2<'a> for ResultEffect<E, X, Y, Z> {
    type Z = Z;
    type FZ = Result<Z, E>;
    fn fmap2(r1: Self::FX,
             r2: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        r1.and_then(|i| r2.map(|j| func(i, j)))
    }
}

impl<'a, E: Debug, X, Y, Z> Monad<'a> for ResultEffect<E, X, Y, Z> {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        f.and_then(func)
    }
}

impl<'a, X, Y: Clone, Z, E: Debug> Foldable<'a> for ResultEffect<E, X, Y, Z> {
    type Z = Y;
    fn fold(f: Self::FX,
            init: Self::Y,
            func: impl 'a + Fn(Self::Y, Self::X) -> Self::Y + Send + Sync) -> Self::Z {
        match f {
            Ok(i) => func(init, i),
            Err(_e) => init
        }
    }
}

impl<'a, E: Debug, X, Y, Z> MonadError<'a> for ResultEffect<E, X, Y, Z> {
    type E=E;
    fn raise_error(err: Self::E) -> Self::FX {
        Err(err)
    }

    fn handle_error(f: Self::FX, recovery: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
        f.or_else(|e| recovery(e))
    }

    fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
        f
    }
}

impl<'a, X: Clone, Y: Clone, Z, E: Debug> Productable<'a> for ResultEffect<E, X, Y, Z> {
    type FXY = Result<(X, Y), E>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
        fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        let a = Ok(3);
        let b = Ok(5);

        let out: Result<i32, ()> = combine(a, b);
        assert_eq!(Ok(8), out);

        let a = Ok(3);
        let b = Err(());

        let out = combine(a, b);
        assert_eq!(Err(()), out);

        let a = Ok("Hello");
        let b = Ok(" World".to_string());

        let out: Result<String, ()> = combine(a, b);
        assert_eq!("Hello World", out.unwrap());

        let a = Ok(3);
        let b = Ok(5);

        let out: Result<i32, ()> = ResultEffect::combine_inner::<IntMulSemigroup>(a, b);
        assert_eq!(Ok(15), out);

        let a = Ok(3);
        let b = Ok(5);

        let out: Result<i32, ()> = combine_inner::<_, _, IntMulSemigroup>(a, b);
        assert_eq!(Ok(15), out);
    }

    #[test]
    fn test_monoid() {
        let out: Result<u32, ()> = empty();
        assert_eq!(Ok(0), out);
    }

    #[test]
    fn test_applicative() {
        let out = pure::<Result::<u32, ()>>(3);
        assert_eq!(Ok(3), out);

        let out: Result<&str, ()> = pure("test");
        assert_eq!(Ok("test"), out);
    }

    #[test]
    fn test_apply() {
        let input: Result<u32, ()> = pure(3);
        let out: Result<String, ()> = apply(Ok(|i| format!("{} beans", i)), input);
        assert_eq!(Ok("3 beans".to_string()), out);
    }

    #[test]
    fn test_functor() {
        let out: Result<u32, ()> = pure(3);
        let res = fmap(out, |i| i + 4);
        assert_eq!(Ok(7), res);

        let out: Result<String, ()> = pure(format!("Hello"));
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!("Hello World", res.unwrap());

        let out: Result<String, ()> = empty();
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!(Ok(" World".to_string()), res);

        let out1: Result<u32, ()> = pure(3);
        let out2 = pure::<Result<String, ()>>(format!("Bowls"));
        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i+4, j));
        assert_eq!("7 Bowls of salad", res.unwrap());

        let out1: Result<u32, ()> = pure(3);
        let out2: Result<String, ()> = Err(());
        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i+4, j));
        assert_eq!(Err(()), res);
    }

    #[test]
    fn test_monad() {
        let out: Result<u32, ()> = pure(3);
        let res = flat_map(out, |i| Ok(i + 4));
        assert_eq!(Ok(7), res);

        let out: Result<String, ()> = empty();
        let res = flat_map(out, |i| Ok(format!("{} World", i)));
        assert_eq!(Ok(" World".to_string()), res);

    }

    #[test]
    fn test_monad_error() {
        let out: Result<u32, u32> = pure(3);
        let res: Result<String, u32> = flat_map(out, |i| match i % 2 {
            0 => pure("Good".to_string()),
            _ => raise_error(i)
        });
        assert_eq!(Err(3), res);
    }

    #[test]
    fn test_product() {
        let out1: Result<u32, ()> = pure(3);
        let out2: Result<u32, ()> = pure(5);
        let res = product(out1, out2);
        assert_eq!(Ok((3, 5)), res);

        let out1: Result<u32, ()> = pure(3);
        let out2: Result<u32, ()> = empty();
        let res = product(out1, out2);
        assert_eq!(Ok((3, 0)), res);
    }
}
