use super::F;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub trait Semigroup<T> {
    fn combine(self, a: T, b: T) -> T;
}

pub fn combine<T>(t: impl Semigroup<T>, a: T, b: T) -> T {
    t.combine(a, b)
}

pub struct StringSemigroup;

impl Semigroup<String> for StringSemigroup {
    fn combine(self, a: String, b: String) -> String { format!("{}{}", a, b) }
}

pub struct IntAddSemigroup;

impl<T: Add<Output=T>> Semigroup<T> for IntAddSemigroup {
    fn combine(self, a: T, b: T) -> T { a + b }
}

pub struct IntMulSemigroup;

impl<T: Mul<Output=T>> Semigroup<T> for IntMulSemigroup {
    fn combine(self, a: T, b: T) -> T { a * b }
}

pub struct OneParamSemigroup<X, T: Semigroup<X>> {
    pub t: T,
    _p: PhantomData<X>
}

impl<X, T: Semigroup<X>> OneParamSemigroup<X, T> {
    pub fn apply(t: T) -> Self {
        OneParamSemigroup {
            t,
            _p: PhantomData
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::typeclasses::semigroup::*;

    #[test]
    fn string_combine() {
        let out = StringSemigroup.combine(format!("Hello"), format!(" World"));
        assert_eq!("Hello World", out);
    }

    #[test]
    fn int_combine() {
        let out = IntAddSemigroup.combine(1, 2);
        assert_eq!(3, out);

        let out = Semigroup::combine(IntMulSemigroup, 1.2, 2.2);
        assert_eq!(2.64, out);
    }

    #[test]
    fn functional_combine() {
        let out = combine(IntMulSemigroup, 1, 2);
        assert_eq!(2, out);

        let out = combine(IntAddSemigroup, 5, 4);
        assert_eq!(9, out);
    }
}
