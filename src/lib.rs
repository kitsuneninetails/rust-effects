pub mod applicative;
pub mod applicative_functor;
//pub mod free_monad;
pub mod functor;
pub mod monad;
pub mod monoid;
pub mod semigroup;

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
