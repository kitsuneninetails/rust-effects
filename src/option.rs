/// Option Typeclass Behaviors
///
/// Semigroup (result-type must be a monoid for identity and parameters must be semigroup
/// for combination)
///     `combine(Some(X), Some(Y)) => Some(combine(X, Y))`
///     `combine(Some(X), None) => Some(X)`
///     `combine(None, Some(Y)) => Some(Y)`
///     `combine(None, None) => None`
/// Monoid
///     `empty() => None`
/// Applicative
///     `pure(X) => Some(X)`
/// ApplicativeApply
///     `apply(Some(fn X -> X2), Some(X)) => Some(fn(X)) => Some(X2)`
///     `apply(Some(fn X -> X2), None) => None`
///     `apply(None, Some(X)) => None`
///     `apply(None, None) => None`
/// Functor
///     `fmap(Some(X), fn X -> X2) => Some(fn(X)) => Some(X2)`
///     `fmap(None, fn X -> X2) => None`
/// Functor2
///     `fmap2(Some(X), Some(Y), fn X, Y -> Z) => Some(fn(X, Y)) => Some(Z)`
///     `fmap2(Some(X), None, fn X, Y -> Z) => None`
///     `fmap2(None, Some(Y), fn X, Y -> Z) => None`
///     `fmap2(None, None, fn X, Y -> Z) => None`
/// Monad
///     `flat_map(Some(X), fn X -> Some(Y)) => fn(X) => Some(Y)`
///     `flat_map(Some(X), fn X -> None) => None`
///     `flat_map(None, fn X -> Some(Y)) => None`
///     `flat_map(None, fn X -> None) => None`
/// MonadError
///     `raise_error(E) => None`
///     `handle_error(Some(X), fn E -> Some(X2)) => Some(X)`
///     `handle_error(Some(X), fn E -> None) => Some(X)`
///     `handle_error(None, fn E -> Some(X2)) => Some(X2)`
///     `handle_error(None, fn E -> None) => None`
///     `attempt(Some(X)) => Ok(X)`
///     `attempt(None) => Err(())`
/// Foldable
///     `fold(Some(X), init, fn Y, X -> Y2) => fn(Y, X) => Y2`
///     `fold(None, init, fn Y, X -> Y2) => Y`
///     Note: Y and Y2 are the same type, just possibly two different values.
/// Productable -
///     `product(Some(X), Some(Y)) => Some((X, Y))`
///     `product(Some(X), None) => None`
///     `product(None, Some(Y)) => None`
///     `product(None, None) => None`
/// Traverse
///     `Not implemented`

use super::prelude::*;

use crate::*;
use std::marker::PhantomData;

impl<X> F<X> for Option<X> {}

semigroup_effect! { 1, Option, OptionEffect }
monoid_effect! { 1, Option, OptionEffect }
applicative_effect! { 1, Option, OptionEffect }
applicativeapply_effect! { 1, Option, OptionEffect }
functor_effect! { 1, Option, OptionEffect }
functor2_effect! { 1, Option, OptionEffect }
monad_effect! { 1, Option, OptionEffect }
foldable_effect! { 1C, Option, OptionEffect }
monaderror_effect! { 1, Option, OptionEffect }
productable_effect! { 1, Option, OptionEffect }

pub struct OptionEffect<X=(), Y=(), Z=()> {
    _a: PhantomData<X>,
    _b: PhantomData<Y>,
    _c: PhantomData<Z>
}
impl<X, Y, Z> OptionEffect<X, Y, Z> {
    pub fn new(_: Z) -> Self {
        OptionEffect {
            _a: PhantomData,
            _b: PhantomData,
            _c: PhantomData
        }
    }
}

#[macro_export]
macro_rules! option_monad {
    () => (OptionEffect::new(()))
}

impl<X, Y, Z> Effect for OptionEffect<X, Y, Z> {}

impl<X, X2, XR> Semigroup<Option<X>, Option<X2>, Option<XR>> for OptionEffect<X, X2, XR>
    where
        X: SemigroupEffect<X, X2, XR> + SemigroupEffect<X, XR, XR>,
        X2: SemigroupEffect<X2, XR, XR>,
        XR: MonoidEffect<XR> {
    fn combine(a: Option<X>, b: Option<X2>) -> Option<XR> {
        match (a, b) {
            (None, None) => None,
            (Some(a), None) => Some(combine::<X, XR, XR>(a, empty::<XR>())),
            (None, Some(b)) => Some(combine::<X2, XR, XR>(b, empty::<XR>())),
            (Some(a), Some(b)) => Some(combine(a, b))
        }
    }
}

impl <'a, X> SemigroupInner<'a, Option<X>, X> for OptionEffect<X, X, X> where X: 'a {
    fn combine_inner<TO>(a: Option<X>, b: Option<X>) -> Option<X>
        where
            TO: 'a + Semigroup<X, X, X>,
            X: MonoidEffect<X> {
        match (a, b) {
            (None, None) => None,
            (Some(a), None) => Some(TO::combine(a, empty::<X>())),
            (None, Some(b)) => Some(TO::combine(empty::<X>(), b)),
            (Some(a), Some(b)) => Some(TO::combine(a, b))
        }
    }
}

impl<X, Y, Z> Monoid<Option<X>> for OptionEffect<X, Y, Z> {
    fn empty() -> Option<X> {
        None
    }
}
impl<'a, X, Y, Z> Functor<'a> for OptionEffect<X, Y, Z> {
    type X = X;
    type Y = Y;
    type FX = Option<X>;
    type FY = Option<Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
        f.map(func)
    }
}

impl<'a, X, Y, Z> Applicative<'a> for OptionEffect<X, Y, Z> {
    fn pure(x: X) -> Self::FX {
        Some(x)
    }
}

impl<'a, X, Y, Z, M> ApplicativeApply<'a, M> for OptionEffect<X, Y, Z>
    where
        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
    type FMapper = Option<M>;
    fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
        func.and_then(|f| x.map(|x_in| f(x_in)))
    }
}

impl<'a, X, Y, Z> Functor2<'a> for OptionEffect<X, Y, Z> {
    type Z = Z;
    type FZ = Option<Z>;
    fn fmap2(fa: Self::FX,
             fb: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        fa.and_then(|i| fb.map(|j| func(i, j)))
    }
}

impl<'a, X, Y, Z> Monad<'a> for OptionEffect<X, Y, Z> {
    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        f.and_then(func)
    }
}

impl<'a, X, Y, Z> Foldable<'a> for OptionEffect<X, Y, Z> {
    type Y2=Y;
    fn fold(f: Self::FX, init: Self::Y, func: impl 'a + Fn(Self::Y, Self::X) -> Y + Send + Sync) -> Self::Y2 {
        match f {
            Some(i) => func(init, i),
            None => init
        }
    }
}

impl<'a, X, Y, Z> MonadError<'a> for OptionEffect<X, Y, Z> {
    type E=();
    fn raise_error(_err: Self::E) -> Self::FX {
        None
    }

    fn handle_error(f: Self::FX, recovery: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
        f.or_else(|| recovery(()))
    }

    fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
        f.ok_or(())
    }
}

impl<'a, X, Y> Productable<'a> for OptionEffect<X, Y, (X, Y)> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        let a = Some(3);
        let b = Some(5);

        let out = combine(a, b);
        assert_eq!(Some(8), out);

        let a = Some(3);
        let b = None;

        let out = combine(a, b);
        assert_eq!(Some(3), out);

        let a = Some("Hello");
        let b = Some(" World".to_string());

        let out = combine(a, b);
        assert_eq!("Hello World", out.unwrap());

        let a = Some(3);
        let b = Some(4);

        let out = OptionEffect::combine_inner::<IntMulSemigroup>(a, b);
        assert_eq!(Some(12), out);

        let a = Some(3);
        let b = Some(4);

        let out = combine_inner::<_, _, IntMulSemigroup>(a, b);
        assert_eq!(Some(12), out);
    }

    #[test]
    fn test_monoid() {
        let out: Option<u32> = empty();
        assert_eq!(None, out);
    }

    #[test]
    fn test_applicative() {
        let out = pure::<Option::<_>>(3);
        assert_eq!(Some(3), out);

        let out: Option<_> = pure("test");
        assert_eq!(Some("test"), out);
    }

    #[test]
    fn test_apply() {
        let input: Option<u32> = pure(3);
        let out: Option<String> = apply(Some(|i| format!("{} beans", i)), input);
        assert_eq!(Some("3 beans".to_string()), out);

        let input: Option<_> = pure(3);
        let input2: Option<_> = pure(4);
        let func = pure(|i| { move |x| i + x });
        let out2: Option<_> = apply(apply(func, input), input2);

        assert_eq!(Some(7), out2);

        let input: Option<i32> = empty();
        let input2: Option<_> = pure(4);
        let func = pure(|i| { move |x| i + x });
        let out2: Option<_> = apply(apply(func, input), input2);

        assert_eq!(None, out2);
    }

    #[test]
    fn test_functor() {
        let out: Option<u32> = pure(3);
        let res = fmap(out, |i| i + 4);
        assert_eq!(Some(7), res);

        let out: Option<String> = pure(format!("Hello"));
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!("Hello World", res.unwrap());

        let out: Option<u32> = empty();
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!(None, res);

        let out1: Option<u32> = pure(3);
        let out2: Option<String> = pure(format!("Bowls"));
        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
        assert_eq!("7 Bowls of salad", res.unwrap());
    }

    #[test]
    fn test_monad() {
        let out: Option<u32> = pure(3);
        let res = flat_map(out, |i| Some(i + 4));
        assert_eq!(Some(7), res);

        let out: Option<u32> = empty();
        let res = flat_map(out, |i| Some(i + 4));
        assert_eq!(None, res);

        let out: Option<u32> = pure(2);
        let res = fold(out, 0, |init, i| init + i);
        assert_eq!(2, res);

        let out: Option<u32> = empty();
        let res = fold(out, 0, |init, i| init + i);
        assert_eq!(0, res);
    }

    #[test]
    fn test_monaderror() {
        let out: Option<u32> = pure(3);
        let res: Option<u32> = flat_map(out, |_| raise_error(()));
        assert_eq!(None, res);
    }

    #[test]
    fn test_product() {
        let out1: Option<u32> = pure(3);
        let out2: Option<u32> = pure(5);
        let res = product(out1, out2);
        assert_eq!(Some((3, 5)), res);

        let out1: Option<u32> = pure(3);
        let out2: Option<u32> = empty();
        let res = product(out1, out2);
        assert_eq!(None, res);
    }
}
