use crate::{CFuture, applicative::Applicative, functor::Functor};

pub trait ApplicativeFunctor<'a, F: Fn(T) -> U, T, U = ()>: Applicative<'a, T, U> {
    type AOut: Applicative<'a, U>;
    type AFunc: Functor<'a, F, U, F = Self::AOut>;

    fn seq(m: Self, func: Self::AFunc) -> Self::AOut;
}

impl<'a, F, T, U> ApplicativeFunctor<'a, F, T, U> for Option<T>
where
    F: Fn(T) -> U,
    T: Send + Clone + 'a,
{
    type AOut = Option<U>;
    type AFunc = Option<F>;
    fn seq(m: Self, func: Self::AFunc) -> Self::AOut {
        func.and_then(|f| m.map(|t| f(t)))
    }
}

impl<'a, F, T, U, E> ApplicativeFunctor<'a, F, T, U> for Result<T, E>
where
    F: Fn(T) -> U,
    T: Send + Clone + 'a,
{
    type AOut = Result<U, E>;
    type AFunc = Result<F, E>;
    fn seq(m: Self, func: Self::AFunc) -> Self::AOut {
        func.and_then(|f| m.map(|t| f(t)))
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

impl<'a, F, T, U> ApplicativeFunctor<'a, F, T, U> for CFuture<'a, T>
where
    F: Fn(T) -> U + Sync + Send + Clone + 'a,
    T: Send + Clone + Sync + 'a,
    U: Send + Clone + Sync + 'a,
{
    type AOut = CFuture<'a, U>;
    type AFunc = CFuture<'a, F>;
    fn seq(m: Self, func: Self::AFunc) -> Self::AOut {
        let in_f = func;
        CFuture::new_fut(async { in_f.await(m.await) })
    }
}

pub fn seq<'a, N, M, T, U>(m: N, func: N::AFunc) -> N::AOut
where
    N: ApplicativeFunctor<'a, M, T, U>,
    M: Fn(T) -> U,
{
    N::seq(m, func)
}

#[cfg(test)]
mod test {
    use crate::{CFuture, applicative::pure, applicative_functor::seq};

    #[test]
    fn test_pure_option() {
        assert_eq!(pure::<Option<_>, _>(2), Some(2));
    }
    #[test]
    fn test_pure_result() {
        assert_eq!(pure::<Result<_, ()>, _>(2), Ok(2));
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
    #[test]
    fn test_seq_result() {
        let func = Some(|x| x + 2);
        let func_none = func.filter(|_| false);
        assert_eq!(seq(Ok(3), func.ok_or(())), Ok(5));
        assert!(seq(Ok(3), func_none.ok_or(())).is_err());
        assert!(seq(Err(()), func.ok_or(())).is_err());
        assert!(seq(Err(()), func_none.ok_or(())).is_err());
    }
    #[test]
    fn test_seq_vec() {
        let func: Vec<Box<dyn Fn(u32) -> u32>> = vec![Box::new(|x| x + 2), Box::new(|x| x + 3)];
        assert_eq!(seq(vec![3u32, 4, 5], func), vec![5, 6, 7, 6, 7, 8]);
    }
    #[tokio::test]
    async fn test_seq_future() {
        let func = CFuture::lazy(|x| x + 4);
        assert_eq!(seq(CFuture::lazy(3), func).await, 7);
    }
}
