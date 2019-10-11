use super::prelude::*;

impl<X> F<X> for Option<X> {}
impl<X> MonoidEffect<Option<X>> for Option<X> {
    type Fct = OptionEffect;
}
impl<X> ApplicativeEffect for Option<X> {
    type X = X;
    type Fct = OptionEffect;
}
impl<'a, X, Y> MonadEffect<'a, Option<X>, Option<Y>, X, Y> for Option<X> {
    type Fct = OptionEffect;
    fn monad(&self) -> Self::Fct { OptionEffect }
}
impl<'a, X, Y: Clone> FoldableEffect<'a, Option<X>, X, Y, Y> for Option<X> {
    type Fct = OptionEffect;
    fn foldable(&self) -> Self::Fct { OptionEffect }
}
impl<'a, X, Y> FunctorEffect<'a, Option<X>, Option<Y>, X, Y> for Option<X> {
    type Fct = OptionEffect;
    fn functor(&self) -> Self::Fct { OptionEffect }
}
impl<'a, X, Y, Z> Functor2Effect<'a, Option<X>, Option<Y>, Option<Z>, X, Y, Z> for Option<X> {
    type Fct = OptionEffect;
    fn functor2(&self) -> Self::Fct { OptionEffect }
}
impl<'a, X: Clone, Y: Clone> ProductableEffect<Option<X>, Option<Y>, Option<(X, Y)>, X, Y> for Option<X> {
    type Fct = OptionEffect;
    fn productable(&self) -> Self::Fct { OptionEffect }
}

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

impl Effect for OptionEffect {}

pub const OP_EV: &OptionEffect = &OptionEffect;

impl<X> Monoid<Option<X>> for OptionEffect {
    fn empty() -> Option<X> {
        None
    }
}
impl<X> Applicative<Option<X>, X> for OptionEffect {
    fn pure(x: X) -> Option<X> {
        Some(x)
    }
}
impl<'a, X, Y> Functor<'a, Option<X>, Option<Y>, X, Y> for OptionEffect {
    fn fmap(&self, f: Option<X>, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Option<Y> {
        f.map(func)
    }
}
impl<'a, X, Y, Z> Functor2<'a, Option<X>, Option<Y>, Option<Z>, X, Y, Z> for OptionEffect {
    fn fmap2(&self, fa: Option<X>, fb: Option<Y>, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Option<Z> {
        fa.and_then(|i| fb.map(|j| func(i, j)))
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
        let out: Option<u32> = empty();
        assert_eq!(None, out);
    }

    #[test]
    fn test_applicative() {
        let out = <Option::<u32> as ApplicativeEffect>::Fct::pure(3);
        assert_eq!(Some(3), out);

        let out: Option<&str> = pure("test");
        assert_eq!(Some("test"), out);
    }

    #[test]
    fn test_functor() {
        let out = <Option::<u32> as ApplicativeEffect>::Fct::pure(3);
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
