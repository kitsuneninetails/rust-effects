use super::{Effect};

pub trait Applicative<FX, X>: Effect {
    fn pure(x: X) -> FX;
}

pub trait ApplicativeEffect: Sized {
    type X;
    type Fct: Applicative<Self, Self::X>;
}

pub fn pure<I: ApplicativeEffect>(x: I::X) -> I {
    I::Fct::pure(x)
}
