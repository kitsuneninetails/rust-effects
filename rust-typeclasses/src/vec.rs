use super::prelude::*;

impl<X> F<X> for Vec<X> {}
impl<'a, X> SemigroupEffect<Vec<X>, Vec<X>, Vec<X>> for Vec<X> {
    type Fct = VecEffect;
}
impl<X> MonoidEffect<Vec<X>> for Vec<X> {
    type Fct = VecEffect;
}
impl<X> ApplicativeEffect for Vec<X> {
    type X = X;
    type Fct = VecEffect;
}
impl<'a, X, Y> MonadEffect<'a, Vec<X>, Vec<Y>, X, Y> for Vec<X> {
    type Fct = VecEffect;
}
impl<'a, X, Y> FoldableEffect<'a, Vec<X>, X, Y, Y> for Vec<X> {
    type Fct = VecEffect;
}
impl<'a, X, Y> FunctorEffect<'a, Vec<X>, Vec<Y>, X, Y> for Vec<X> {
    type Fct = VecEffect;
}
impl<'a, X, Y, Z> Functor2Effect<'a, Vec<X>, Vec<Y>, Vec<Z>, X, Y, Z> for Vec<X>
    where
        X: Clone,
        Y: Clone {
    type Fct = VecEffect;
}
impl<'a, X: Clone, Y: Clone> ProductableEffect<Vec<X>, Vec<Y>, Vec<(X, Y)>, X, Y> for Vec<X> {
    type Fct = VecEffect;
}
impl<'a, E, FR, X, Y, T> TraverseEffect<'a, Vec<X>, E, Vec<Y>, FR, X, Y> for Vec<X>
    where
        E: F<Y> + Functor2Effect<'a, E, FR, FR, Y, Vec<Y>, Vec<Y>>,
        FR: F<Vec<Y>> + ApplicativeEffect<X=Vec<Y>, Fct=T>,
        T: Applicative<FR, Vec<Y>> {
    type Fct = VecEffect;
}

#[derive(Clone)]
pub struct VecEffect;
impl Effect for VecEffect {}

impl<X> Semigroup<Vec<X>, Vec<X>, Vec<X>> for VecEffect {
    fn combine(a: Vec<X>, b: Vec<X>) -> Vec<X> {
        let mut ret = a;
        ret.extend(b);
        ret
    }
}

impl<X> Monoid<Vec<X>> for VecEffect {
    fn empty() -> Vec<X> {
        vec![]
    }
}
impl<X> Applicative<Vec<X>, X> for VecEffect {
    fn pure(x: X) -> Vec<X> {
        vec![x]
    }
}
impl<'a, X, Y> Functor<'a, Vec<X>, Vec<Y>, X, Y> for VecEffect {
    fn fmap(f: Vec<X>, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Vec<Y> {
        f.into_iter().map(func).collect()
    }
}
impl<'a, X, Y, Z> Functor2<'a, Vec<X>, Vec<Y>, Vec<Z>, X, Y, Z> for VecEffect
    where X: Clone,
          Y: Clone {
    fn fmap2(fa: Vec<X>, fb: Vec<Y>, func: impl 'a + Fn(X, Y) -> Z + Send + Sync) -> Vec<Z> {
        fa.into_iter().flat_map(|i| {
            let ret: Vec<Z> = fb.iter().map(|j| func(i.clone(), j.clone())).collect();
            ret
        }).collect()
    }
}
impl<'a, X, Y> Monad<'a, Vec<X>, Vec<Y>> for VecEffect {
    type In = X;
    type Out = Y;

    fn flat_map(f: Vec<X>, func: impl 'a + Fn(X) -> Vec<Y> + Send + Sync) -> Vec<Y> {
        f.into_iter().flat_map(func).collect()
    }
}
impl<'a, X, Y> Foldable<'a, Vec<X>, X, Y, Y> for VecEffect {
    fn fold(f: Vec<X>, init: Y, func: impl 'a + Fn(Y, X) -> Y + Send + Sync) -> Y {
        f.into_iter().fold(init, func)
    }
}
impl<X: Clone, Y: Clone> Productable<Vec<X>, Vec<Y>, Vec<(X, Y)>, X, Y> for VecEffect {
    fn product(fa: Vec<X>, fb: Vec<Y>) -> Vec<(X, Y)> {
        fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

impl<'a, E, FR, X, Y, T> Traverse<'a, Vec<X>, E, Vec<Y>, FR, X, Y> for VecEffect
    where
        E: F<Y> + Functor2Effect<'a, E, FR, FR, Y, Vec<Y>, Vec<Y>>,
        FR: F<Vec<Y>> + ApplicativeEffect<X=Vec<Y>, Fct=T>,
        T: Applicative<FR, Vec<Y>>{
    fn traverse(fa: Vec<X>,
                func: impl Fn(X) -> E + Send + Sync) -> FR {
        // Initialize the fold to the pure value of the resulting effect (Future, Option, IO, etc.)
        // Takes an empty vector of Y to start with
        let init: FR = pure::<FR>(empty::<Vec<Y>>());

        // Fold on the initial list (Vec<X>) and start with initial accumulator set to
        // A basic E<Vec<Y>> where E is the effect that will be returned from the specified
        // function (Vec, Future, Either, etc.).
        fold(fa, init, |y, x| {
            // The folding function should take this effect (Vec, Future, etc.) and
            // "combine" the results with the accumulated value.  This is what determines
            // whether the accumulated value turns into a "negative" result (like a None,
            // or a Future::fail(), or a Either::Err, etc.)

            // First, get the returned effect from the func call:
            let ret_ay = func(x);

            // Apply a map between the returned value and the accumulated value.  The
            // mapping function should know how to put the two together (they are the same
            // effect type, but they each hold a different type inside).
            fmap2(
                ret_ay,
                y,
                |fx, y| {
                    // This function adds the returned inner value onto the accumulating list
                    // inside the effect.  Functors know how to only allow this
                    // combination if both the accumulated effect and the returned
                    // effect both match up to "positive" values (like success or Some()).
                    // These next lines won't even get called unless that is the case.
                    let r = pure::<Vec<Y>>(fx);
                    combine(r, y)
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        let a = vec![3, 4, 5];
        let b = vec![6, 7, 8];

        let out = combine(a, b);
        assert_eq!(vec![3, 4, 5, 6, 7, 8], out);

        let a = vec![3, 4, 5];
        let b = vec![];

        let out = combine(a, b);
        assert_eq!(vec![3, 4, 5], out);

        let a = vec!["Hello".to_string()];
        let b = vec!["World".to_string()];

        let out = combine(a, b);
        assert_eq!(vec![format!("Hello"), format!("World")], out);
    }

    #[test]
    fn test_monoid() {
        let out: Vec<u32> = empty();
        assert!(out.is_empty());
    }

    #[test]
    fn test_applicative() {
        let out = <Vec::<u32> as ApplicativeEffect>::Fct::pure(3);
        assert_eq!(vec![3], out);
        let out: Vec<&str> = pure("test");
        assert_eq!(vec!["test"], out);
    }

    #[test]
    fn test_functor() {
        let out: Vec<u32> = pure(3);
        let res = fmap(out, |i| i + 4);
        assert_eq!(vec![7], res);

        let out: Vec<String> = pure(format!("Hello"));
        let res = fmap(out, |i| format!("{} World", i));
        assert_eq!(vec![format!("Hello World")], res);

        let out: Vec<String> = empty();
        let res = fmap(out, |i| format!("{} World", i));
        assert!(res.is_empty());

        let out1: Vec<u32> = pure(3);
        let out2: Vec<String> = pure(format!("Bowls"));
        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
        assert_eq!(vec![format!("7 Bowls of salad")], res);
    }

    #[test]
    fn test_monad() {
        let out: Vec<u32> = pure(3);
        let res = flat_map(out, |i| vec![i + 1, i + 2, i + 3]);
        assert_eq!(vec![4, 5, 6], res);

        let out: Vec<u32> = vec![3, 4];
        let res = flat_map(out, |i| vec![i + 1, i + 2, i + 3]);
        assert_eq!(vec![4, 5, 6, 5, 6, 7], res);

        let out: Vec<u32> = empty();
        let res = flat_map(out, |i| vec![i + 1, i + 2, i + 3]);
        assert!(res.is_empty());

        let out: Vec<u32> = vec![3, 4, 5];
        let res: Vec<u32> = flat_map(out, |_i| empty());
        assert!(res.is_empty());

        let out: Vec<u32> = vec![2, 3, 4];
        let res = fold(out, 0, |init, i| init + i);
        assert_eq!(9, res);

        let out: Vec<u32> = empty();
        let res = fold(out, 0, |init, i| init + i);
        assert_eq!(0, res);
    }

    #[test]
    fn test_product() {
        let out1: Vec<u32> = vec![2, 3];
        let out2: Vec<u32> = vec![4, 5];
        let res = product(out1, out2);
        assert_eq!(vec![(2, 4), (2, 5), (3, 4), (3, 5)], res);

        let out1: Vec<u32> = vec![2, 3];
        let out2: Vec<u32> = empty();
        let res = product(out1, out2);
        assert!(res.is_empty());
    }

    #[test]
    fn test_traverse() {
        let o = traverse(vec![2, 4, 6], |x| match x % 2 == 0 {
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
        let o = traverse(vec![2, 5, 6], |x| match x % 2 == 0 {
            true => Some(format!("{}", x)),
            false => None
        });
        assert!(o.is_none());
    }

}
