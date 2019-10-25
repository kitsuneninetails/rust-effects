use crate::typeclasses::monad::Monad;
use crate::typeclasses::F;

pub trait SyncT<'a> : Monad<'a> {
    fn suspend(thunk: impl Fn() -> Self::FX + 'a + Send + Sync) -> Self::FX;
    fn delay(thunk: impl Fn() -> Self::X + 'a + Send + Sync) -> Self::FX {
        Self::suspend(move || Self::pure(thunk()))
    }
}

pub fn suspend<'a, T, FX, X>(_: T, thunk: impl Fn() -> FX + 'a + Send + Sync) -> FX
    where
        FX: F<X>,
        X: 'a + Send + Sync,
        T: SyncT<'a, X=X, FX=FX, Y=X, FY=FX> {
    T::suspend(thunk)
}


pub fn delay<'a, T, FX, X>(_: T, thunk: impl Fn() -> X + 'a + Send + Sync) -> FX
    where
        FX: F<X>,
        X: 'a + Send + Sync,
        T: SyncT<'a, X=X, FX=FX, Y=X, FY=FX> {
    T::delay(thunk)
}
