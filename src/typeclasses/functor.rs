use super::{F, Effect};

/// The `Functor` type class.  This represents a mapping from one type to another, which takes
/// place inside the given context.  For a Functor for context `C` containing members of type `X`,
/// a function `fmap` must defined `X -> Y` where `Y` is the target type.  Applying the Functor
/// for context `C` will result in the context `C'` which contains the same number of members as
/// the original `C`, but containing the members `{ fmap(x1), fmap(x2), ..., fmap(xn) } ` for all
/// members` x1, x2, ..., xn` in `C`.
pub trait Functor<'a>: Effect {
    type X;
    type Y;
    type FX: F<Self::X>;
    type FY: F<Self::Y>;
    fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY;
}

/// Functor2 is similar to a normal `Functor`, except it can take two contexts and combine them
/// with a function.
pub trait Functor2<'a>: Functor<'a> {
    type Z;
    type FZ: F<Self::Z>;
    fn fmap2(fa: Self::FX,
             fb: Self::FY,
             func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ;
}