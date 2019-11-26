#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
#![feature(default_type_parameter_fallback)]

pub mod effects;
pub mod future;
pub mod futureresult;
pub mod option;
pub mod result;
pub mod typeclasses;
pub mod vec;
mod macros;

pub extern crate futures;

pub mod prelude {
    pub use crate::typeclasses::{
        applicative::*,
        functor::*,
        monad::*,
        monaderror::*,
        monoid::*,
        product::*,
        semigroup::*,
        traverse::*,
        synct::*,
        F, Effect
    };
    pub use crate::effects::{
        io::*
    };
    pub use crate::future::*;
    pub use crate::futureresult::*;
    pub use crate::option::*;
    pub use crate::result::*;
    pub use crate::vec::*;
    pub use crate::Effectful;
}

use prelude::*;

pub trait Effectful<'a, Inner, Err=(), Mapped=Inner, MapEffect: F<Mapped> = Self>:
    SyncTEffect<'a, X=Inner> +
    ApplicativeEffect<'a, Mapped, X=Inner> +
    FunctorEffect<'a, Inner, Mapped, FX=Self, FY=MapEffect> +
    MonadEffect<'a, Inner, Mapped, FX=Self, FY=MapEffect> +
    MonadErrorEffect<'a, Inner, E=Err> {}

impl<'a, Tp, X, Y, E, FY> Effectful<'a, X, E, Y, FY> for Tp
    where FY: F<Y>,
          Tp: SyncTEffect<'a, X=X> +
              ApplicativeEffect<'a, Y, X=X> +
              FunctorEffect<'a, X, Y, FX=Tp, FY=FY> +
              MonadEffect<'a, X, Y, FX=Tp, FY=FY> +
              MonadErrorEffect<'a, X, E=E> {}

pub struct Empty;
impl<T> F<T> for Empty {}

semigroup_effect! { 0, Empty, NilEffect }
monoid_effect! { 0, Empty, NilEffect }
applicative_effect! { 0, Empty, NilEffect }
functor_effect! { 0, Empty, NilEffect }
functor2_effect! { 0, Empty, NilEffect }
monad_effect! { 0, Empty, NilEffect }
foldable_effect! { 0, Empty, NilEffect }
monaderror_effect! { 0, Empty, NilEffect }
productable_effect! { 0, Empty, NilEffect }
synct_effect! { 0, Empty, NilEffect }

pub struct NilEffect;
impl Effect for NilEffect {}

impl Semigroup<Empty, Empty, Empty> for NilEffect {
    fn combine(_: Empty, _: Empty) -> Empty { Empty }
}

impl<'a> SemigroupInner<'a, Empty, ()> for NilEffect {
    fn combine_inner<TO>(_: Empty, _: Empty) -> Empty { Empty }
}

impl Monoid<Empty> for NilEffect {
    fn empty() -> Empty { Empty }
}
impl<'a> Functor<'a> for NilEffect {
    type X = ();
    type Y = ();
    type FX = Empty;
    type FY = Empty;
    fn fmap(_: Self::FX, _: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
        Empty
    }
}

impl<'a> Applicative<'a> for NilEffect {
    fn pure(_: ()) -> Self::FX {
        Empty
    }
}

impl<'a> Functor2<'a> for NilEffect {
    type Z = ();
    type FZ = Empty;
    fn fmap2(_: Self::FX,
             _: Self::FY,
             _: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
        Empty
    }
}

impl<'a> Monad<'a> for NilEffect {
    fn flat_map(_: Self::FX, _: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
        Empty
    }
}

impl<'a> Foldable<'a> for NilEffect {
    type Z = ();
    fn fold(_: Self::FX, init: Self::Y, _: impl 'a + Fn(Self::Y, Self::X) -> () + Send + Sync) -> Self::Z {
        init
    }
}

impl<'a> MonadError<'a> for NilEffect {
    type E = ();
    fn raise_error(_err: Self::E) -> Self::FX {
        Empty
    }

    fn handle_error(_: Self::FX, _: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
        Empty
    }

    fn attempt(_: Self::FX) -> Result<Self::X, Self::E> {
        Ok(())
    }
}

impl<'a> Productable<'a> for NilEffect {
    type FXY = Empty;
    fn product(_: Self::FX, _: Self::FY) -> Self::FXY {
        Empty
    }
}

impl<'a> SyncT<'a> for NilEffect {
    fn suspend(_: impl 'a + Fn() -> Self::FX + Send + Sync) -> Self::FX {
        Empty
    }

}
