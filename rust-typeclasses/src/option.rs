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
impl OptionEffect {
    pub fn sg<X, X2, XR, T: Semigroup<X, X2, XR>>(&self, ev: T) -> CombineInnerSemigroup<X, X2, XR, T>{
        CombineInnerSemigroup::apply(ev)
    }
}

pub const OP_EV: &OptionEffect = &OptionEffect;

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
impl<'a, X, Y> Functor<'a, Option<X>, Option<Y>, X, Y> for OptionEffect {
    fn fmap(&self, f: Option<X>, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Option<Y> {
        f.map(func)
    }
}
impl<'a, X, Y, Z> Functor2<'a, Option<X>, Option<Y>, Option<Z>, X, Y, Z> for OptionEffect {
    fn fmap2(&self, fa: Option<X>, fb: Option<Y>, func: impl 'a + Fn(&X, &Y) -> Z + Send + Sync) -> Option<Z> {
        fa.and_then(|i| fb.map(|j| func(&i, &j)))
    }
}
impl<'a, X, Y> Monad<'a, Option<X>, Option<Y>> for OptionEffect {
    type In = X;
    type Out = Y;

    fn flat_map(&self, f: Option<X>, func: impl 'a + Fn(X) -> Option<Y> + Send + Sync) -> Option<Y> {
        f.and_then(func)
    }
}
impl<'a, X, Y: Clone> Foldable<'a, Option<X>, X, Y, Y> for OptionEffect {
    fn fold(&self, f: Option<X>, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Y {
        match f {
            Some(i) => func(init, i),
            None => init
        }
    }
}
impl<X: Clone, Y: Clone> Productable<Option<X>, Option<Y>, Option<(X, Y)>, X, Y> for OptionEffect {
    fn product(&self, fa: Option<X>, fb: Option<Y>) -> Option<(X, Y)> {
        fmap2(OP_EV, fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        let a = Some(3);
        let b = Some(5);

        let out = combine(OP_EV.sg(IADD_SG), a, b);
        assert_eq!(Some(8), out);

        let a = Some(3);
        let b = None;

        let out = combine(OP_EV.sg(IADD_SG), a, b);
        assert_eq!(None, out);

        let a = Some("Hello");
        let b = Some(" World".to_string());

        let out = combine(OP_EV.sg(STR_SG), a, b);
        assert_eq!("Hello World", out.unwrap());
    }

    #[test]
    fn test_monoid() {
        let out: Option<u32> = empty(OP_EV);
        assert_eq!(None, out);
    }

    #[test]
    fn test_applicative() {
        let out = OP_EV.pure(3);
        assert_eq!(Some(3), out);

        let out = pure(OP_EV, "test");
        assert_eq!(Some("test"), out);
    }

    #[test]
    fn test_functor() {
        let out: Option<u32> = pure(OP_EV, 3);
        let res = fmap(OP_EV, out, |i| i + 4);
        assert_eq!(Some(7), res);

        let out: Option<String> = pure(OP_EV, format!("Hello"));
        let res = fmap(OP_EV, out, |i| format!("{} World", i));
        assert_eq!("Hello World", res.unwrap());

        let out: Option<String> = empty(OP_EV);
        let res = fmap(OP_EV, out, |i| format!("{} World", i));
        assert_eq!(None, res);

        let out1: Option<u32> = pure(OP_EV, 3);
        let out2: Option<String> = pure(OP_EV, format!("Bowls"));
        let res = fmap2(OP_EV, out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
        assert_eq!("7 Bowls of salad", res.unwrap());
    }

    #[test]
    fn test_monad() {
        let out: Option<u32> = pure(OP_EV, 3);
        let res = flat_map(OP_EV, out, |i| Some(i + 4));
        assert_eq!(Some(7), res);

        let out: Option<u32> = empty(OP_EV);
        let res = flat_map(OP_EV, out, |i| Some(i + 4));
        assert_eq!(None, res);

        let out: Option<u32> = pure(OP_EV, 2);
        let res = fold(OP_EV, out, 0, |init, i| init + i);
        assert_eq!(2, res);

        let out: Option<u32> = empty(OP_EV);
        let res = fold(OP_EV, out, 0, |init, i| init + i);
        assert_eq!(0, res);
    }

    #[test]
    fn test_product() {
        let out1: Option<u32> = pure(OP_EV, 3);
        let out2: Option<u32> = pure(OP_EV, 5);
        let res = product(OP_EV, out1, out2);
        assert_eq!(Some((3, 5)), res);

        let out1: Option<u32> = pure(OP_EV, 3);
        let out2: Option<u32> = empty(OP_EV);
        let res = product(OP_EV, out1, out2);
        assert_eq!(None, res);
    }
}
