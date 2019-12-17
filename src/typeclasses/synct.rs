//use crate::typeclasses::monaderror::MonadError;
//use crate::typeclasses::{F, Effect};
//
//pub trait SyncT<'a> : MonadError<'a> {
//    fn suspend(thunk: impl 'a + Fn() -> Self::FctForX + Send + Sync) -> Self::FctForX;
//    fn delay(thunk: impl 'a + Fn() -> Self::FnctX + Send + Sync) -> Self::FctForX {
//        Self::suspend(move || Self::pure(thunk()))
//    }
//}
//
//pub trait SyncTEffect<'a>: Sized where Self: F<<Self as SyncTEffect<'a>>::X> {
//    type X;
//    type E;
//    type Fct: SyncT<'a, FnctX=Self::X, FctForX=Self, FnctY=Self::X, FctForY=Self, E=Self::E> + Effect;
//}
//
//pub fn suspend<'a, FX, X>(thunk: impl Fn() -> FX + 'a + Send + Sync) -> FX
//    where
//        FX: SyncTEffect<'a, X=X>,
//        X: 'a + Send + Sync {
//    FX::Fct::suspend(thunk)
//}
//
//pub fn delay<'a, FX>(thunk: impl Fn() -> FX::X + 'a + Send + Sync) -> FX
//    where
//        FX: SyncTEffect<'a> {
//    FX::Fct::delay(thunk)
//}
