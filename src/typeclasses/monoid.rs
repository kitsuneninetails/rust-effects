use super::Effect;
use std::ops::{Add, Mul};
use serde::export::PhantomData;
use std::fmt::Display;

/// The Monoid and Semigroup Typeclasses
///
/// `Semigropup` encapsulates any type which is "combine-able", in other words, can be combined
/// to form a new value of the functionally equivalent type:
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
///
/// The `Monoid` typeclass provides the concept of a "zero" or "empty" value.  This should function
/// as the identity for the typeclass.  Combined with `Semigroup` above, this typeclass qualifies
/// as a mathematical Monoid, with an associative operation (provided by Semigroup's `combine`
/// method) which can combine values and an identity value for which the following holds true
/// given a value A of type T and an identity value I of type T:
///
/// A: T · I: T == A: T
///
/// for all legal values in type T.
// pub trait Monoid : Effect {
//     fn empty() -> Out;
// }
//
// pub trait Semigroup<In1, In2, Out> : Effect {
//     fn combine<InnerMonoid>(a: In1, b: In2) -> Out where InnerMonoid: MonoidEffect;
// }

pub trait Numeric : Display + Add<Output=Self> + Mul<Output=Self> + Sized {
    fn from_u64(i: u64) -> Self;
    fn from_f64(i: f64) -> Self;
}

macro_rules! numeric_impl {
    ($($t:ty)+) => ($(
        impl Numeric for $t {
            fn from_u64(i: u64) -> Self { i as $t }
            fn from_f64(i: f64) -> Self { i as $t }
        }
    )+)
}

numeric_impl!(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);

pub trait Monoid {
    type Output;
    fn empty() -> Self::Output;
}

pub trait Semigroup<In1, In2, Out> : Monoid<Output=Out> {
    fn combine(a: In1, b: In2) -> Self::Output;
}

pub struct StringMonoid {}
pub struct IntAddMonoid<T> {
    _t: PhantomData<T>
}
pub struct IntMulMonoid<T> {
    _t: PhantomData<T>
}

impl Effect for StringMonoid {}
impl<T> Effect for IntAddMonoid<T> {}
impl<T> Effect for IntMulMonoid<T> {}

impl Monoid for StringMonoid {
    type Output = String;
    fn empty() -> Self::Output { "".to_string() }
}

impl<In1, In2> Semigroup<In1, In2, String> for StringMonoid
    where
        In1: ToString + Display,
        In2: ToString + Display {
    fn combine(a: In1, b: In2) -> Self::Output {
        format!("{}{}", a.to_string(), b.to_string())
    }
}

impl<T: Numeric> Monoid for IntAddMonoid<T> {
    type Output = T;
    fn empty() -> Self::Output { T::from_u64(0) }
}

impl<T: Numeric> Semigroup<T, T, T> for IntAddMonoid<T> {
    fn combine(a: T, b: T) -> Self::Output { a + b }
}

impl<T: Numeric> Monoid for IntMulMonoid<T> {
    type Output = T;
    fn empty() -> Self::Output { T::from_u64(1) }
}

impl<T: Numeric> Semigroup<T, T, T> for IntMulMonoid<T> {
    fn combine(a: T, b: T) -> Self::Output { a * b }
}

#[macro_export]
macro_rules! int_add_monoid {
    () => (IntAddMonoid { _t: PhantomData })
}

#[macro_export]
macro_rules! int_mul_monoid {
    () => (IntMulMonoid { _t: PhantomData })
}

#[macro_export]
macro_rules! string_monoid {
    () => (StringMonoid)
}

#[cfg(test)]
mod tests {
    use crate::typeclasses::monoid::{StringMonoid, IntAddMonoid};
    use super::*;

    #[test]
    fn string_combine() {
        let out = StringMonoid::combine("Hello", format!(" World"));
        assert_eq!("Hello World", out);
    }

    #[test]
    fn int_combine() {
        let out: u32 = IntAddMonoid::combine(1u32, 2u32);
        assert_eq!(3, out);
        let out: u32 = IntMulMonoid::combine(1u32, 2u32);
        assert_eq!(2, out);

        fn foo<X, S: Semigroup<X, X, X>>(_ev: S, a: X, b: X) -> X {
            S::combine(a, b)
        }

        assert_eq!(foo(int_add_monoid!(), 2i32, -2i32), 0);
        assert_eq!(foo(int_mul_monoid!(), 2i32, -2i32), -4i32);
    }

    #[test]
    fn functional_combine() {
        let out = IntMulMonoid::combine(1, 2);
        assert_eq!(2, out);

        let out = IntAddMonoid::combine(5, 4);
        assert_eq!(9, out);
    }

    #[test]
    fn test_strings() {
        let out: String = StringMonoid::empty();
        assert_eq!(out, "");
    }

    #[test]
    fn test_ints() {
        let out: i32 = IntAddMonoid::empty();
        assert_eq!(out, 0);

        let out: u32 = IntMulMonoid::empty();
        assert_eq!(out, 1);
    }
}
