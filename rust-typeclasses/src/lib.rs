#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]

pub mod future;
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
        monoid::*,
        product::*,
        semigroup::*,
        traverse::*,
        F, Effect
    };
    pub use crate::future::*;
    pub use crate::option::*;
    pub use crate::result::*;
    pub use crate::vec::*;
}
