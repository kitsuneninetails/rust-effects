use std::ops::{Mul, Add};
use super::F;

pub trait Monoid<T> {
    fn empty(self) -> T;
}

pub fn empty<T>(ev: impl Monoid<T>) -> T {
    ev.empty()
}

pub struct StringMonoid;
impl Monoid<String> for StringMonoid {
    fn empty(self) -> String { "".to_string() }
}

pub struct IntAddMonoid;
impl Monoid<u8> for IntAddMonoid {
    fn empty(self) -> u8 { 0 }
}
impl Monoid<i8> for IntAddMonoid {
    fn empty(self) -> i8 { 0 }
}
impl Monoid<u16> for IntAddMonoid {
    fn empty(self) -> u16 { 0 }
}
impl Monoid<i16> for IntAddMonoid {
    fn empty(self) -> i16 { 0 }
}
impl Monoid<u32> for IntAddMonoid {
    fn empty(self) -> u32 { 0 }
}
impl Monoid<i32> for IntAddMonoid {
    fn empty(self) -> i32 { 0 }
}
impl Monoid<u64> for IntAddMonoid {
    fn empty(self) -> u64 { 0 }
}
impl Monoid<i64> for IntAddMonoid {
    fn empty(self) -> i64 { 0 }
}
impl Monoid<f32> for IntAddMonoid {
    fn empty(self) -> f32 { 0.0 }
}
impl Monoid<f64> for IntAddMonoid {
    fn empty(self) -> f64 { 0.0 }
}

pub struct IntMulMonoid;
impl Monoid<u8> for IntMulMonoid {
    fn empty(self) -> u8 { 1 }
}
impl Monoid<i8> for IntMulMonoid {
    fn empty(self) -> i8 { 1 }
}
impl Monoid<u16> for IntMulMonoid {
    fn empty(self) -> u16 { 1 }
}
impl Monoid<i16> for IntMulMonoid {
    fn empty(self) -> i16 { 1 }
}
impl Monoid<u32> for IntMulMonoid {
    fn empty(self) -> u32 { 1 }
}
impl Monoid<i32> for IntMulMonoid {
    fn empty(self) -> i32 { 1 }
}
impl Monoid<u64> for IntMulMonoid {
    fn empty(self) -> u64 { 1 }
}
impl Monoid<i64> for IntMulMonoid {
    fn empty(self) -> i64 { 1 }
}
impl Monoid<f32> for IntMulMonoid {
    fn empty(self) -> f32 { 1.0 }
}
impl Monoid<f64> for IntMulMonoid {
    fn empty(self) -> f64 { 1.0 }
}

#[cfg(test)]
mod tests {
    use crate::typeclasses::monoid::*;

    #[test]
    fn test_strings() {
        let out = StringMonoid.empty();
        assert_eq!(out, "");
    }

    #[test]
    fn test_ints() {
        let out: i32 = IntAddMonoid.empty();
        assert_eq!(out, 0);

        let out: u32 = IntMulMonoid.empty();
        assert_eq!(out, 1);
    }

    #[test]
    fn test_functionals() {
        let out: i32 = empty(IntAddMonoid);
        assert_eq!(out, 0);

        let out = empty(StringMonoid);
        assert_eq!(out, "");
    }
}
