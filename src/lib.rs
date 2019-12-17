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
pub mod macros;

pub extern crate futures;

pub mod prelude {
    pub use crate::typeclasses::{
        applicative::*,
        foldable::*,
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
//    pub use crate::Effectful;
}

use prelude::*;

//pub trait Effectful<'a, Inner, Err=(), Mapped=Inner, MapEffect: F<Mapped> = Self>:
//    SyncTEffect<'a, X=Inner> +
//    ApplicativeEffect<'a, Mapped, X=Inner> +
//    FunctorEffect<'a, Mapped, FctForY=MapEffect> +
//    MonadEffect<'a, Mapped, FY=MapEffect> +
//    MonadErrorEffect<'a, E=Err> {}
//
//impl<'a, Tp, X, Y, E, FY> Effectful<'a, X, E, Y, FY> for Tp
//    where FY: F<Y>,
//          Tp: SyncTEffect<'a, X=X> +
//              ApplicativeEffect<'a, Y, X=X> +
//              FunctorEffect<'a, Y, FctForY=FY> +
//              MonadEffect<'a, Y, FY=FY> +
//              MonadErrorEffect<'a, E=E> {}

pub struct Empty;
impl<T> F<T> for Empty {}

semigroup_effect! { 0, Empty, NilEffect }
monoid_effect! { 0, Empty, NilEffect }
functor_effect! { 0, Empty, NilEffect }
//applicative_effect! { 0, Empty, NilEffect }
//monad_effect! { 0, Empty, NilEffect }
//foldable_effect! { 0, Empty, NilEffect }
//monaderror_effect! { 0, Empty, NilEffect }
//productable_effect! { 0, Empty, NilEffect }
//synct_effect! { 0, Empty, NilEffect }

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
    type FnctX = ();
    type FnctY = ();
    type FnctZ = ();
    type FctForX = Empty;
    type FctForY = Empty;
    type FctForZ = Empty;
    fn fmap(_: Self::FctForX, _: impl 'a + Fn(Self::FnctX) -> Self::FnctY + Send + Sync) -> Self::FctForY {
        Empty
    }
    fn fmap2(_: Self::FctForX,
             _: Self::FctForY,
             _: impl 'a + Fn(Self::FnctX, Self::FnctY) -> Self::FnctZ + Send + Sync) -> Self::FctForZ {
        Empty
    }
}

//impl<'a> Applicative<'a> for NilEffect {
//    fn pure(_: ()) -> Self::FX {
//        Empty
//    }
//}
//
//impl<'a> Monad<'a> for NilEffect {
//    fn flat_map(_: Self::FX, _: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//        Empty
//    }
//}
//
//impl<'a> Foldable<'a> for NilEffect {
//    type Fld = Empty;
//    type FldInner = ();
//    type Folded = ();
//    type Folded2 = ();
//    fn fold(f: Self::Fld,
//            init: Self::Folded,
//            func: impl 'a + Fn(Self::Folded, Self::FldInner) -> Self::Folded2 + Send + Sync) -> Self::Folded2 {
//        init
//    }
//}
//
//impl<'a> MonadError<'a> for NilEffect {
//    type E = ();
//    fn raise_error(_err: Self::E) -> Self::FX {
//        Empty
//    }
//
//    fn handle_error(_: Self::FX, _: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
//        Empty
//    }
//
//    fn attempt(_: Self::FX) -> Result<Self::X, Self::E> {
//        Ok(())
//    }
//}
//
//impl<'a> Productable<'a> for NilEffect {}
//
//impl<'a> SyncT<'a> for NilEffect {
//    fn suspend(_: impl 'a + Fn() -> Self::FX + Send + Sync) -> Self::FX {
//        Empty
//    }
//
//}
