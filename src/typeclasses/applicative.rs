use crate::typeclasses::functor::Functor;

/// The Applicative typeclass
///
/// Applicatives are an extension of `Functor` and generally fall between `Functor`
/// and `Monad` in terms of utility.  The functionality of APplicative is split
/// between `Applicative` and `ApplicativeFunctor` in order to better control the
/// necessary type parameters.  All Applicatives are Functors, meaning all types which
/// derive the `Applicative` typeclass must also derive `Functor`.
///
/// The basic version of the `Applicative` must only define the `pure` function.
/// The role of the `pure` function is simply to create a new Applicative derivation
/// from a piece of data.  All of these type classes are type constructors, meaning they
/// must accept one and only one type parameter `T` in order to realize the type and be
/// able to construct concrete data.  Thus, the `pure` function defines the type `T` and
/// provides a single piece of data in order to not only realize the type, but instantiate
/// it with concrete data.  The result of `pure` is this instantiated type constructor,
/// parameterized on type `T`, holding the data passed to the `pure` function.
///
/// Another effect of the `pure` function is to oppose the `empty` function from `Monoid`.
/// Where as the latter sets up data in such a way that it will be ignored by combinations,
/// mapping, and chaining, the `pure` function sets up data in such a way as to be ready for
/// combination and mapping.
///
/// To implement the Applicative trait, a type must implement the `Functor` type class.  
/// Then, it must only implement the `pure` function:
///
/// ```text
///  fn pure(a: T) -> Self;
/// ```
/// Example:
/// ```rust
/// use rust_effects::prelude::*;
/// struct MyStruct<T>(T);
///
/// impl<'a, T, U> Functor<'a, T, U> for MyStruct<T> {
///   type F = MyStruct<U>;
///   fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F {
///     MyStruct(func(m.0))
///   }
/// }
/// impl<'a, T, U> Applicative<'a, T, U> for MyStruct<T> {
///   fn pure(a: T) -> Self {
///     MyStruct(a)
///   }
/// }
/// ```
pub trait Applicative<'a, T, U = ()>: Functor<'a, T, U> {
    fn pure(a: T) -> Self;
}

/// Global `pure` function
///
/// Calls the `pure` implementation for type `A`.
///
/// The `pure` function takes two type parameters and one data parameter.  The
/// type parameters are the type `A`, which is the type constructor to create and
/// type `T`, the type of the data parameter.  The result of the `pure` function is
/// a type constructor containing the passed in data.
///
/// Since the type parameter `A` is completely reliant on the return type, it is
/// much more difficult for the Rusat type inference to figure it out without hints.
/// Unless the return from `pure` is directly consumed by a typed parameter or the
/// `let` is used with a type designation, the type parameter must be explicitly
/// stated,. which means the type `T` must also be present.  Usually, the type `A`,
/// being a type constructor for type `T` also has to mention the type `T`, forcing
/// a lot of redundant information into the type parameter declaration.  Fortunately,
/// _ can be used in most cases to omit the actual type T, since the data parameter
/// almost always sets that.  However, the usage is still one of the clunky examples:
///
/// ```rust
///   use rust_effects::prelude::pure;
///   let a: Option<u32> = pure(2);
///   let b = pure::<Option<_>, _>(2);
///   fn foo(_b: Option<u32>) {}
///   foo(pure(2));
/// ```
///
/// None of which are partiucularly compelling.  Thus, a macro has been provided.
///
/// # Pure Macro
///
/// The `pure!` macro takes the type of the type constructor in []s and allows type
/// inference to figure out `T`:
///
/// ```text
///   pure![Type](data)
/// ```
/// Example:
/// ```rust
///
///   use rust_effects::{prelude::pure};
///   assert_eq!(pure![Option](2), Some(2));
/// ```
pub fn pure<'a, A: Applicative<'a, T>, T>(t: T) -> A {
    A::pure(t)
}

#[macro_export]
macro_rules! pure {
    ($m:tt) => {
        pure::<$m<_>, _>
    };
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pure_macro() {
        assert_eq!(pure![Option](33), Some(33));
    }
}
