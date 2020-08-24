use crate::typeclasses::monad::Foldable;

/// The `MonadError` typeclass.  This just ensures a `flat_map` operation is available for a context
/// of type `F<_>` which operates on a type `X` which can perform a new function returning
/// another context for the given type `X`.  This context is then "flattened" into the originating
/// context, essentially taking its place as the context holder for `X`.
pub trait MonadError<'a>: Foldable<'a> {
    type E;
    fn raise_error(err: Self::E) -> Self::FX;
    fn handle_error(f: Self::FX, recovery: impl 'a + Send + Sync + Fn(Self::E) -> Self::FX) -> Self::FX;
    fn attempt(f: Self::FX) -> Result<Self::X, Self::E>;
}

