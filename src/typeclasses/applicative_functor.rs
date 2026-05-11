use crate::typeclasses::{applicative::Applicative, functor::Functor};

/// Applicative typeclass (Applicative Functor)
///
/// The other main function of an Applicative is the ability to sequence calls
/// similar to `fmap` for Functors.  This typeclass contains the ability to provide
/// this sequencing.  `ApplicativeFunctor` is separate from the main `Applicative``
/// typeclass due to the type parameters necessary to allow for the function.
///
/// To derive this type class, one must declare the types `AOut` and `AFunc`.
/// Respectively, these are the Output result of the `seq` function (almost always
/// the same as the deriving type with a <U> type parameter) and the function/closure
/// type (almost always same as the deriving type with an <F> parameter).  The `T`
/// type parameter to the trait is the source's contained type while `U` is the
/// generic type of the data contained in the output from `seq`.  The `F` type
/// parameter holds the function/closure and must implement the `Fn` trait.
///
/// After declaring the `AOut` and `AFunc` types, an implementing type must then
/// define the `seq` function.  THe type must also implement the `Applicative`
/// (and hence `Functor`) traits.
///
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
/// impl<'a, F, T, U> ApplicativeFunctor<'a, F, T, U> for MyStruct<T>
/// where
///     F: Fn(T) -> U,
///     T: Send + Clone + 'a, {
///   type AOut = MyStruct<U>;
///   type AFunc = MyStruct<F>;
///   fn seq(m: Self, func: Self::AFunc) -> Self::AOut  {
///     MyStruct(func.0(m.0))
///   }
/// }
/// ```
///
/// The sequencing funciton `seq` is different from the Functor's fmap only by
/// virtue of the provided function/closure also being wrapped in a type
/// constructor (where type `T` is the function type).  This allows the
/// application of the function/closure to also be determined by this type
/// constructor.  If the type constructor wrapping the function/closure is of
/// an empty variant, the function will not be applied to the source object.  Only
/// if the source object and the function/closure are wrapped in pure states will
/// the function be applied.  This adds flexibility to the application of the
/// function mapping.
///
/// In addition, the `seq` has one other neat sidce effect around currying.  If
/// given a curriable function, seq can allow for a mapping function/closure
/// with multiple parameters.  Typically, an `fmap` takes a source A<T>, applies
/// the fmap T -> U and gets a A<U>.  However, we may also want to have a mapping
/// S, T -> U, but this won't work with `fmap`.  Instead, we can seq to apply the
/// function to the source (the first parameter to the function), resulting in a
/// partially-applied functionb, which we then pass as the function/closure
/// parameter.  Using the second argument as the source for the scond call gets us
/// a resulting type constructor with the 2-argument function applied using two
/// arguments.  More arguments are possible with further seq calls.
///
/// ```text
/// pure + <*> pure 3 <*> pure 4
/// seq(pure(4), seq(pure(3), pure(add)))
/// ```
///
/// With rust-effets:
///
/// ```rust
///  use rust_effects::prelude::*;
///  fn add(a: u32) -> impl Fn(u32) -> u32 {
///    move |b| a + b
///  }
///  let res = seq(pure![Option](4), seq(pure![Option](3), pure![Option](add)));
/// ```
///
/// With `fmap`, this can only be done with unwrapping the options to pass it in
/// to the next step procedurally.
pub trait ApplicativeFunctor<'a, F: Fn(T) -> U, T, U = ()>: Applicative<'a, T, U> {
    type AOut: Applicative<'a, U>;
    type AFunc: Functor<'a, F, U, F = Self::AOut>;

    fn seq(m: Self, func: Self::AFunc) -> Self::AOut;
}

/// Global `seq` funtion.
///
/// Calls the `seq` implementation for type `A`.
///
/// The `seq` function takes two parameters: the source type constructor of type
/// `A<T>` and the function/closure wrapped in the same type constructor of type
/// `A<Func>`.  The type parameters are, in order, the type construtor `A`
/// parameterized on `T` (so `A<T>`), the function/closure type `M` (which must
/// implement the `Fn(U) -> T` trait), the source contained type `T` and the
/// return's contained type `U`.  These are almost always resolved by the type
/// inference, so rarely need to be specified.
///
/// Example:
/// ```rust
/// use rust_effects::prelude::seq;
/// assert_eq!(seq(Some(3), Some(|a| a + 3)), Some(6));
/// ```
pub fn seq<'a, A, M, T, U>(m: A, func: A::AFunc) -> A::AOut
where
    A: ApplicativeFunctor<'a, M, T, U>,
    M: Fn(T) -> U,
{
    A::seq(m, func)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::fmap;

    #[test]
    fn test_two_param_seq() {
        fn add(a: u32) -> impl Fn(u32) -> u32 {
            move |b| a + b
        }
        let res = seq(Some(4), seq(Some(3), Some(add)));
        assert_eq!(res, Some(7));
    }

    #[test]
    fn test_bad_two_param_fmap() {
        // Have to have a curryable function for the example
        fn add(a: u32) -> impl Fn(u32) -> u32 {
            move |b| a + b
        }

        // Using Option for example
        let add3 = fmap(Some(3u32), add); // Returns Some(impl Fn(u32) -> u32) = Some(|b| 3 + b)
        let res = fmap(Some(4), add3.unwrap()); // Won't compile without .unwrap()
        assert_eq!(res, Some(7));
        assert_eq!(fmap(Some(4), fmap(Some(3), add).unwrap()), Some(7)); // Compact
    }
}
