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

pub extern crate futures;

pub mod prelude {
    pub use crate::typeclasses::{
        applicative::*,
        functor::*,
        monad::*,
        monaderror::*,
        monoid::*,
        product::*,
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
}

use prelude::*;

pub struct Empty;
impl<T> F<T> for Empty {}

pub struct NilEffect;
impl Effect for NilEffect {}
//
// impl<T> Monoid<T> for NilEffect {
//     fn empty() -> Empty { Empty }
//     fn combine(_: Empty, _: Empty) -> Empty { Empty }
// }

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
    type Z = ((), ());
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
    type Y2 = ();
    fn fold(_: Self::FX, init: Self::Y, _: impl 'a + Fn(Self::Y, Self::X) -> () + Send + Sync) -> Self::Y2 {
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

impl<'a> Productable<'a> for NilEffect {}

impl<'a> SyncT<'a> for NilEffect {
    fn suspend(_: impl 'a + Fn() -> Self::FX + Send + Sync) -> Self::FX {
        Empty
    }

}
