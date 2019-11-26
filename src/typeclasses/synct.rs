use crate::typeclasses::monaderror::MonadError;
use crate::typeclasses::{F, Effect};

pub trait SyncT<'a> : MonadError<'a> {
    fn suspend(thunk: impl 'a + Fn() -> Self::FX + Send + Sync) -> Self::FX;
    fn delay(thunk: impl 'a + Fn() -> Self::X + Send + Sync) -> Self::FX {
        Self::suspend(move || Self::pure(thunk()))
    }
}

pub trait SyncTEffect<'a>: Sized where Self: F<<Self as SyncTEffect<'a>>::X> {
    type X;
    type E;
    type Fct: SyncT<'a, X=Self::X, FX=Self, Y=Self::X, FY=Self, E=Self::E> + Effect;
}

pub fn suspend<'a, FX, X>(thunk: impl Fn() -> FX + 'a + Send + Sync) -> FX
    where
        FX: F<X> + SyncTEffect<'a, X=X>,
        X: 'a + Send + Sync {
    FX::Fct::suspend(thunk)
}

pub fn delay<'a, FX>(thunk: impl Fn() -> FX::X + 'a + Send + Sync) -> FX
    where
        FX: SyncTEffect<'a> {
    FX::Fct::delay(thunk)
}
