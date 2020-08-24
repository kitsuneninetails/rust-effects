use crate::typeclasses::monaderror::MonadError;

pub trait SyncT<'a> : MonadError<'a> {
    fn suspend(thunk: impl 'a + Fn() -> Self::FX + Send + Sync) -> Self::FX;
    fn delay(thunk: impl 'a + Fn() -> Self::X + Send + Sync) -> Self::FX {
        Self::suspend(move || Self::pure(thunk()))
    }
}
