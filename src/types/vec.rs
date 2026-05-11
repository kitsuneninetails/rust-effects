use crate::prelude::typeclasses::*;

impl<A> Monoid for Vec<A> {
    fn empty() -> Self {
        vec![]
    }
}

impl<A> Semigroup for Vec<A> {
    fn combine(mut a: Self, b: Self) -> Self {
        a.extend(b);
        a
    }
}

impl<'a, T, U: Send> Functor<'a, T, U> for Vec<T> {
    type F = Vec<U>;
    fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F {
        m.into_iter().map(func).collect()
    }
}

impl<'a, T: Send, U: Send> Applicative<'a, T, U> for Vec<T> {
    fn pure(a: T) -> Self {
        vec![a]
    }
}

impl<'a, F, T, U: Send> ApplicativeFunctor<'a, F, T, U> for Vec<T>
where
    F: Fn(T) -> U,
    T: Send + Clone + 'a,
{
    type AOut = Vec<U>;
    type AFunc = Vec<F>;
    fn seq(m: Self, func: Self::AFunc) -> Self::AOut {
        func.iter()
            .flat_map(|f| m.iter().map(|i| f(i.clone())).collect::<Vec<U>>())
            .collect()
    }
}

impl<'a, T: Send, U: 'a + Send> Monad<'a, T, U> for Vec<T> {
    type M = Vec<U>;
    fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M {
        m.into_iter().flat_map(func).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_vec() {
        assert_eq!(Vec::<u32>::empty(), []);
    }
    #[test]
    fn test_identity_vec() {
        assert_eq!(combine(vec![0, 1, 2], Vec::<u32>::empty()), vec![0, 1, 2]);
        assert_eq!(combine(Vec::<u32>::empty(), vec![0, 1, 2]), vec![0, 1, 2]);
        assert!(combine(Vec::<u32>::empty(), Vec::<u32>::empty()).is_empty());
    }

    #[test]
    fn test_combine_vec() {
        assert_eq!(combine(vec![3], vec![4]), vec![3, 4]);
        assert_eq!(combine(vec![3], vec![]), vec![3]);
        assert_eq!(combine(vec![], vec![4]), vec![4]);
        assert_eq!(combine::<Vec<u32>>(vec![], vec![]), vec![]);
    }
    #[test]
    fn test_fmap_vec() {
        assert_eq!(fmap(vec![3, 4], |i| i + 4), vec![7, 8]);
        assert_eq!(fmap(vec![], |i: u32| i + 4), vec![]);
    }
    #[test]
    fn test_pure_vec() {
        assert_eq!(pure::<Vec<_>, _>(2), vec![2]);
    }

    #[test]
    fn test_seq_vec() {
        let func: Vec<Box<dyn Fn(u32) -> u32>> = vec![Box::new(|x| x + 2), Box::new(|x| x + 3)];
        assert_eq!(seq(vec![3u32, 4, 5], func), vec![5, 6, 7, 6, 7, 8]);
    }

    fn empty_if_even<'a, M: Monad<'a, u32> + Monoid + Applicative<'a, u32>>(input: String) -> M {
        if input.len() % 2 == 0 {
            M::empty()
        } else {
            M::pure(input.len() as u32)
        }
    }

    #[test]
    fn test_bind_vec() {
        assert_eq!(
            bind(vec!["dog".to_string(), "crow".to_string()], empty_if_even),
            vec![3]
        );
        assert_eq!(bind(vec![], empty_if_even), vec![]);
    }

    fn add4(x: u32) -> u32 {
        x + 4
    }

    #[test]
    fn test_lift1_vec() {
        let new_func = lift_m1::<Vec<_>, _, _>(add4);
        assert_eq!(new_func(vec![2, 3, 4]), vec![6, 7, 8]);
        assert!(new_func(vec![]).is_empty());
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }
    #[test]
    fn test_lift2_vec() {
        let new_func = lift_m2::<Vec<_>, _, _, _, _>(add2);
        assert_eq!(
            new_func(vec![1, 2, 3], vec![4, 5, 6]),
            vec![5, 6, 7, 6, 7, 8, 7, 8, 9]
        );
        assert!(new_func(vec![1, 2, 3], vec![]).is_empty());
        assert!(new_func(vec![], vec![4, 5, 6]).is_empty());
        assert!(new_func(vec![], vec![]).is_empty());
    }
}
