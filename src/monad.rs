#![allow(refining_impl_trait)]
use crate::{CFuture, applicative::Applicative};
use futures::FutureExt;

pub trait Monad<'a, T, U = ()>: Sized + Applicative<'a, T, U, F = Self::M> {
    type M: Monad<'a, U> + Send;
    fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M;
    fn lift_m1<S: Send + 'a, In: Monad<'a, S, T, M = Self>>(
        func: impl Fn(S) -> T + Send + Clone + 'a,
    ) -> impl Fn(In) -> Self {
        move |n: In| In::fmap(n, func.clone())
    }
    fn lift_m2<
        S1: Send + Clone + 'a,
        In1: Monad<'a, S1, T, M = Self> + Send + 'a,
        S2: Send + 'a,
        In2: Monad<'a, S2, T, M = Self> + Send + Clone + 'a,
    >(
        func: impl Fn(S1, S2) -> T + Send + Clone + 'a,
    ) -> impl Fn(In1, In2) -> Self {
        move |in1: In1, in2: In2| {
            let fnc_tmp = func.clone();
            In1::bind(in1, move |s1: S1| {
                let tmp = fnc_tmp.clone();
                In2::fmap(in2.clone(), move |s2: S2| tmp.clone()(s1.clone(), s2))
            })
        }
    }
}

impl<'a, T: Send + 'a, U: Send + 'a> Monad<'a, T, U> for Option<T> {
    type M = Option<U>;
    fn bind(m: Self, func: impl FnOnce(T) -> Self::M + Send + 'a) -> Self::M {
        m.and_then(func)
    }
}

impl<'a, T, U: Send, E: Send> Monad<'a, T, U> for Result<T, E> {
    type M = Result<U, E>;
    fn bind(m: Self, func: impl FnOnce(T) -> Self::M + Send + 'a) -> Self::M {
        m.and_then(func)
    }
}

impl<'a, T: Send, U: 'a + Send> Monad<'a, T, U> for Vec<T> {
    type M = Vec<U>;
    fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M {
        m.into_iter().flat_map(func).collect()
    }
}

impl<'a, T: Send + Sync + Clone + 'a, U: Send + Sync + Clone + 'a> Monad<'a, T, U>
    for CFuture<'a, T>
{
    type M = CFuture<'a, U>;
    fn bind(m: Self, func: impl FnOnce(T) -> Self::M + Send + 'a) -> Self::M {
        CFuture::new_fut(m.then(func))
    }
}

pub fn bind<'a, T: Send + 'a, U: Send + 'a, M: Monad<'a, T, U>>(
    m: M,
    func: impl Fn(T) -> M::M + Send + 'a,
) -> M::M {
    M::bind(m, func)
}

pub fn lift_m1<'a, In: Monad<'a, S, T>, S: Send + 'a, T>(
    func: impl Fn(S) -> T + Send + Clone + 'a,
) -> impl Fn(In) -> In::M {
    In::M::lift_m1(func)
}

pub fn lift_m2<
    'a,
    In1: Monad<'a, S1, T> + Send + Clone + 'a,
    In2: Monad<'a, S2, T, M = In1::M> + Send + Clone + 'a,
    S2: Send + Clone + 'a,
    S1: Send + Clone + 'a,
    T,
>(
    func: impl Fn(S1, S2) -> T + Send + Clone + 'a,
) -> impl Fn(In1, In2) -> In1::M {
    In1::M::lift_m2(func)
}

#[cfg(test)]
mod test {
    use crate::{
        applicative::{Applicative, pure},
        monoid::Monoid,
    };

    use super::*;

    fn empty_if_even<'a, M: Monad<'a, u32> + Monoid + Applicative<'a, u32>>(input: String) -> M {
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

    #[test]
    fn test_bind_result() {
        assert_eq!(bind(Ok::<_, u32>("dog".to_string()), empty_if_even), Ok(3));
        assert_eq!(
            bind(Ok::<_, u32>("crow".to_string()), empty_if_even),
            Err(0)
        );
        assert_eq!(bind(Err::<String, _>(2), empty_if_even), Err(2));
    }

    #[test]
    fn test_bind_vec() {
        assert_eq!(
            bind(vec!["dog".to_string(), "crow".to_string()], empty_if_even),
            vec![3]
        );
        assert_eq!(bind(vec![], empty_if_even), vec![]);
    }

    #[tokio::test]
    async fn test_bind_future() {
        assert_eq!(
            bind(pure::<CFuture<_>, _>("dog".to_string()), empty_if_even).await,
            3
        );
        assert_eq!(
            bind(pure::<CFuture<_>, _>("crow".to_string()), empty_if_even).await,
            0
        );
    }

    fn add4(x: u32) -> u32 {
        x + 4
    }

    #[test]
    fn test_lift1_option() {
        let new_func = lift_m1::<Option<_>, _, _>(add4);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }
    #[test]
    fn test_lift1_result() {
        let new_func = lift_m1::<Result<_, ()>, _, _>(add4);
        assert_eq!(new_func(Ok(3)), Ok(7));
        assert!(new_func(Err(())).is_err());
    }
    #[test]
    fn test_lift1_vec() {
        let new_func = lift_m1::<Vec<_>, _, _>(add4);
        assert_eq!(new_func(vec![2, 3, 4]), vec![6, 7, 8]);
        assert!(new_func(vec![]).is_empty());
    }
    #[tokio::test]
    async fn test_lift1_future() {
        let new_func = lift_m1::<CFuture<_>, _, _>(add4);
        assert_eq!(new_func(CFuture::lazy(3)).await, 7);
    }
    #[test]
    fn test_lift1_option_closure() {
        let add4_closure = |x: u32| x + 4;
        let new_func = lift_m1::<Option<_>, _, _>(add4_closure);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }
    #[test]
    fn test_lift1_option_closure_as_param() {
        let new_func = lift_m1::<Option<_>, _, _>(|x: u32| x + 4);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test]
    fn test_lift2_option() {
        let new_func = lift_m2::<Option<_>, _, _, _, _>(add2);
        assert_eq!(new_func(Some(3), Some(4)), Some(7));
        assert!(new_func(Some(3), None).is_none());
        assert!(new_func(None, Some(4)).is_none());
        assert!(new_func(None, None).is_none());
    }
    #[test]
    fn test_lift2_result() {
        let new_func = lift_m2::<Result<_, ()>, _, _, _, _>(add2);
        assert_eq!(new_func(Ok(3), Ok(4)), Ok(7));
        assert!(new_func(Ok(3), Err(())).is_err());
        assert!(new_func(Err(()), Ok(4)).is_err());
        assert!(new_func(Err(()), Err(())).is_err());
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
    #[tokio::test]
    async fn test_lift2_future() {
        let new_func = lift_m2::<CFuture<_>, _, _, _, _>(add2);
        assert_eq!(new_func(CFuture::lazy(3), CFuture::lazy(4)).await, 7);
    }
}
