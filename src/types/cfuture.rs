use crate::prelude::typeclasses::*;
use futures::future::{BoxFuture, Shared, lazy};
use futures_util::FutureExt;

#[derive(Clone)]
pub struct CFuture<A> {
    inner: Shared<BoxFuture<'static, A>>,
}

impl<A: Clone + Sync + Send + 'static> CFuture<A> {
    pub fn lazy(val: A) -> CFuture<A> {
        CFuture::new(lazy(|_| val))
    }

    pub fn new(fut: impl Future<Output = A> + Send + 'static) -> CFuture<A> {
        CFuture {
            inner: fut.boxed().shared(),
        }
    }
}

impl<A> Future for CFuture<A>
where
    A: Clone + Send + Sync,
{
    type Output = A;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

unsafe impl<A> Send for CFuture<A> where A: Clone + Send + Sync {}
unsafe impl<A> Sync for CFuture<A> where A: Clone + Send + Sync {}

impl<A> Monoid for CFuture<A>
where
    A: Monoid + Clone + Send + Sync + 'static,
{
    fn empty() -> Self {
        CFuture::lazy(A::empty())
    }
    fn empty_m() -> Self {
        CFuture::lazy(A::empty_m())
    }
}

impl<A> Semigroup for CFuture<A>
where
    A: Semigroup + Clone + Send + Sync + 'static,
{
    fn combine(a: Self, b: Self) -> Self {
        let f = a
            .inner
            .then(move |a_res| b.inner.map(move |b_res| A::combine(a_res, b_res)));
        CFuture::new(f)
    }
    fn combine_m(a: Self, b: Self) -> Self {
        let f = a
            .inner
            .then(move |a_res| b.inner.map(move |b_res| A::combine_m(a_res, b_res)));
        CFuture::new(f)
    }
}

impl<T, U> Functor<U> for CFuture<T>
where
    T: Send + Sync + Clone + 'static,
    U: Send + Sync + Clone + 'static,
{
    type FuncT = T;
    type FunctorOut = CFuture<U>;
    fn fmap(m: Self, func: impl FnOnce(T) -> U + Send + 'static) -> Self::FunctorOut {
        CFuture::new(m.map(func))
    }
}

impl<T, U> Applicative<U> for CFuture<T>
where
    T: Send + Sync + Clone + 'static,
    U: Send + Sync + Clone + 'static,
{
    type AppT = T;
    fn pure(a: T) -> Self {
        CFuture::lazy(a)
    }
}

impl<F, T, U> ApplicativeFunctor<F, U> for CFuture<T>
where
    F: Fn(T) -> U + Sync + Send + Clone + 'static,
    T: Send + Clone + Sync + 'static,
    U: Send + Clone + Sync + 'static,
{
    type AppFuncT = T;
    type AppFuncOut = CFuture<U>;
    type AppFuncFn = CFuture<F>;
    fn seq(m: Self, func: Self::AppFuncFn) -> Self::AppFuncOut {
        let in_f = func;
        CFuture::new(async { in_f.await(m.await) })
    }
}

impl<T, U> Monad<U> for CFuture<T>
where
    T: Send + Sync + Clone + 'static,
    U: Send + Sync + Clone + 'static,
{
    type MonadT = T;
    type MonadOut = CFuture<U>;
    fn bind(m: Self, func: impl FnOnce(T) -> Self::MonadOut + Send + 'static) -> Self::MonadOut {
        CFuture::new(m.then(func))
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
        assert_eq!(fmap(pure::<CFuture<_>>(3), |i| i + 4).await, 7);
    }
    #[tokio::test]
    async fn test_pure_future() {
        assert_eq!(pure::<CFuture<_>>(2).await, 2);
    }
    #[tokio::test]
    async fn test_seq_future() {
        let func = CFuture::lazy(|x| x + 4);
        assert_eq!(seq(CFuture::lazy(3), func).await, 7);
    }

    fn empty_if_even<'a, M: Monad<u32, MonadT = u32> + Monoid + Applicative<u32>>(
        input: String,
    ) -> M {
        if input.len() % 2 == 0 {
            M::empty()
        } else {
            M::pure(input.len() as u32)
        }
    }

    #[tokio::test]
    async fn test_bind_future() {
        assert_eq!(
            bind(pure::<CFuture<_>>("dog".to_string()), empty_if_even).await,
            3
        );
        assert_eq!(
            bind(pure::<CFuture<_>>("crow".to_string()), empty_if_even).await,
            0
        );
    }

    fn add4(x: u32) -> u32 {
        x + 4
    }
    #[tokio::test]
    async fn test_lift1_future() {
        let new_func = lift_m1::<CFuture<_>, _>(add4);
        assert_eq!(new_func(CFuture::lazy(3)).await, 7);
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }
    #[tokio::test]
    async fn test_lift2_future() {
        let new_func = lift_m2::<CFuture<_>, _, _>(add2);
        assert_eq!(new_func(CFuture::lazy(3), CFuture::lazy(4)).await, 7);
    }
}
