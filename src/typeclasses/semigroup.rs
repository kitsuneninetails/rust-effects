use super::Effect;
use crate::{sg_impl, sg_eff_impl};

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
pub trait Semigroup<T, T2, TR>: Effect {
    fn combine(a: T, b: T2) -> TR;
}

pub trait SemigroupEffect<T, T2, TR> {
    type Fct: Semigroup<T, T2, TR>;
}

pub trait SemigroupInner<'a, T, X> where T: 'a {
    fn combine_inner<TO>(a: T, b: T) -> T where TO: 'a + Semigroup<X, X, X>;
}

pub fn combine<T, T2, TR>(a: T, b: T2) -> TR
    where
        T: SemigroupEffect<T, T2, TR> {
    T::Fct::combine(a, b)
}

pub fn combine_inner<'a, T, X, TO>(a: T, b: T) -> T
    where
        T: 'a + SemigroupEffect<T, T, T, Fct: SemigroupInner<'a, T, X>>,
        TO: 'a + Semigroup<X, X, X> {
    T::Fct::combine_inner::<TO>(a, b)
}

// String types

pub struct StringSemigroup;
impl<T: ToString, T2: ToString> SemigroupEffect<T, T2, String> for String {
    type Fct=StringSemigroup;
}
impl<'a, T: ToString, T2: ToString> SemigroupEffect<T, T2, String> for &'a str {
    type Fct=StringSemigroup;
}

impl Effect for StringSemigroup {}

impl<T: ToString, T2: ToString> Semigroup<T, T2, String> for StringSemigroup {
    fn combine(a: T, b: T2) -> String { format!("{}{}", a.to_string(), b.to_string()) }
}

// Integer and Rational types (add)
pub struct IntAddSemigroup;
impl Effect for IntAddSemigroup {}

pub struct IntMulSemigroup;
impl Effect for IntMulSemigroup {}

sg_impl! { IntAddSemigroup, +, u8 u16 u32 u64 i8 i16 i32 i64 f32 f64}
sg_eff_impl! { IntAddSemigroup, u8 u16 u32 u64 i8 i16 i32 i64 f32 f64}

sg_impl! { IntMulSemigroup, *, u8 u16 u32 u64 i8 i16 i32 i64 f32 f64}

#[cfg(test)]
mod tests {
    use crate::typeclasses::semigroup::*;

    #[test]
    fn string_combine() {
        let out = combine("Hello", format!(" World"));
        assert_eq!("Hello World", out);
    }

    #[test]
    fn int_combine() {
        let out = combine(1, 2);
        assert_eq!(3, out);

        let out = IntMulSemigroup::combine(1.2, 2.2);
        assert_eq!(2.64, out);
    }

    #[test]
    fn functional_combine() {
        let out = IntMulSemigroup::combine(1, 2);
        assert_eq!(2, out);

        let out = combine(5, 4);
        assert_eq!(9, out);
    }
}
