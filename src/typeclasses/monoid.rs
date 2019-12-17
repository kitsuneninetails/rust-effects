use super::Effect;
use std::marker::PhantomData;

/// The Monoid typeclass.  This provides the concept of a "zero" or "empty" value.  This should
/// function as the identity for the typeclass.  Combined with Semigroup, this typeclass qualifies
/// as a mathematical Monoid, with an associative operation (provided by Semigroup's `combine`
/// method) which can combine values and an identity value for which the following holds true
/// given a value A of type T and an identity value I of type T:
///
/// A: T · I: T == A: T
///
/// for all legal values in type T.
pub trait Monoid<M> : Effect {
    fn empty() -> M;
}

pub trait MonoidEffect : Sized {
    type Fct: Monoid<Self>;
}

pub fn empty<FX>() -> FX
    where
        FX: MonoidEffect {
    FX::Fct::empty()
}

pub struct StringMonoid<X> {
    _1: PhantomData<X>
}

impl<X> Effect for StringMonoid<X> {}

impl Monoid<&'static str> for StringMonoid<&'static str> {
    fn empty() -> &'static str { "" }
}
impl MonoidEffect for &'static str {
    type Fct = StringMonoid<&'static str>;
}

impl Monoid<String> for StringMonoid<String> {
    fn empty() -> String { format!("") }
}

impl MonoidEffect for String {
    type Fct = StringMonoid<String>;
}

#[macro_export]
macro_rules! monoid_int_impl {
    ($m:tt, $v:expr, $($t:ty)+) => ($(
        impl Monoid<$t> for $m<$t> {
            fn empty() -> $t { $v }
        }
    )+)
}

#[macro_export]
macro_rules! monoid_eff_int_impl {
    ($m:tt, $($t:ty)+) => ($(
        impl MonoidEffect for $t {
            type Fct = $m<$t>;
        }
    )+)
}

pub struct IntAddMonoid<X> {
    _1: PhantomData<X>
}
impl<X> Effect for IntAddMonoid<X> {}

pub struct IntMulMonoid<X> {
    _1: PhantomData<X>
}
impl<X> Effect for IntMulMonoid<X> {}

monoid_int_impl! { IntAddMonoid, 0, u8 u16 u32 u64 i8 i16 i32 i64 }
monoid_int_impl! { IntAddMonoid, 0.0, f32 f64 }

monoid_int_impl! { IntMulMonoid, 1, u8 u16 u32 u64 i8 i16 i32 i64 }
monoid_int_impl! { IntMulMonoid, 1.0, f32 f64 }

monoid_eff_int_impl! { IntAddMonoid, u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 }

#[cfg(test)]
mod tests {
    use crate::typeclasses::monoid::*;

    #[test]
    fn test_strings() {
        let out: &str = StringMonoid::empty();
        assert_eq!(out, "");
    }

    #[test]
    fn test_ints() {
        let out: i32 = IntAddMonoid::empty();
        assert_eq!(out, 0);

        let out: u32 = IntMulMonoid::empty();
        assert_eq!(out, 1);
    }

    #[test]
    fn test_functionals() {
        let out: i32 = empty();
        assert_eq!(out, 0);

        let out: String = empty();
        assert_eq!(out, "");
    }
}
