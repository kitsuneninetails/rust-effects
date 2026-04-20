use crate::CFuture;
use futures::FutureExt;

pub trait Functor<'a, T, U = ()> {
    type F: Functor<'a, U>;
    fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F;
}

impl<'a, T, U> Functor<'a, T, U> for Option<T> {
    type F = Option<U>;
    fn fmap(m: Self, func: impl FnOnce(T) -> U + Send + 'a) -> Self::F {
        m.map(func)
    }
}

impl<'a, T, U, E> Functor<'a, T, U> for Result<T, E> {
    type F = Result<U, E>;
    fn fmap(m: Self, func: impl FnOnce(T) -> U + Send + 'a) -> Self::F {
        m.map(func)
    }
}

impl<'a, T, U: Send> Functor<'a, T, U> for Vec<T> {
    type F = Vec<U>;
    fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F {
        m.into_iter().map(func).collect()
    }
}

impl<'a, T: Send + Sync + Clone + 'a, U: Send + Sync + Clone + 'a> Functor<'a, T, U>
    for CFuture<'a, T>
{
    type F = CFuture<'a, U>;
    fn fmap(m: Self, func: impl FnOnce(T) -> U + Send + 'a) -> Self::F {
        CFuture::new_fut(m.map(func))
    }
}

pub fn fmap<'a, T, U, A: Functor<'a, T, U>>(a: A, func: impl Fn(T) -> U + Send + 'a) -> A::F {
    A::fmap(a, func)
}

#[cfg(test)]
mod test {
    use crate::{CFuture, applicative::pure, functor::fmap};

    #[test]
    fn test_fmap_option() {
        assert_eq!(fmap(Some(3), |i| i + 4), Some(7));
        assert_eq!(fmap(None, |i: u32| i + 4), None);
    }
    #[test]
    fn test_fmap_result() {
        assert_eq!(fmap(Ok::<_, u32>(3), |i| i + 4), Ok(7));
        assert_eq!(fmap(Err(3), |i: u32| i + 4), Err(3));
    }
    #[test]
    fn test_fmap_vec() {
        assert_eq!(fmap(vec![3, 4], |i| i + 4), vec![7, 8]);
        assert_eq!(fmap(vec![], |i: u32| i + 4), vec![]);
    }
    #[tokio::test]
    async fn test_fmap_future() {
        assert_eq!(fmap(pure::<CFuture<_>, _>(3), |i| i + 4).await, 7);
    }
}
