use super::F;
use crate::typeclasses::monad::Monad;
use crate::typeclasses::functor::{Functor, Functor2};

/// `Productable` is an extended typeclass that just specifies the ability to combine two
/// type constructors and map the internal type to a new, combined type, represented by a 2-tuple
/// of the original values.  This is usually equivalent to a `fmap2` from `Functor2`, using a
/// tuple output as the combining function.
pub trait Productable<'a>: Monad<'a, Z=(<Self as Functor<'a>>::X, <Self as Functor<'a>>::Y)>
    where Self::FZ: F<<Self as Functor2<'a>>::Z> {
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FZ {
        Self::fmap2(fa, fb, |a, b| (a, b))
    }
}
