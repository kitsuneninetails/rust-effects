use super::prelude::*;
use std::marker::PhantomData;

impl<X, E> F<X> for Result<X, E> {}
impl<'a, X, X2, XR, E> SemigroupEffect<Result<X, E>, Result<X2, E>, Result<XR, E>> for Result<X, E>
    where
        X: SemigroupEffect<X, X2, XR> {
    type Fct = ResultEffect<E>;
}
impl<X, E: Default> MonoidEffect<Result<X, E>> for Result<X, E> {
    type Fct = ResultEffect<E>;
}
impl<X, E> ApplicativeEffect for Result<X, E> {
    type X = X;
    type Fct = ResultEffect<E>;
}
impl<'a, X, Y, E> MonadEffect<'a, X, Y> for Result<X, E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    type Fct = ResultEffect<E>;
}
impl<'a, X, Y: Clone, E> FoldableEffect<'a, X, Y, Y> for Result<X, E> {
    type FX = Result<X,E>;
    type Fct = ResultEffect<E>;
}
impl<'a, X, Y, E> FunctorEffect<'a, X, Y> for Result<X,E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    type Fct = ResultEffect<E>;
}
impl<'a, X, Y, Z, E> Functor2Effect<'a, X, Y, Z> for Result<X,E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    type FZ = Result<Z, E>;
    type Fct = ResultEffect<E>;
}
impl<'a, X: Clone, Y: Clone, E> ProductableEffect<X, Y> for Result<X,E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    type FXY = Result<(X, Y), E>;
    type Fct = ResultEffect<E>;
}

pub struct ResultEffect<E> {
    _p: PhantomData<E>
}

impl<E> ResultEffect<E> {
    pub fn apply() -> ResultEffect<E> {
        ResultEffect {
            _p: PhantomData
        }
    }

    fn combine_results<X, X2, XR, F>(a: Result<X, E>,
                                     b: Result<X2, E>,
                                     func: F) -> Result<XR, E>
        where
            F: FnOnce(X, X2) -> XR {
        a.and_then(|i| b.map(|j| func(i, j)))
    }
}
impl<E> Effect for ResultEffect<E>
{}

impl<X, X2, XR, E> Semigroup<
    Result<X, E>,
    Result<X2, E>,
    Result<XR, E>> for ResultEffect<E>
    where
        X: SemigroupEffect<X, X2, XR> {
    fn combine(a: Result<X, E>, b: Result<X2, E>) -> Result<XR, E> {
        Self::combine_results(a, b, combine)
    }
}
impl <X, E> SemigroupInner<Result<X, E>, X> for ResultEffect<E> {
    fn combine_inner<TO>(a: Result<X, E>, b: Result<X, E>) -> Result<X, E>
        where
            TO: Semigroup<X, X, X> {
        Self::combine_results(a, b, TO::combine)
    }
}
impl<X, E: Default> Monoid<Result<X, E>> for ResultEffect<E> {
    fn empty() -> Result<X, E> {
        Err(E::default())
    }
}
impl<X, E> Applicative<X> for ResultEffect<E> {
    type FX = Result<X, E>;
    fn pure(x: X) -> Self::FX {
        Ok(x)
    }
}
impl<'a, X, Y, E> Functor<'a, X, Y> for ResultEffect<E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
        f.map(func)
    }
}
impl<'a, X, Y, Z, E> Functor2<'a, X, Y, Z> for ResultEffect<E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
    type FZ = Result<Z, E>;
    fn fmap2(r1: Self::FX, r2: Self::FY, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Self::FZ {
        r1.and_then(|i| r2.map(|j| func(i, j)))
    }
}
impl<'a, X, Y, E> Monad<'a, X, Y> for ResultEffect<E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;

    fn flat_map(f: Self::FX, func: impl 'a + Fn(X) -> Self::FY + Send + Sync) -> Self::FY {
        f.and_then(func)
    }
}
impl<'a, X, Y: Clone, E> Foldable<'a, X, Y, Y> for ResultEffect<E> {
    type FX = Result<X, E>;
    fn fold(f: Self::FX, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Y {
        match f {
            Ok(i) => func(init, i),
            Err(_e) => init
        }
    }
}
impl<X: Clone, Y: Clone, E> Productable<X, Y> for ResultEffect<E> {
    type FX = Result<X, E>;
    type FY = Result<Y, E>;
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
        assert_eq!(Err(()), out);
    }

    #[test]
    fn test_applicative() {
        let out = pure::<Result::<u32, ()>>(3);
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

        let out: Result<String, ()> = empty();
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

        let out: Result<String, ()> = empty();
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
        let out2: Result<u32, ()> = empty();
        let res = product(out1, out2);
        assert_eq!(Err(()), res);
    }
}
