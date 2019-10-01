use super::typeclasses::{F,
                         applicative::*,
                         functor::*,
                         monad::*,
                         monoid::*,
                         product::*,
                         semigroup::*,
                         traverse::*};
use std::marker::PhantomData;

impl<X, E> F<X> for Result<X, E> {}

impl<X, E, T: Semigroup<X>> Semigroup<Result<X, E>> for OneParamSemigroup<X, T> {
    fn combine(self, a: Result<X, E>, b: Result<X, E>) -> Result<X, E> {
        a.and_then(|i| b.map(|j| self.t.combine(i, j)))
    }
}

struct ResultEffect;
impl<T, E: Default> Monoid<Result<T, E>> for ResultEffect {
    fn empty(self) -> Result<T, E> {
        Err(E::default())
    }
}
impl<T, E> Applicative<Result<T, E>, T> for ResultEffect {
    fn pure(self, x: T) -> Result<T, E> {
        Ok(x)
    }
}
impl<X, Y, E> Functor<Result<X, E>, Result<Y, E>, X, Y> for ResultEffect {
    fn fmap(self, f: Result<X, E>, func: fn(X) -> Y) -> Result<Y, E> {
        f.map(|i| func(i))
    }
}
impl<X, Y, Z, E> Functor2<Result<X, E>, Result<Y, E>, Result<Z, E>, X, Y, Z> for ResultEffect {
    fn fmap2(self, r1: Result<X, E>, r2: Result<Y, E>, func: fn(&X, &Y) -> Z) -> Result<Z, E> {
        r1.and_then(|i| r2.map(|j| func(&i, &j)))
    }
}

#[cfg(test)]
mod tests {
    use crate::typeclasses::semigroup::*;
    use crate::typeclasses::applicative::*;
    use crate::typeclasses::monoid::*;
    use crate::typeclasses::functor::*;
    use crate::result::*;

    #[test]
    fn test_semigroup() {
        let a = Ok(3);
        let b = Ok(5);

        let out: Result<i32, ()> = combine(OneParamSemigroup::apply(IntAddSemigroup), a, b);
        assert_eq!(Ok(8), out);

        let a = Ok(3);
        let b = Err(());

        let out = combine(OneParamSemigroup::apply(IntAddSemigroup), a, b);
        assert_eq!(Err(()), out);

        let a = Ok("Hello".to_string());
        let b = Ok(" World".to_string());

        let out: Result<String, ()> = combine(OneParamSemigroup::apply(StringSemigroup), a, b);
        assert_eq!("Hello World", out.unwrap());
    }

    #[test]
    fn test_monoid() {
        let out: Result<u32, ()> = ResultEffect.empty();
        assert_eq!(Err(()), out);
    }

    #[test]
    fn test_applicative() {
        let out: Result<u32, ()> = ResultEffect.pure(3);
        assert_eq!(Ok(3), out);

        let out: Result<&str, ()> = ResultEffect.pure("test");
        assert_eq!(Ok("test"), out);
    }

    #[test]
    fn test_functor() {
        let out: Result<u32, ()> = pure(ResultEffect, 3);
        let res = fmap(ResultEffect, out, |i| i + 4);
        assert_eq!(Ok(7), res);

        let out: Result<String, ()> = pure(ResultEffect, format!("Hello"));
        let res = fmap(ResultEffect, out, |i| format!("{} World", i));
        assert_eq!("Hello World", res.unwrap());

        let out: Result<String, ()> = empty(ResultEffect);
        let res = fmap(ResultEffect, out, |i| format!("{} World", i));
        assert_eq!(Err(()), res);

        let out1: Result<u32, ()> = pure(ResultEffect, 3);
        let out2: Result<String, ()> = pure(ResultEffect, format!("Bowls"));
        let res = fmap2(ResultEffect, out1, out2, |i, j| format!("{} {} of salad", i+4, j));
        assert_eq!("7 Bowls of salad", res.unwrap());

        let out1: Result<u32, ()> = pure(ResultEffect, 3);
        let out2: Result<String, ()> = Err(());
        let res = fmap2(ResultEffect, out1, out2, |i, j| format!("{} {} of salad", i+4, j));
        assert_eq!(Err(()), res);

    }
}
