use crate::typeclasses::{applicative::Applicative, functor::Functor};

pub trait ApplicativeFunctor<'a, F: Fn(T) -> U, T, U = ()>: Applicative<'a, T, U> {
    type AOut: Applicative<'a, U>;
    type AFunc: Functor<'a, F, U, F = Self::AOut>;

    fn seq(m: Self, func: Self::AFunc) -> Self::AOut;
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
}
