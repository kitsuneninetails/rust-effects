use std::default::Default;

use super::*;

impl<X, E> F<X> for Result<X, E> {}

impl<X, E> Semigroup<Result<X, E>> for Result<X, E> where X: Semigroup<X> {
    fn combine(a: Result<X, E>, b: Result<X, E>) -> Result<X, E> {
        a.and_then(|i| b.map(|j| X::combine(i, j)))
    }
}

impl<X: Default, E> Monoid for Result<X, E> {
    fn empty() -> Self { Ok(X::default()) }
}

impl<X, E, Y> Functor<Result<X, E>, Result<Y, E>, X, Y> for Result<X, E> {
    fn fmap(f: Result<X, E>, func: fn(X) -> Y) -> Result<Y, E> {
        f.map(func)
    }
}

impl<X, E, Y, Z> Functor2<Result<X, E>, Result<Y, E>, Result<Z, E>, X, Y, Z> for Result<X, E> {
    fn fmap2(fa: Result<X, E>, fb: Result<Y, E>, func: fn(&X, &Y) -> Z) -> Result<Z, E> {
        fa.and_then(|i| fb.map(|j| func(&i, &j)))
    }
}

impl<X: Clone, E, Y: Clone> Productable<Result<X, E>, Result<Y, E>, Result<(X, Y), E>, X, Y> for Result<X, E> {
    fn product(fa: Result<X, E>, fb: Result<Y, E>) -> Result<(X,Y), E> {
        Result::fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }

}

impl<X, E> Applicative<Result<X, E>, X> for Result<X, E> {
    fn pure(x: X) -> Result<X, E> {
        Ok(x)
    }
}

impl<X, E, Y> Monad<Result<X, E>, Result<Y, E>, X, Y> for Result<X, E> {
    fn flat_map(f: Result<X, E>, func: fn(X) -> Result<Y, E>) -> Result<Y, E> {
        f.and_then(func)
    }
}

impl<X, E, Y> Foldable<Result<X, E>, X, Y>  for Result<X, E> {
    fn fold(f: Result<X, E>, init: Y, func: impl Fn(Y, X) -> Y) -> Y {
        match f {
            Ok(i) => func(init, i),
            Err(_) => init
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmap() {
        let r = Result::<u32, String>::fmap(Ok(1), |x| x + 1);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 2);
    }

    #[test]
    fn test_fmap_change_type() {
        let r: Result<String, String> = Result::fmap(Ok(1), |x| format!("{}", x));
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "1");
    }

    #[test]
    fn test_fmap_err() {
        let r = Result::fmap(Err("Bad".to_string()), |x: u32| x + 1);
        assert!(r.is_err());
    }

    #[test]
    fn test_flat_map() {
        let r = Result::<u32, String>::flat_map(Result::pure(1),
                                                 |x| Ok(format!("{}", x)));
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "1");
    }

    #[test]
    fn test_fmap2() {
        let r: Result<String, String> = Result::fmap2(Result::pure(2), Result::pure("tons".to_string()),
                                                       |x, y| format!("{} {}", x, y));
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "2 tons");
    }

    #[test]
    fn test_fold() {
        let r = Result::<u32, String>::fold(Ok(1), 2, |x, y| x + y);
        assert_eq!(r, 3);
    }

    #[test]
    fn test_fold_err() {
        let v = Result::fold(Err("Bad".to_string()), 2, |x, y: u32| x + y);
        assert_eq!(v, 2);
    }
    
}
