use super::typeclasses::{F,
                         applicative::*,
                         functor::*,
                         monad::*,
                         monoid::*,
                         product::*,
                         semigroup::*};

impl<X> F<X> for Option<X> {}

impl<X, X2, XR, T: Semigroup<X, X2, XR>> Semigroup<
    Option<X>,
    Option<X2>,
    Option<XR>> for CombineInnerSemigroup<X, X2, XR, T> {
    fn combine(self, a: Option<X>, b: Option<X2>) -> Option<XR> {
        a.and_then(|i| b.map(|j| self.t.combine(i, j)))
    }
}

pub struct OptionEffect;
impl<T> Monoid<Option<T>> for OptionEffect {
    fn empty(&self) -> Option<T> {
        None
    }
}
impl<T> Applicative<Option<T>, T> for OptionEffect {
    fn pure(&self, x: T) -> Option<T> {
        Some(x)
    }
}
impl<X, Y> Functor<Option<X>, Option<Y>, X, Y> for OptionEffect {
    fn fmap(&self, f: Option<X>, func: fn(X) -> Y) -> Option<Y> {
        f.map(func)
    }
}
impl<X, Y, Z> Functor2<Option<X>, Option<Y>, Option<Z>, X, Y, Z> for OptionEffect {
    fn fmap2(&self, fa: Option<X>, fb: Option<Y>, func: fn(&X, &Y) -> Z) -> Option<Z> {
        fa.and_then(|i| fb.map(|j| func(&i, &j)))
    }
}
impl<X, Y> Monad<Option<X>, Option<Y>, X, Y> for OptionEffect {
    fn flat_map(&self, f: Option<X>, func: fn(X) -> Option<Y>) -> Option<Y> {
        f.and_then(func)
    }
}
impl<X, Y: Clone> Foldable<Option<X>, X, Y, Y> for OptionEffect {
    fn fold(&self, f: Option<X>, init: Y, func: fn(Y, X) -> Y) -> Y {
        match f {
            Some(i) => func(init, i),
            None => init
        }
    }
}
impl<X: Clone, Y: Clone> Productable<Option<X>, Option<Y>, Option<(X, Y)>, X, Y> for OptionEffect {
    fn product(&self, fa: Option<X>, fb: Option<Y>) -> Option<(X, Y)> {
        fmap2(&OptionEffect, fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        let a = Some(3);
        let b = Some(5);

        let out = combine(CombineInnerSemigroup::apply(IntAddSemigroup), a, b);
        assert_eq!(Some(8), out);

        let a = Some(3);
        let b = None;

        let out = combine(CombineInnerSemigroup::apply(IntAddSemigroup), a, b);
        assert_eq!(None, out);

        let a = Some("Hello");
        let b = Some(" World".to_string());

        let out = combine(CombineInnerSemigroup::apply(StringSemigroup), a, b);
        assert_eq!("Hello World", out.unwrap());
    }

    #[test]
    fn test_monoid() {
        let out: Option<u32> = empty(&OptionEffect);
        assert_eq!(None, out);
    }

    #[test]
    fn test_applicative() {
        let out = OptionEffect.pure(3);
        assert_eq!(Some(3), out);

        let out = pure(&OptionEffect, "test");
        assert_eq!(Some("test"), out);
    }

    #[test]
    fn test_functor() {
        let out: Option<u32> = pure(&OptionEffect, 3);
        let res = fmap(&OptionEffect, out, |i| i + 4);
        assert_eq!(Some(7), res);

        let out: Option<String> = pure(&OptionEffect, format!("Hello"));
        let res = fmap(&OptionEffect, out, |i| format!("{} World", i));
        assert_eq!("Hello World", res.unwrap());

        let out: Option<String> = empty(&OptionEffect);
        let res = fmap(&OptionEffect, out, |i| format!("{} World", i));
        assert_eq!(None, res);

        let out1: Option<u32> = pure(&OptionEffect, 3);
        let out2: Option<String> = pure(&OptionEffect, format!("Bowls"));
        let res = fmap2(&OptionEffect, out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
        assert_eq!("7 Bowls of salad", res.unwrap());
    }

    #[test]
    fn test_monad() {
        let out: Option<u32> = pure(&OptionEffect, 3);
        let res = flat_map(&OptionEffect, out, |i| Some(i + 4));
        assert_eq!(Some(7), res);

        let out: Option<u32> = empty(&OptionEffect);
        let res = flat_map(&OptionEffect, out, |i| Some(i + 4));
        assert_eq!(None, res);

        let out: Option<u32> = pure(&OptionEffect, 2);
        let res = fold(&OptionEffect, out, 0, |init, i| init + i);
        assert_eq!(2, res);

        let out: Option<u32> = empty(&OptionEffect);
        let res = fold(&OptionEffect, out, 0, |init, i| init + i);
        assert_eq!(0, res);
    }

    #[test]
    fn test_product() {
        let out1: Option<u32> = pure(&OptionEffect, 3);
        let out2: Option<u32> = pure(&OptionEffect, 5);
        let res = product(&OptionEffect, out1, out2);
        assert_eq!(Some((3, 5)), res);

        let out1: Option<u32> = pure(&OptionEffect, 3);
        let out2: Option<u32> = empty(&OptionEffect);
        let res = product(&OptionEffect, out1, out2);
        assert_eq!(None, res);
    }
}
