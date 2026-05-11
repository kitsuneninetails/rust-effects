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

pub fn pure<'a, A: Applicative<'a, T>, T>(t: T) -> A {
    A::pure(t)
}

#[cfg(test)]
mod test {}
