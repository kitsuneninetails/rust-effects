use super::typeclasses::{F,
                         applicative::*,
                         functor::*,
                         monad::*,
                         monoid::*,
                         product::*,
                         semigroup::*,
                         traverse::*};

impl<X> F<X> for Vec<X> {}

#[derive(Clone)]
pub struct VecEffect;
pub const VEC_SG: VecEffect = VecEffect;
pub const VEC_EV: &VecEffect = &VecEffect;

impl<X> Semigroup<Vec<X>, Vec<X>, Vec<X>> for VecEffect {
    fn combine(self, a: Vec<X>, b: Vec<X>) -> Vec<X> {
        let mut ret = a;
        ret.extend(b);
        ret
    }
}

impl<T> Monoid<Vec<T>> for VecEffect {
    fn empty(&self) -> Vec<T> {
        vec![]
    }
}
impl<T> Applicative<Vec<T>, T> for VecEffect {
    fn pure(&self, x: T) -> Vec<T> {
        vec![x]
    }
}
impl<X, Y> Functor<Vec<X>, Vec<Y>, X, Y> for VecEffect {
    fn fmap(&self, f: Vec<X>, func: fn(X) -> Y) -> Vec<Y> {
        f.into_iter().map(func).collect()
    }
}
impl<X, Y, Z> Functor2<Vec<X>, Vec<Y>, Vec<Z>, X, Y, Z> for VecEffect {
    fn fmap2(&self, fa: Vec<X>, fb: Vec<Y>, func: fn(&X, &Y) -> Z) -> Vec<Z> {
        fa.into_iter().flat_map(|i| {
            let ret: Vec<Z> = fb.iter().map(|j| func(&i, j)).collect();
            ret
        }).collect()
    }
}
impl<X, Y> Monad<Vec<X>, Vec<Y>, X, Y> for VecEffect {
    fn flat_map(&self, f: Vec<X>, func: fn(X) -> Vec<Y>) -> Vec<Y> {
        f.into_iter().flat_map(func).collect()
    }
}
impl<X, Y> Foldable<Vec<X>, X, Y, Y> for VecEffect {
    fn fold(&self, f: Vec<X>, init: Y, func: fn(Y, X) -> Y) -> Y {
        f.into_iter().fold(init, func)
    }
}
impl<X: Clone, Y: Clone> Productable<Vec<X>, Vec<Y>, Vec<(X, Y)>, X, Y> for VecEffect {
    fn product(&self, fa: Vec<X>, fb: Vec<Y>) -> Vec<(X, Y)> {
        fmap2(VEC_EV, fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

fn trv_fn<X, Y, T, E, FR>((acc, eff, func): (FR, &T, fn(X) -> E), item: X) -> (FR, &T, fn(X) -> E)
    where
        Y: Clone,
        T: Applicative<FR, Vec<Y>> + Functor2<E, FR, FR, Y, Vec<Y>, Vec<Y>>,
        FR:  F<Vec<Y>>,
        E: F<Y> {
    // The folding function should take this effect (Vec, Future, etc.) and
    // "combine" the results with the accumulated value.  This is what determines
    // whether the accumulated value turns into a "negative" result (like a None,
    // or a Future::fail(), or a Either::Err, etc.)

    // First, get the returned effect from the func call:
    let ret_ay = func(item);

    // Apply a map between the returned value and the accumulated value.  The
    // mapping function should know how to put the two together (they are the same
    // effect type, but they each hold a different type inside).
    let new_acc = fmap2(
        eff,
        ret_ay,
        acc,
        |fx, y| {
            // This function adds the returned inner value onto the accumulating list
            // inside the effect.  Functors know how to only allow this
            // combination if both the accumulated effect and the returned
            // effect both match up to "positive" values (like success or Some()).
            // These next lines won't even get called unless that is the case.
            let r = pure(VEC_EV, fx.clone());
            combine(VEC_EV.clone(), r, y.clone())
        });
    (new_acc, eff, func)
}

impl<E: F<Y>, FR: F<Vec<Y>>, X, Y: Clone> Traverse<Vec<X>, E, Vec<Y>, FR, X, Y> for VecEffect {
    fn traverse(&self,
                e_effect: &(impl Applicative<FR, Vec<Y>> + Functor2<E, FR, FR, Y, Vec<Y>, Vec<Y>>),
                fa: Vec<X>,
                func: fn(X) -> E) -> FR {
        // Initialize the fold to the pure value of the resulting effect (Future, Option, IO, etc.)
        // Takes an empty vector of Y to start with
        let init: FR = pure(e_effect, empty(VEC_EV));

        // Fold on the initial list (Vec<X>) and start with initial accumulator set to
        // A basic E<Vec<Y>> where E is the effect that will be returned from the specified
        // function (Vec, Future, Either, etc.).
        fold(
            VEC_EV,
            fa,
            (init, e_effect, func),
            trv_fn).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::*;

    #[test]
    fn test_semigroup() {
        let a = vec![3, 4, 5];
        let b = vec![6, 7, 8];

        let out = combine(VEC_SG, a, b);
        assert_eq!(vec![3, 4, 5, 6, 7, 8], out);

        let a = vec![3, 4, 5];
        let b = vec![];

        let out = combine(VEC_SG, a, b);
        assert_eq!(vec![3, 4, 5], out);

        let a = vec!["Hello".to_string()];
        let b = vec!["World".to_string()];

        let out = combine(VEC_SG, a, b);
        assert_eq!(vec![format!("Hello"), format!("World")], out);
    }

    #[test]
    fn test_monoid() {
        let out: Vec<u32> = empty(VEC_EV);
        assert!(out.is_empty());
    }

    #[test]
    fn test_applicative() {
        let out = VEC_EV.pure(3);
        assert_eq!(vec![3], out);
        let out = pure(VEC_EV, "test");
        assert_eq!(vec!["test"], out);
    }

    #[test]
    fn test_functor() {
        let out: Vec<u32> = pure(VEC_EV, 3);
        let res = fmap(VEC_EV, out, |i| i + 4);
        assert_eq!(vec![7], res);

        let out: Vec<String> = pure(VEC_EV, format!("Hello"));
        let res = fmap(VEC_EV, out, |i| format!("{} World", i));
        assert_eq!(vec![format!("Hello World")], res);

        let out: Vec<String> = empty(VEC_EV);
        let res = fmap(VEC_EV, out, |i| format!("{} World", i));
        assert!(res.is_empty());

        let out1: Vec<u32> = pure(VEC_EV, 3);
        let out2: Vec<String> = pure(VEC_EV, format!("Bowls"));
        let res = fmap2(VEC_EV, out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
        assert_eq!(vec![format!("7 Bowls of salad")], res);
    }

    #[test]
    fn test_monad() {
        let out: Vec<u32> = pure(VEC_EV, 3);
        let res = flat_map(VEC_EV, out, |i| vec![i + 1, i + 2, i + 3]);
        assert_eq!(vec![4, 5, 6], res);

        let out: Vec<u32> = vec![3, 4];
        let res = flat_map(VEC_EV, out, |i| vec![i + 1, i + 2, i + 3]);
        assert_eq!(vec![4, 5, 6, 5, 6, 7], res);

        let out: Vec<u32> = empty(VEC_EV);
        let res = flat_map(VEC_EV, out, |i| vec![i + 1, i + 2, i + 3]);
        assert!(res.is_empty());

        let out: Vec<u32> = vec![3, 4, 5];
        let res: Vec<u32> = flat_map(VEC_EV, out, |_i| empty(VEC_EV));
        assert!(res.is_empty());

        let out: Vec<u32> = vec![2, 3, 4];
        let res = fold(VEC_EV, out, 0, |init, i| init + i);
        assert_eq!(9, res);

        let out: Vec<u32> = empty(VEC_EV);
        let res = fold(VEC_EV, out, 0, |init, i| init + i);
        assert_eq!(0, res);
    }

    #[test]
    fn test_product() {
        let out1: Vec<u32> = vec![2, 3];
        let out2: Vec<u32> = vec![4, 5];
        let res = product(VEC_EV, out1, out2);
        assert_eq!(vec![(2, 4), (2, 5), (3, 4), (3, 5)], res);

        let out1: Vec<u32> = vec![2, 3];
        let out2: Vec<u32> = empty(VEC_EV);
        let res = product(VEC_EV, out1, out2);
        assert!(res.is_empty());
    }

    #[test]
    fn test_traverse() {
        let o = traverse(VEC_EV, OP_EV, vec![2, 4, 6], |x| match x % 2 == 0 {
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
        let o = traverse(VEC_EV, OP_EV, vec![2, 5, 6], |x| match x % 2 == 0 {
            true => Some(format!("{}", x)),
            false => None
        });
        assert!(o.is_none());
    }

}
