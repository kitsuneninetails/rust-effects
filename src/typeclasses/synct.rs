use crate::typeclasses::monad::Monad;
use crate::typeclasses::{F, Effect};

pub trait SyncT<'a> : Monad<'a> {
    fn suspend(thunk: impl 'a + FnOnce() -> Self::FX + Send + Sync) -> Self::FX;
    fn delay(thunk: impl 'a + FnOnce() -> Self::X + Send + Sync) -> Self::FX {
        Self::suspend(move || Self::pure(thunk()))
    }
}

pub trait SyncTEffect<'a>: Sized where Self: F<<Self as SyncTEffect<'a>>::X> {
    type X;
    type E;
    type Fct: SyncT<'a, X=Self::X, FX=Self, Y=Self::X, FY=Self> + Effect;
}

pub fn suspend<'a, FX, X>(thunk: impl FnOnce() -> FX + 'a + Send + Sync) -> FX
    where
        FX: F<X> + SyncTEffect<'a>,
        X: 'a + Send + Sync {
    FX::Fct::suspend(thunk)
}

pub fn delay<'a, FX, X>(thunk: impl FnOnce() -> FX::X + 'a + Send + Sync) -> FX
    where
        FX: F<X> + SyncTEffect<'a>,
        X: 'a + Send + Sync {
    FX::Fct::delay(thunk)
}
