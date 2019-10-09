use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Semigroup Typeclass
/// Encapsulates any type which is "combine-able", in other words, can be combined to form
/// a new value of the functionally equivalent type:
///
/// A: T · B: T => C: T
///
/// It also must follow the rule of associativity:
///
/// (A · B) · C === A · (B · C)
///
/// Rust Note:
/// Having a separate T2/TR allows for instances when the input types and output types are
/// functionally equivalent, but due to Rust's strict typing (where types have to be statically
/// known) are technically different.  The best example is with Futures, where the two input
/// futures might be a Ready and a Lazy and the output will be an AndThen, and even though all
/// implement the `TryFuture` trait, they are actually different static types.
///
/// The `ToString` trait is another that can make use of this (where one parameter is a string slice
/// while the other is an owned string).
pub trait Semigroup<T, T2, TR> {
    fn combine(self, a: T, b: T2) -> TR;
}

pub fn combine<T, T2, TR>(t: impl Semigroup<T, T2, TR>, a: T, b: T2) -> TR {
    t.combine(a, b)
}

pub struct StringSemigroup;
pub const STR_SG: StringSemigroup = StringSemigroup;

impl<T: ToString, T2: ToString> Semigroup<T, T2, String> for StringSemigroup {
    fn combine(self, a: T, b: T2) -> String { format!("{}{}", a.to_string(), b.to_string()) }
}

pub struct IntAddSemigroup;
pub const IADD_SG: IntAddSemigroup = IntAddSemigroup;

impl<T: Add<Output=T>> Semigroup<T, T, T> for IntAddSemigroup {
    fn combine(self, a: T, b: T) -> T { a + b }
}

pub struct IntMulSemigroup;
pub const IMUL_SG: IntMulSemigroup = IntMulSemigroup;

impl<T: Mul<Output=T>> Semigroup<T, T, T> for IntMulSemigroup {
    fn combine(self, a: T, b: T) -> T { a * b }
}

pub struct CombineInnerSemigroup<X, X2, XR, T: Semigroup<X, X2, XR>> {
    pub t: T,
    _p1: PhantomData<X>,
    _p2: PhantomData<X2>,
    _p3: PhantomData<XR>
}

impl<X, X2, XR, T: Semigroup<X, X2, XR>> CombineInnerSemigroup<X, X2, XR, T> {
    pub fn apply(t: T) -> Self {
        CombineInnerSemigroup {
            t,
            _p1: PhantomData,
            _p2: PhantomData,
            _p3: PhantomData
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::typeclasses::semigroup::*;

    #[test]
    fn string_combine() {
        let out = STR_SG.combine("Hello", format!(" World"));
        assert_eq!("Hello World", out);
    }

    #[test]
    fn int_combine() {
        let out = IADD_SG.combine(1, 2);
        assert_eq!(3, out);

        let out = Semigroup::combine(IMUL_SG, 1.2, 2.2);
        assert_eq!(2.64, out);
    }

    #[test]
    fn functional_combine() {
        let out = combine(IMUL_SG, 1, 2);
        assert_eq!(2, out);

        let out = combine(IADD_SG, 5, 4);
        assert_eq!(9, out);
    }
}
