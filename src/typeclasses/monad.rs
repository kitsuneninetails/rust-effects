#![allow(refining_impl_trait)]
use crate::typeclasses::applicative::Applicative;

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
pub fn bind<'a, M: Monad<'a, T, U>, T: Send + 'a, U: Send + 'a>(
    m: M,
    func: impl Fn(T) -> M::M + Send + 'a,
) -> M::M {
    M::bind(m, func)
}

pub fn lift_m1<'a, In, S, T>(func: impl Fn(S) -> T + Send + Clone + 'a) -> impl Fn(In) -> In::M
where
    In: Monad<'a, S, T>,
    S: Send + 'a,
{
    In::M::lift_m1(func)
}

macro_rules! lift_m1 {
    ($m:tt) => {
        lift_m1::<$m<_>, _, _>
    };
}

pub fn lift_m2<'a, In1, In2, S2, S1, T>(
    func: impl Fn(S1, S2) -> T + Send + Clone + 'a,
) -> impl Fn(In1, In2) -> In1::M
where
    In1: Monad<'a, S1, T> + Send + Clone + 'a,
    In2: Monad<'a, S2, T, M = In1::M> + Send + Clone + 'a,
    S2: Send + Clone + 'a,
    S1: Send + Clone + 'a,
{
    In1::M::lift_m2(func)
}

macro_rules! lift_m2 {
    ($m:tt) => {
        lift_m2::<$m<_>, _, _, _, _>
    };
}

#[cfg(test)]
mod test {
    use super::*;

    fn add4(x: u32) -> u32 {
        x + 4
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test]
    fn test_lift1_macro() {
        let new_func = lift_m1![Option](add4);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }
    #[test]
    fn test_lift2_macro() {
        let new_func = lift_m2![Option](add2);
        assert_eq!(new_func(Some(3), Some(4)), Some(7));
        assert!(new_func(Some(3), None).is_none());
        assert!(new_func(None, Some(4)).is_none());
        assert!(new_func(None, None).is_none());
    }
}
