pub mod applicative;
pub mod functor;
pub mod monad;
pub mod monaderror;
pub mod monoid;
pub mod product;
pub mod synct;
pub mod traverse;

pub trait F<X> {}
pub trait Effect {}
