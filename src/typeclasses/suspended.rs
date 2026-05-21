use std::marker::PhantomData;

use crate::{
    prelude::Functor,
    typeclasses::{applicative::Applicative, monad::Monad},
};

pub trait FreeEffect {
    fn fold<T, U, M>(self, source: M) -> M::M
    where
        M: Monad<T, U> + Send,
        T: Send,
        U: Send;
}

pub enum Compound<T: Send, U: Send, V: Send, M: Monad<U, V> + Send> {
    Foo(Box<Compound<U, V, (), M::M>>),
    Bar(T, U, M),
}

pub struct FreeMap<T: Send, U: Send> {
    func: Box<dyn Fn(T) -> U + Send>,
}

impl<T: Send, U: Send> FreeMap<T, U> {
    pub fn apply<M: Monad<T, U> + Send>(self, source: M) -> M::M {
        M::fmap(source, self.func)
    }
}

impl<T, U> FreeEffect for FreeMap<T, U>
where
    T: Send,
    U: Send,
{
    fn fold<X, Y, M>(self, source: M) -> M::M
    where
        M: Monad<T, U> + Send,
        X: Send,
        Y: Send,
    {
        M::fmap(source, self.func)
    }
}

pub struct FreeBind<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    func: Box<dyn Fn(T) -> M::M + Send>,
}

impl<T: Send, U: Send, M: Monad<T, U> + Send> FreeBind<T, U, M> {
    pub fn apply(self, source: M) -> M::M {
        M::bind(source, self.func)
    }
}

impl<T, U, M> FreeEffect for FreeBind<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    fn fold<T, U, M>(
        self,
        source: Self::M,
    ) -> <<Self as FreeEffect<'a>>::M as Monad<Self::T, Self::U>>::M {
        M::bind(source, self.func)
    }
}

pub enum CompoundFree {}

pub struct Free<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    source: M,
    transformations: Vec<Box<dyn FreeEffect<'a>>>,
}

impl<T, U, M> Free<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    pub fn fold_map(self) -> M {
        todo!()
    }
    pub fn map(self, func: impl Fn(T) -> U) -> Self {}
    pub fn pure(t: T) -> Self {
        Suspended::Pure(M::pure(t))
    }
    pub fn flat_map(source: Self, func: impl Fn(T) -> Suspended<U, (), M::M>) -> Self {
        Suspended::Bind(Box::new(source), Box::new(func))
    }
}

impl<T, U, M> Functor<T, U> for Suspended<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    type F = Suspended<U, (), M::M>;
    fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F {
        m.map(func)
    }
}

impl<T, U, M> Applicative<T, U> for Suspended<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    fn pure(t: T) -> Self {
        Self::pure(t)
    }
}

impl<T, U, M> Monad<T, U> for Suspended<T, U, M>
where
    M: Monad<T, U> + Send,
    T: Send,
    U: Send,
{
    type M = Suspended<T, U, M>;
    fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M {
        m.flat_map(func)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_vec_free_monad() {
        let to_s = lift_m1![Vec](str::to_string);
        let len = lift_m1![Vec](String::len);
        let only_evens = |a: usize| {
            if a % 2 == 0 {
                pure![Vec](a)
            } else {
                Vec::empty()
            }
        };

        let startv = vec!["fox", "horse", "ox", "crow", "mouse", "donkey"];
        let std_step1 = fmap(startv, str::to_string);
        let std_step2 = fmap(std_step1, |a| a.len());
        let standard_out = bind(std_step2, only_evens);

        let startv2 = vec!["fox", "horse", "ox", "crow", "mouse", "donkey"];
        let free_step1 = fmap(Suspended::suspend(startv2), str::to_string);
        let free_step2 = fmap(free_step1, |a| a.len());
        let free_out = bind(free_step2, only_evens);
        // Nothing has been done until the wrap-up call:
        let standard_out: Vec<usize> = free_out.fold_map();
    }
}
