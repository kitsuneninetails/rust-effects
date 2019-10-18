use super::Effect;

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

pub trait MonoidEffect<M> {
    type Fct: Monoid<M>;
}

pub fn empty<X: MonoidEffect<X>>() -> X {
    X::Fct::empty()
}

macro_rules! monoid_impl {
    ($m:ty, $v:expr, $($t:ty)+) => ($(
        impl Monoid<$t> for $m {
            fn empty() -> $t { $v }
        }
    )+)
}

macro_rules! monoid_eff_impl {
    ($m:ty, $me:expr, $($t:ty)+) => ($(
        impl MonoidEffect<$t> for $t {
            type Fct = $m;
        }
    )+)
}

pub struct StringMonoid;
impl Effect for StringMonoid {}

pub struct IntAddMonoid;
impl Effect for IntAddMonoid {}

pub struct IntMulMonoid;
impl Effect for IntMulMonoid {}

monoid_impl! { StringMonoid, "".to_string(), String }
monoid_impl! { IntAddMonoid, 0, u8 u16 u32 u64 i8 i16 i32 i64 }
monoid_impl! { IntAddMonoid, 0.0, f32 f64 }

monoid_impl! { IntMulMonoid, 1, u8 u16 u32 u64 i8 i16 i32 i64 }
monoid_impl! { IntMulMonoid, 1.0, f32 f64 }

monoid_eff_impl! { StringMonoid, StringMonoid, String }
monoid_eff_impl! { IntAddMonoid, IntAddMonoid, u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 }

#[cfg(test)]
mod tests {
    use crate::typeclasses::monoid::*;

    #[test]
    fn test_strings() {
        let out = StringMonoid::empty();
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
