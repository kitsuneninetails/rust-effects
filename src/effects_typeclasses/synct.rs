use crate::typeclasses::monad::Monad;

trait SyncT<'a> : Monad<'a> {
    fn suspend(fx_fn: impl Fn() -> Self::FX) -> Self::FX;
    fn delay(x_fn: impl Fn() -> Self::X) -> Self::FX {
        Self::suspend(|| Self::pure(x_fn()))
    }
}

