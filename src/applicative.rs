use crate::{CFuture, functor::Functor};
use futures::future::lazy;

pub trait Applicative<'a, T, U = ()>: Functor<'a, T, U> {
    //type A: Applicative<'a, U, ()> + Send;
    //type AFunc: Applicative<'a, Fn(T) -> U> + Send;
    fn pure(a: T) -> Self;
}

impl<'a, T, U> Applicative<'a, T, U> for Option<T> {
    fn pure(a: T) -> Self {
        Some(a)
    }
}

impl<'a, T, U, E> Applicative<'a, T, U> for Result<T, E> {
    fn pure(a: T) -> Self {
        Ok(a)
    }
}

impl<'a, T: Send, U: Send> Applicative<'a, T, U> for Vec<T> {
    fn pure(a: T) -> Self {
        vec![a]
    }
}

impl<'a, T: Send + Sync + Clone + 'a, U: Send + Sync + Clone + 'a> Applicative<'a, T, U>
    for CFuture<'a, T>
{
    fn pure(a: T) -> Self {
        CFuture::new_fut(lazy(move |_| a))
    }
}

pub fn pure<'a, A: Applicative<'a, T>, T>(t: T) -> A {
    A::pure(t)
}

#[cfg(test)]
mod test {
    use crate::{CFuture, applicative::pure};

    #[test]
    fn test_pure_option() {
        assert_eq!(pure::<Option<_>, _>(2), Some(2));
    }
    #[test]
    fn test_pure_result() {
        assert_eq!(pure::<Result<_, ()>, _>(2), Ok(2));
    }
    #[test]
    fn test_pure_vec() {
        assert_eq!(pure::<Vec<_>, _>(2), vec![2]);
    }
    #[tokio::test]
    async fn test_pure_future() {
        assert_eq!(pure::<CFuture<_>, _>(2).await, 2);
    }
}
