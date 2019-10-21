use super::prelude::*;

#[macro_use] use crate::*;

impl<X> F<X> for Option<X> {}

semigroup_effect! { 1, Option, OptionEffect }
monoid_effect! { 1, Option, OptionEffect }
applicative_effect! { 1, Option, OptionEffect }
functor_effect! { 1, Option, OptionEffect }
functor2_effect! { 1, Option, OptionEffect }
monad_effect! { 1, Option, OptionEffect }
foldable_effect! { 1C, Option, OptionEffect }
productable_effect! { 1, Option, OptionEffect }

pub struct OptionEffect;
impl OptionEffect {
    fn combine_options<X, X2, XR, F>(a: Option<X>,
                                     b: Option<X2>,
                                     func: F) -> Option<XR>
    where
    F: FnOnce(X, X2) -> XR {
        a.and_then(|i| b.map(|j| func(i, j)))
    }
}

impl Effect for OptionEffect {}

impl<X, X2, XR> Semigroup<Option<X>, Option<X2>, Option<XR>> for OptionEffect
    where
        X: SemigroupEffect<X, X2, XR> {
    fn combine(a: Option<X>, b: Option<X2>) -> Option<XR> {
        OptionEffect::combine_options(a, b, combine)
    }
}

impl <'a, X> SemigroupInner<'a, Option<X>, X> for OptionEffect where X: 'a {
    fn combine_inner<TO>(a: Option<X>, b: Option<X>) -> Option<X>
        where
            TO: 'a + Semigroup<X, X, X> {
        Self::combine_options(a, b, TO::combine)
    }
}

impl<X> Monoid<Option<X>> for OptionEffect {
    fn empty() -> Option<X> {
        None
    }
}

impl<X> Applicative<X> for OptionEffect {
    type FX = Option<X>;
    fn pure(x: X) -> Self::FX {
        Some(x)
    }
}

impl<'a, X, Y> Functor<'a, X, Y> for OptionEffect {
    type FX = Option<X>;
    type FY = Option<Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
        f.map(func)
    }
}

impl<'a, X, Y, Z> Functor2<'a, X, Y, Z> for OptionEffect {
    type FX = Option<X>;
    type FY = Option<Y>;
    type FZ = Option<Z>;
    fn fmap2(fa: Self::FX, fb: Self::FY, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Self::FZ {
        fa.and_then(|i| fb.map(|j| func(i, j)))
    }
}

impl<'a, X, Y> Monad<'a, X, Y> for OptionEffect {
    type FX = Option<X>;
    type FY = Option<Y>;

    fn flat_map(f: Self::FX, func: impl 'a + Fn(X) -> Self::FY + Send + Sync) -> Self::FY {
        f.and_then(func)
    }
}

impl<'a, X, Y: Clone> Foldable<'a, X, Y, Y> for OptionEffect {
    type FX = Option<X>;
    fn fold(f: Self::FX, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Y {
        match f {
            Some(i) => func(init, i),
            None => init
        }
    }
}
impl<X: Clone, Y: Clone> Productable<X, Y> for OptionEffect {
    type FX = Option<X>;
    type FY = Option<Y>;
    type FXY = Option<(X, Y)>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY {
        fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

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
        assert_eq!(None, out);

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
        let out = pure::<Option::<u32>>(3);
        assert_eq!(Some(3), out);

        let out: Option<&str> = pure("test");
        assert_eq!(Some("test"), out);
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
