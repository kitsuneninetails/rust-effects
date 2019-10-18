use super::{Effect};

pub trait Applicative<X>: Effect {
    type FX;
    fn pure(x: X) -> Self::FX;
}

pub trait ApplicativeEffect: Sized {
    type X;
    type Fct: Applicative<Self::X, FX=Self>;
}

pub fn pure<I: ApplicativeEffect>(x: I::X) -> I {
    I::Fct::pure(x)
}
