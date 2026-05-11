use crate::prelude::typeclasses::*;
use futures::future::{Shared, lazy};
use futures_util::{FutureExt, future::BoxFuture};

#[derive(Clone)]
pub struct CFuture<'a, A: Clone + Send + Sync> {
    inner: Shared<BoxFuture<'a, A>>,
}

impl<'a, A: Clone + Send + Sync + 'a> CFuture<'a, A> {
    pub fn lazy(val: A) -> Self {
        CFuture::new_fut(lazy(move |_| val))
    }

    pub fn new_fut(inner: impl Future<Output = A> + Send + 'a) -> Self {
        CFuture {
            inner: inner.boxed().shared(),
        }
    }
}

impl<'a, A: Clone + Send + Sync> Future for CFuture<'a, A> {
    type Output = A;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

unsafe impl<'a, A: Clone + Send + Sync> Send for CFuture<'a, A> {}
unsafe impl<'a, A: Clone + Send + Sync> Sync for CFuture<'a, A> {}

impl<'a, A: Monoid + Send + Sync + Clone + 'a> Monoid for CFuture<'a, A> {
    fn empty() -> Self {
        CFuture::new_fut(lazy(|_| A::empty()))
    }
    fn empty_m() -> Self {
        CFuture::new_fut(lazy(|_| A::empty_m()))
    }
}

impl<'a, A: 'a + Send + Sync + Clone + Semigroup> Semigroup for CFuture<'a, A> {
    fn combine(a: Self, b: Self) -> Self {
        let new_fut = a
            .inner
            .then(move |a_res| b.inner.map(move |b_res| A::combine(a_res, b_res)));
        CFuture::new_fut(new_fut)
    }
    fn combine_m(a: Self, b: Self) -> Self {
        let new_fut = a
            .inner
            .then(move |a_res| b.inner.map(move |b_res| A::combine_m(a_res, b_res)));
        CFuture::new_fut(new_fut)
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

impl<'a, T: Send + Sync + Clone + 'a, U: Send + Sync + Clone + 'a> Applicative<'a, T, U>
    for CFuture<'a, T>
{
    fn pure(a: T) -> Self {
        CFuture::new_fut(lazy(move |_| a))
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

impl<'a, T: Send + Sync + Clone + 'a, U: Send + Sync + Clone + 'a> Monad<'a, T, U>
    for CFuture<'a, T>
{
    type M = CFuture<'a, U>;
    fn bind(m: Self, func: impl FnOnce(T) -> Self::M + Send + 'a) -> Self::M {
        CFuture::new_fut(m.then(func))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_empty_future() {
        assert_eq!(CFuture::<u32>::empty().await, 0);
        assert_eq!(CFuture::<u32>::empty_m().await, 1);
    }

    #[tokio::test]
    async fn test_identity_future() {
        assert_eq!(
            combine(CFuture::lazy(3u32), CFuture::<u32>::empty()).await,
            3u32
        );
        assert_eq!(
            combine(CFuture::<u32>::empty(), CFuture::lazy(3u32)).await,
            3u32
        );
        assert_eq!(
            combine(CFuture::<u32>::empty(), CFuture::<u32>::empty()).await,
            0u32
        );
        assert_eq!(
            combine_m(CFuture::lazy(3u32), CFuture::<u32>::empty_m()).await,
            3u32
        );
        assert_eq!(
            combine_m(CFuture::<u32>::empty_m(), CFuture::lazy(3u32)).await,
            3u32
        );
        assert_eq!(
            combine_m(CFuture::<u32>::empty_m(), CFuture::<u32>::empty_m()).await,
            1u32
        );
    }

    #[tokio::test]
    async fn test_combine_future() {
        assert_eq!(combine(CFuture::lazy(3), CFuture::lazy(4)).await, 7);
    }
    #[tokio::test]
    async fn test_fmap_future() {
        assert_eq!(fmap(pure::<CFuture<_>, _>(3), |i| i + 4).await, 7);
    }
    #[tokio::test]
    async fn test_pure_future() {
        assert_eq!(pure::<CFuture<_>, _>(2).await, 2);
    }
    #[tokio::test]
    async fn test_seq_future() {
        let func = CFuture::lazy(|x| x + 4);
        assert_eq!(seq(CFuture::lazy(3), func).await, 7);
    }

    fn empty_if_even<'a, M: Monad<'a, u32> + Monoid + Applicative<'a, u32>>(input: String) -> M {
        if input.len() % 2 == 0 {
            M::empty()
        } else {
            M::pure(input.len() as u32)
        }
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
    #[tokio::test]
    async fn test_lift1_future() {
        let new_func = lift_m1::<CFuture<_>, _, _>(add4);
        assert_eq!(new_func(CFuture::lazy(3)).await, 7);
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }
    #[tokio::test]
    async fn test_lift2_future() {
        let new_func = lift_m2::<CFuture<_>, _, _, _, _>(add2);
        assert_eq!(new_func(CFuture::lazy(3), CFuture::lazy(4)).await, 7);
    }
}
