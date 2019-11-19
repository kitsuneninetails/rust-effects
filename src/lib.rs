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
}
