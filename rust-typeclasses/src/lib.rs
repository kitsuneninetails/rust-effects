pub mod option;
pub mod result;
pub mod typeclasses;
pub mod vec;

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
    pub use crate::option::*;
    pub use crate::result::*;
    pub use crate::vec::*;
}
