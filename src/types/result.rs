use crate::prelude::typeclasses::*;

impl<A: Monoid, E: Monoid> Monoid for Result<A, E> {
    fn empty() -> Self {
        Err(E::empty())
    }
    fn empty_m() -> Self {
        Err(E::empty_m())
    }
}

impl<A: Monoid, E: Monoid> Semigroup for Result<A, E> {
    fn combine(a: Self, b: Self) -> Self {
        match (a, b) {
            (Ok(t), Ok(u)) => Ok(combine(t, u)),
            (Ok(t), Err(_)) => Ok(A::combine(t, A::empty())),
            (Err(_), Ok(u)) => Ok(A::combine(u, A::empty())),
            (Err(e), Err(e2)) => Err(combine(e, e2)),
        }
    }
    fn combine_m(a: Self, b: Self) -> Self {
        match (a, b) {
            (Ok(t), Ok(u)) => Ok(combine_m(t, u)),
            (Ok(t), Err(_)) => Ok(A::combine_m(t, A::empty_m())),
            (Err(_), Ok(u)) => Ok(A::combine_m(u, A::empty_m())),
            (Err(e), Err(e2)) => Err(combine_m(e, e2)),
        }
    }
}

impl<T, U, E> Functor<T, U> for Result<T, E> {
    type FunctorOut = Result<U, E>;
    fn fmap(m: Self, func: impl FnOnce(T) -> U + Send) -> Self::FunctorOut {
        m.map(func)
    }
}

impl<T, U, E> Applicative<T, U> for Result<T, E> {
    fn pure(a: T) -> Self {
        Ok(a)
    }
}

impl<F, T, U, E> ApplicativeFunctor<F, T, U> for Result<T, E>
where
    F: Fn(T) -> U,
    T: Send + Clone,
{
    type AppFuncOut = Result<U, E>;
    type AppFuncFn = Result<F, E>;
    fn seq(m: Self, func: Self::AppFuncFn) -> Self::AppFuncOut {
        func.and_then(|f| m.map(|t| f(t)))
    }
}

impl<T, U: Send, E: Send> Monad<T, U> for Result<T, E> {
    type MonadOut = Result<U, E>;
    fn bind(m: Self, func: impl FnOnce(T) -> Self::MonadOut + Send) -> Self::MonadOut {
        m.and_then(func)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_result() {
        assert_eq!(Result::<u32, u32>::empty(), Err(0));
        assert_eq!(Result::<u32, u32>::empty_m(), Err(1));
    }

    #[test]
    fn test_identity_result() {
        assert_eq!(combine(Ok(3u32), Result::<u32, ()>::empty()), Ok(3u32));
        assert_eq!(combine(Result::<u32, ()>::empty(), Ok(3u32)), Ok(3u32));
        assert_eq!(
            combine(Result::<u32, u32>::empty(), Result::<u32, u32>::empty()),
            Err(0u32)
        );
        assert_eq!(combine_m(Ok(3u32), Result::<u32, u32>::empty_m()), Ok(3u32));
        assert_eq!(combine_m(Result::<u32, u32>::empty_m(), Ok(3u32)), Ok(3u32));
        assert_eq!(
            combine_m(Result::<u32, u32>::empty_m(), Result::<u32, u32>::empty_m()),
            Err(1u32)
        );
    }

    #[test]
    fn test_combine_result() {
        assert_eq!(combine(Ok::<_, u32>(3), Ok(4)), Ok(7));
        assert_eq!(combine(Ok(3), Err(4)), Ok(3));
        assert_eq!(combine(Err(3), Ok(4)), Ok(4));
        assert_eq!(combine(Err::<u32, _>(3), Err(4)), Err(7));
        assert_eq!(combine_m(Ok::<_, u32>(3), Ok(4)), Ok(12));
        assert_eq!(combine_m(Ok(3), Err(4)), Ok(3));
        assert_eq!(combine_m(Err(3), Ok(4)), Ok(4));
        assert_eq!(combine_m(Err::<u32, _>(3), Err(4)), Err(12));
    }
    #[test]
    fn test_fmap_result() {
        assert_eq!(fmap(Ok::<_, u32>(3), |i| i + 4), Ok(7));
        assert_eq!(fmap(Err(3), |i: u32| i + 4), Err(3));
    }
    #[test]
    fn test_pure_result() {
        assert_eq!(pure::<Result<_, ()>, _>(2), Ok(2));
    }

    #[test]
    fn test_seq_result() {
        let func = Some(|x| x + 2);
        let func_none = func.filter(|_| false);
        assert_eq!(seq(Ok(3), func.ok_or(())), Ok(5));
        assert!(seq(Ok(3), func_none.ok_or(())).is_err());
        assert!(seq(Err(()), func.ok_or(())).is_err());
        assert!(seq(Err(()), func_none.ok_or(())).is_err());
    }

    fn empty_if_even<M: Monad<u32> + Monoid + Applicative<u32>>(input: String) -> M {
        if input.len() % 2 == 0 {
            M::empty()
        } else {
            M::pure(input.len() as u32)
        }
    }

    #[test]
    fn test_bind_result() {
        assert_eq!(bind(Ok::<_, u32>("dog".to_string()), empty_if_even), Ok(3));
        assert_eq!(
            bind(Ok::<_, u32>("crow".to_string()), empty_if_even),
            Err(0)
        );
        assert_eq!(bind(Err::<String, _>(2), empty_if_even), Err(2));
    }

    fn add4(x: u32) -> u32 {
        x + 4
    }
    #[test]
    fn test_lift1_result() {
        let new_func = lift_m1::<Result<_, ()>, _, _>(add4);
        assert_eq!(new_func(Ok(3)), Ok(7));
        assert!(new_func(Err(())).is_err());
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }
    #[test]
    fn test_lift2_result() {
        let new_func = lift_m2::<Result<_, ()>, _, _, _, _>(add2);
        assert_eq!(new_func(Ok(3), Ok(4)), Ok(7));
        assert!(new_func(Ok(3), Err(())).is_err());
        assert!(new_func(Err(()), Ok(4)).is_err());
        assert!(new_func(Err(()), Err(())).is_err());
    }
}
