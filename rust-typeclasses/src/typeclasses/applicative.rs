use super::{Effect};

pub trait Applicative<FX, X>: Effect {
    fn pure(&self, x: X) -> FX;
}

pub trait ApplicativeEffect: Sized {
    type X;
    type Fct: Applicative<Self, Self::X>;
    fn app() -> Self::Fct;
}

pub fn pure<I: ApplicativeEffect>(x: I::X) -> I {
    I::app().pure(x)
}
