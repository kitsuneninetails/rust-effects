use crate::prelude::typeclasses::*;

impl<A: Monoid> Monoid for Option<A> {
    fn empty() -> Self {
        None
    }
    fn empty_m() -> Self {
        None
    }
}

impl<A: Monoid> Semigroup for Option<A> {
    fn combine(a: Self, b: Self) -> Self {
        match (a, b) {
            (Some(t), Some(u)) => Some(combine(t, u)),
            (Some(t), None) => Some(A::combine(t, A::empty())),
            (None, Some(u)) => Some(A::combine(u, A::empty())),
            (None, None) => None,
        }
    }
    fn combine_m(a: Self, b: Self) -> Self {
        match (a, b) {
            (Some(t), Some(u)) => Some(combine_m(t, u)),
            (Some(t), None) => Some(A::combine_m(t, A::empty_m())),
            (None, Some(u)) => Some(A::combine_m(u, A::empty_m())),
            (None, None) => None,
        }
    }
}

impl<T, U> Functor<U> for Option<T> {
    type FuncT = T;
    type FunctorOut = Option<U>;
    fn fmap(m: Self, func: impl FnOnce(T) -> U + Send) -> Self::FunctorOut {
        m.map(func)
    }
}

impl<T, U> Applicative<U> for Option<T> {
    type AppT = T;
    fn pure(a: T) -> Self {
        Some(a)
    }
}

impl<F, T, U> ApplicativeFunctor<F, U> for Option<T>
where
    F: Fn(T) -> U,
    T: Send + Clone,
{
    type AppFuncT = T;
    type AppFuncOut = Option<U>;
    type AppFuncFn = Option<F>;
    fn seq(m: Self, func: Self::AppFuncFn) -> Self::AppFuncOut {
        func.and_then(|f| m.map(|t| f(t)))
    }
}

// impl<T: Send, U: Send> Monad for HktOption<T, U> {
//     type MonadU = U;
//     type MonadT = T;
//     type MonadOut = Option<U>;
//     fn bind(m: Self, func: impl FnOnce(Self::MonadT) -> Self::MonadOut + Send) -> Self::MonadOut {
//         m.and_then(func)
//     }
// }
impl<T: Send, U: Send> Monad<U> for Option<T> {
    type MonadT = T;
    type MonadOut = Option<U>;
    fn bind(m: Self, func: impl FnOnce(Self::MonadT) -> Self::MonadOut + Send) -> Self::MonadOut {
        m.and_then(func)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_option() {
        assert_eq!(Option::<u32>::empty(), None);
        assert_eq!(empty::<Option<u32>>(), None);
        assert_eq!(empty_m::<Option<u32>>(), None);
    }

    #[test]
    fn test_identity_option() {
        assert_eq!(combine(Some(3u32), empty()), Some(3u32));
        assert_eq!(combine(empty(), Some(3u32),), Some(3u32));
        assert_eq!(combine(empty::<Option<u32>>(), empty::<Option<_>>()), None);
        assert_eq!(combine_m(Some(3u32), empty_m()), Some(3u32));
        assert_eq!(combine_m(empty_m(), Some(3u32),), Some(3u32));
        assert_eq!(combine_m(empty_m::<Option<u32>>(), empty_m()), None);
    }
    #[test]
    fn test_combine_option() {
        assert_eq!(combine(Some(3), Some(4)), Some(7));
        assert_eq!(combine(Some(3), None), Some(3));
        assert_eq!(combine(None, Some(4)), Some(4));
        assert_eq!(combine(Option::<u32>::None, None), None);
        assert_eq!(combine_m(Some(3), Some(4)), Some(12));
        assert_eq!(combine_m(Some(3), None), Some(3));
        assert_eq!(combine_m(None, Some(4)), Some(4));
        assert_eq!(combine_m(Option::<u32>::None, None), None);
    }

    #[test]
    fn test_fmap_option() {
        assert_eq!(fmap(Some(3), |i| i + 4), Some(7));
        assert_eq!(fmap(None, |i: u32| i + 4), None);
    }

    #[test]
    fn test_pure_option() {
        assert_eq!(pure::<Option<_>>(2), Some(2));
    }

    #[test]
    fn test_seq_option() {
        let func = Some(|x| x + 2);
        let func_none = func.filter(|_| false);
        assert_eq!(seq(Some(3), func), Some(5));
        assert!(seq(Some(3), func_none).is_none());
        assert!(seq(None, func).is_none());
        assert!(seq(None, func_none).is_none());
    }

    fn empty_if_even<M: Monad<u32, MonadT = u32> + Monoid + Applicative<u32>>(input: String) -> M {
        if input.len() % 2 == 0 {
            M::empty()
        } else {
            M::pure(input.len() as u32)
        }
    }

    #[test]
    fn test_bind_option() {
        assert_eq!(bind(Some("dog".to_string()), empty_if_even), Some(3));
        assert_eq!(bind(Some("crow".to_string()), empty_if_even), None);
        assert_eq!(bind(None::<String>, empty_if_even), None);
    }

    fn add4(x: u32) -> u32 {
        x + 4
    }
    #[test]
    fn test_lift1_option() {
        let new_func = lift_m1::<Option<_>, _>(add4);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }
    #[test]
    fn test_lift1_option_closure() {
        let add4_closure = |x: u32| x + 4;
        let new_func = lift_m1::<Option<_>, _>(add4_closure);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }
    #[test]
    fn test_lift1_option_closure_as_param() {
        let new_func = lift_m1::<Option<_>, _>(|x: u32| x + 4);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test]
    fn test_lift2_option() {
        let new_func = lift_m2::<Option<_>, _, _>(add2);
        assert_eq!(new_func(Some(3), Some(4)), Some(7));
        assert!(new_func(Some(3), None).is_none());
        assert!(new_func(None, Some(4)).is_none());
        assert!(new_func(None, None).is_none());
    }
}
