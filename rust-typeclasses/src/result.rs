use super::prelude::*;

impl<X, E> F<X> for Result<X, E> {}
impl<X, E> ApplicativeEffect for Result<X, E> {
    type X = X;
    type Fct = ResultEffect;
    fn app() -> Self::Fct {
        ResultEffect
    }
}
impl<'a, X, Y, E> MonadEffect<'a, Result<X, E>, Result<Y, E>, X, Y> for Result<X, E> {
    type Fct = ResultEffect;
    fn monad(&self) -> Self::Fct { ResultEffect }
}
impl<'a, X, Y: Clone, E> FoldableEffect<'a, Result<X,E>, X, Y, Y> for Result<X, E> {
    type Fct = ResultEffect;
    fn foldable(&self) -> Self::Fct { ResultEffect }
}
impl<'a, X, Y, E> FunctorEffect<'a, Result<X,E>, Result<Y,E>, X, Y> for Result<X,E> {
    type Fct = ResultEffect;
    fn functor(&self) -> Self::Fct { ResultEffect }
}
impl<'a, X, Y, Z, E> Functor2Effect<'a, Result<X,E>, Result<Y,E>, Result<Z,E>, X, Y, Z> for Result<X,E> {
    type Fct = ResultEffect;
    fn functor2(&self) -> Self::Fct { ResultEffect }
}
impl<'a, X: Clone, Y: Clone, E> ProductableEffect<Result<X,E>, Result<Y,E>, Result<(X, Y), E>, X, Y> for Result<X,E> {
    type Fct = ResultEffect;
    fn productable(&self) -> Self::Fct { ResultEffect }
}

impl<X, X2, XR, E, T: Semigroup<X, X2, XR>> Semigroup<
    Result<X, E>,
    Result<X2, E>,
    Result<XR, E>> for CombineInnerSemigroup<X, X2, XR, T> {
    fn combine(self, a: Result<X, E>, b: Result<X2, E>) -> Result<XR, E> {
        a.and_then(|i| b.map(|j| self.t.combine(i, j)))
    }
}

pub struct ResultEffect;
impl ResultEffect {
    pub fn sg<X, X2, XR, T: Semigroup<X, X2, XR>>(&self, ev: T) -> CombineInnerSemigroup<X, X2, XR, T>{
        CombineInnerSemigroup::apply(ev)
    }
}
impl Effect for ResultEffect
{}
pub const RES_EV: &ResultEffect = &ResultEffect;

impl<T, E: Default> Monoid<Result<T, E>> for ResultEffect {
    fn empty(&self) -> Result<T, E> {
        Err(E::default())
    }
}
impl<T, E> Applicative<Result<T, E>, T> for ResultEffect {
    fn pure(&self, x: T) -> Result<T, E> {
        Ok(x)
    }
}
impl<'a, X, Y, E> Functor<'a, Result<X, E>, Result<Y, E>, X, Y> for ResultEffect {
    fn fmap(&self, f: Result<X, E>, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Result<Y, E> {
        f.map(func)
    }
}
impl<'a, X, Y, Z, E> Functor2<'a, Result<X, E>, Result<Y, E>, Result<Z, E>, X, Y, Z> for ResultEffect {
    fn fmap2(&self, r1: Result<X, E>, r2: Result<Y, E>, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Result<Z, E> {
        r1.and_then(|i| r2.map(|j| func(i, j)))
    }
}
impl<'a, X, Y, E> Monad<'a, Result<X, E>, Result<Y, E>> for ResultEffect {
    type In = X;
    type Out = Y;

    fn flat_map(&self, f: Result<X, E>, func: impl 'a + Fn(X) -> Result<Y, E> + Send + Sync) -> Result<Y, E> {
        f.and_then(func)
    }
}
impl<'a, X, Y: Clone, E> Foldable<'a, Result<X, E>, X, Y, Y> for ResultEffect {
    fn fold(&self, f: Result<X, E>, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Y {
        match f {
            Ok(i) => func(init, i),
            Err(_e) => init
        }
    }
}
impl<X: Clone, Y: Clone, E> Productable<Result<X, E>, Result<Y, E>, Result<(X, Y), E>, X, Y> for ResultEffect {
    fn product(&self, fa: Result<X, E>, fb: Result<Y, E>) -> Result<(X, Y), E> {
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

        let out: Result<i32, ()> = combine(RES_EV.sg(IADD_SG), a, b);
        assert_eq!(Ok(8), out);

        let a = Ok(3);
        let b = Err(());

        let out = combine(RES_EV.sg(IADD_SG), a, b);
        assert_eq!(Err(()), out);

        let a = Ok("Hello");
        let b = Ok(" World".to_string());

        let out: Result<String, ()> = combine(RES_EV.sg(STR_SG), a, b);
        assert_eq!("Hello World", out.unwrap());
    }

    #[test]
    fn test_monoid() {
        let out: Result<u32, ()> = RES_EV.empty();
        assert_eq!(Err(()), out);
    }

    #[test]
    fn test_applicative() {
        let out: Result::<u32, ()> = Result::<u32, ()>::app().pure(3);
        assert_eq!(Ok(3), out);

        let out: Result<&str, ()> = pure("test");
        assert_eq!(Ok("test"), out);
    }

    #[test]
    fn test_functor() {
        let out: Result<u32, ()> = pure(3);
        let res = fmap(out, |i| i + 4);
        assert_eq!(Ok(7), res);

        let out: Result<String, ()> = pure(format!("Hello"));
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!("Hello World", res.unwrap());

        let out: Result<String, ()> = empty(RES_EV);
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!(Err(()), res);

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

        let out: Result<String, ()> = empty(RES_EV);
        let res = flat_map(out, |i| Ok(format!("{} World", i)));
        assert_eq!(Err(()), res);

    }

    #[test]
    fn test_product() {
        let out1: Result<u32, ()> = pure(3);
        let out2: Result<u32, ()> = pure(5);
        let res = product(out1, out2);
        assert_eq!(Ok((3, 5)), res);

        let out1: Result<u32, ()> = pure(3);
        let out2: Result<u32, ()> = empty(RES_EV);
        let res = product(out1, out2);
        assert_eq!(Err(()), res);
    }
}
