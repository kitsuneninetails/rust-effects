use super::{F, Effect};

pub trait Applicative<FX, X>: Effect
    where FX: F<X> {
    fn pure(&self, x: X) -> FX;
}

pub fn pure<FX: F<X>, X>(ev: &impl Applicative<FX, X>, x: X) -> FX {
    ev.pure(x)
}
