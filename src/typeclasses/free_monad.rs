use std::marker::PhantomData;

use futures::future::lazy;

use crate::effects::{CFuture, applicative::Applicative, monad::Monad};

pub trait FreeT<'a, T, M>
where
    M: 'a + Monad<'a, T, ()> + Send,
    T: Send,
{
    async fn fold_map(self) -> M;
    async fn flat_map(self) -> M;
    async fn map(self) -> M;
}

enum Free<'a, T, U, M, N>
where
    M: 'a + Monad<'a, T, ()> + Send,
    N: 'a + Monad<'a, U, ()> + Send,
    T: Send,
    U: Send,
{
    Pure(T),
    Suspend(M),
    FlatMap(Free<'a, T, M>, Box<dyn Fn(T) -> Free<'a, U, N>>),
}

impl FreeT for Free {
    async fn fold_map(s: impl FreeT) -> M {
        todo!()
    }
}

impl<'a, T, M> Applicative<M> for Free<'a, T, M>
where
    M: 'a + Monad<'a, T, ()> + Send,
    T: Send,
{
    fn pure(a: M) -> Self {
        Free::new(a)
    }
}
// impl<'a, T, U, M> Monad<'a, T, U> for Free<'a, T, M>
// where
//     M: 'a + Monad<'a, T, U> + Send,
//     T: Send,
// {
//     type M = Free<'a, U, ()>;
//     fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M {
//         CFuture::bind(m.inner, func)
//     }
// }
