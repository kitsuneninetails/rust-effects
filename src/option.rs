use super::*;

impl<X> F<X> for Option<X> {}

impl<X> Semigroup<Option<X>> for Option<X> where X: Semigroup<X> {
    fn combine(a: Option<X>, b: Option<X>) -> Option<X> {
        a.and_then(|i| b.map(|j| X::combine(i, j)))
    }
}

impl<X> Monoid for Option<X> {
    fn empty() -> Self { None }
}

impl<X, Y> Functor<Option<X>, Option<Y>, X, Y> for Option<X> {
    fn fmap(f: Option<X>, func: fn(X) -> Y) -> Option<Y> {
        f.map(func)
    }
}

impl<X, Y, Z> Functor2<Option<X>, Option<Y>, Option<Z>, X, Y, Z> for Option<X> {
    fn fmap2(fa: Option<X>, fb: Option<Y>, func: fn(&X, &Y) -> Z) -> Option<Z> {
        fa.and_then(|i| fb.map(|j| func(&i, &j)))
    }
}

impl<X: Clone, Y: Clone> Productable<Option<X>, Option<Y>, Option<(X, Y)>, X, Y> for Option<X> {
    fn product(fa: Option<X>, fb: Option<Y>) -> Option<(X,Y)> {
        Option::fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }

}

impl<X> Applicative<Option<X>, X> for Option<X> {
    fn pure(x: X) -> Option<X> {
        Some(x)
    }
}

impl<X, Y> Monad<Option<X>, Option<Y>, X, Y> for Option<X> {
    fn flat_map(f: Option<X>, func: fn(X) -> Option<Y>) -> Option<Y> {
        f.and_then(func)
    }
}

impl<X, Y> Foldable<Option<X>, X, Y>  for Option<X> {
    fn fold(f: Option<X>, init: Y, func: impl Fn(Y, X) -> Y) -> Y {
        match f {
            Some(i) => func(init, i),
            None => init
        }
    }
}

impl<X, Y: Clone, AY, AR> Traverse<Option<X>, Option<Y>, AY, AR, X, Y> for Option<X>
    where AY: F<Y> + Applicative<AY, Y> + Functor2<AY, AR, AR, Y, Option<Y>, Option<Y>>,
          AR: F<Option<Y>> + Applicative<AR, Option<Y>> {
    fn traverse(fa: Option<X>, func: fn(X) -> AY) -> AR {
        let f_ay = fa.map(|i| func(i));
        let empty_option = Option::<Y>::empty();
        let init = AR::pure(empty_option);
        Option::fold(f_ay, init, |acc, item| {
            AY::fmap2(item, acc, |p, _| Some(p.clone()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmap() {
        let o = Option::fmap(Some(1), |x| x + 1);
        assert!(o.is_some());
        assert_eq!(o.unwrap(), 2);
    }

    #[test]
    fn test_fmap_change_type() {
        let o = Option::fmap(Some(1), |x| format!("{}", x));
        assert!(o.is_some());
        assert_eq!(o.unwrap(), "1");
    }

    #[test]
    fn test_flat_map() {
        let o = Option::flat_map(Option::pure(1), |x| Some(format!("{}", x)));
        assert!(o.is_some());
        assert_eq!(o.unwrap(), "1");
    }

    #[test]
    fn test_fmap2() {
        let o = Option::fmap2(Option::pure(2), Option::pure("tons".to_string()),
                               |x, y| format!("{} {}", x, y));
        assert!(o.is_some());
        assert_eq!(o.unwrap(), "2 tons");
    }

    #[test]
    fn test_fold() {
        let o = Option::fold(Some(1), 2, |x, y| x + y);
        assert_eq!(o, 3);
    }

    #[test]
    fn test_traverse() {
        let r = Option::traverse(Some(2), |x| match x {
            2 => Ok(format!("{}", x)),
            _ => Err(format!("Bad data"))
        });
        assert!(r.is_ok());
        let o = r.unwrap();
        assert!(o.is_some());
        assert_eq!(o.unwrap(), "2");
    }

    #[test]
    fn test_traverse_err() {
        let r = Option::traverse(Some(3), |x| match x {
            2 => Ok(format!("{}", x)),
            _ => Err(format!("Bad data"))
        });
        assert!(r.is_err());
    }

    #[test]
    fn test_traverse_none() {
        let r = Option::traverse(None, |x| match x {
            2 => Ok(format!("{}", x)),
            _ => Err(format!("Bad data"))
        });
        assert!(r.is_ok());
        let o = r.unwrap();
        assert!(o.is_none());
    }

}
