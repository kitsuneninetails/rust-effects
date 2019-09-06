use super::*;

impl<X> F<X> for Vec<X> {}

impl<X> Semigroup<Vec<X>> for Vec<X> {
    fn combine(a: Vec<X>, b: Vec<X>) -> Vec<X> {
        let mut ret = a;
        ret.extend(b);
        ret
    }
}

impl<X> Monoid for Vec<X> {
    fn empty() -> Self { vec![] }
}

impl<X, Y> Functor<Vec<X>, Vec<Y>, X, Y> for Vec<X> {
    fn fmap(f: Vec<X>, func: fn(X) -> Y) -> Vec<Y> {
        f.into_iter().map(func).collect()
    }
}

impl<X, Y, Z> Functor2<Vec<X>, Vec<Y>, Vec<Z>, X, Y, Z> for Vec<X> {
    fn fmap2(fa: Vec<X>, fb: Vec<Y>, func: fn(&X, &Y) -> Z) -> Vec<Z> {
        fa.into_iter().flat_map(|i|{
            let ret: Vec<Z> = fb.iter().map(|j| {
                func(&i, j)
            }).collect();
            ret
        }).collect()
    }
}

impl<X: Clone, Y: Clone> Productable<Vec<X>, Vec<Y>, Vec<(X, Y)>, X, Y> for Vec<X> {
    fn product(fa: Vec<X>, fb: Vec<Y>) -> Vec<(X,Y)> {
        Vec::fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

impl<X> Applicative<Vec<X>, X> for Vec<X> {
    fn pure(x: X) -> Vec<X> {
        vec![x]
    }
}

impl<X, Y> Monad<Vec<X>, Vec<Y>, X, Y> for Vec<X> {
    fn flat_map(f: Vec<X>, func: fn(X) -> Vec<Y>) -> Vec<Y> {
        f.into_iter().flat_map(func).collect()
    }
}

impl<X, Y> Foldable<Vec<X>, X, Y> for Vec<X> {
    fn fold(f: Vec<X>, init: Y, func: impl Fn(Y, X) -> Y) -> Y {
        let mut accum = init;
        for i in f.into_iter() {
            accum = func(accum, i);
        }
        accum
    }
}

impl<X, Y: Clone, AY, AR> Traverse<Vec<X>, Vec<Y>, AY, AR, X, Y> for Vec<X>
    where AY: F<Y> + Applicative<AY, Y> + Functor2<AY, AR, AR, Y, Vec<Y>, Vec<Y>>,
          AR: F<Vec<Y>> + Applicative<AR, Vec<Y>> {
    fn traverse(fa: Vec<X>, func: fn(X) -> AY) -> AR {
        // Make an empty list of Y where Y is whatever the Applicative (Option, Future, etc.)
        // is set to hold after the function.  This is used to kick start the contained vector
        // which will hold the resulting values (and will be contained by the specified
        // Applicative (Future, Option, Etiher, etc.))
        let empty_ret_list = Vec::<Y>::empty();

        // Fold on the initial list (Vec<X>) and start with initial accumulator set to
        // A basic G<Vec<Y>> where G is the Applicative that will be returned from the specified
        // function (Option, Future, Either, etc.).
        let init = AR::pure(empty_ret_list);
        Vec::fold(
            fa,
            init,
            |acc, item| {
                // The folding function should take this Applicative (Option, Future, etc.) and
                // "combine" the results with the accumulated value.  This is what determines
                // whether the accumulated value turns into a "negative" result (like a None,
                // or a Future::fail(), or a Either::Err, etc.)

                // First, get the returned Applicative from the func call:
                let ret_ay = func(item);

                // Apply a map between the returned value and the accumulated value.  The
                // mapping function should know how to put the two together (they are the same
                // Applicative type, but they each hold a different type inside).
                AY::fmap2(
                    ret_ay,
                    acc,
                    |fx, y| {
                        // This function adds the returned inner value onto the accumulating list
                        // inside the Applicative.  Applicatives know how to only allow this
                        // combination if both the accumulated Applicative and the returned
                        // Applicative both match up to "positive" values (like success or Some()).
                        // These next lines won't even get called unless that is the case.
                        let mut r = vec![fx.clone()];
                        r.extend(y.clone());
                        r
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmap() {
        let v = Vec::fmap(vec![1], |x| x + 1);
        assert_eq!(v[0], 2);
    }

    #[test]
    fn test_fmap_change_type() {
        let v = Vec::fmap(vec![2], |x| format!("{}", x));
        assert_eq!(v[0], "2");
    }

    #[test]
    fn test_flat_map() {
        let v = Vec::flat_map(vec![1, 2], |x| vec!["V".to_string(), (format!("{}", x))]);
        assert_eq!(v[0], "V");
        assert_eq!(v[1], "1");
        assert_eq!(v[2], "V");
        assert_eq!(v[3], "2");
    }

    #[test]
    fn test_fmap2() {
        let v: Vec<u32> = Vec::fmap2(vec![1, 2], vec![3, 4], |x, y| x.clone() + y.clone());
        assert_eq!(v[0], 4);
        assert_eq!(v[1], 5);
        assert_eq!(v[2], 5);
        assert_eq!(v[3], 6);
    }

    #[test]
    fn test_fold() {
        let r = Vec::fold(vec![1, 2], 0, |x, y| x + y);
        assert_eq!(r, 3);
    }

    #[test]
    fn test_traverse() {
        let o = Vec::traverse(vec![2, 4, 6], |x| match x % 2 == 0 {
            true => Some(format!("{}", x)),
            false => None
        });
        assert!(o.is_some());
        let v = o.unwrap();
        assert!(v.contains(&"2".to_string()));
        assert!(v.contains(&"4".to_string()));
        assert!(v.contains(&"6".to_string()));
    }

    #[test]
    fn test_traverse_err() {
        let o = Vec::traverse(vec![2, 5, 6], |x| match x % 2 == 0 {
            true => Some(format!("{}", x)),
            false => None
        });
        assert!(o.is_none());
    }

}
