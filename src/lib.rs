pub mod typeclasses;
pub mod types;

pub mod prelude {
    pub use {macros::*, typeclasses::*, types::*};
    pub mod typeclasses {
        pub use crate::typeclasses::{
            applicative::{Applicative, pure},
            applicative_functor::{ApplicativeFunctor, seq},
            functor::{Functor, fmap},
            monad::{Monad, bind, lift_m1, lift_m2},
            monoid::{Monoid, empty, empty_m},
            semigroup::{Semigroup, combine, combine_m},
        };
    }
    pub mod types {
        pub use crate::types::cfuture::CFuture;
    }
    pub mod macros {
        pub use crate::{lift_m1, lift_m2, pure};
    }
}
