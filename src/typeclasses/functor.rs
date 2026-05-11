/// The Functor typeclass
///
/// Functors are a mapping from one mathematical category to another.  In software
/// programming, there is only one category to consider: the types and transitions
/// built in to the programming language's grammar.  This reduces the Functor
/// typeclass to a simple mapping between two types.
///
/// To implement the Functor trait, a type must first be a "type constructor" which
/// accepts a type parameter to become a concrete type.  Then, it should delcare a type `F`:
/// ```text
/// type F: Functor<'a, U>;
/// ```
///
/// and implement the `fmap` function:
///
/// ```text
///  fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F;
/// ```
///
/// The type `F` declared should be the Functor type implementation, but typed
/// on U instead of T.  This is the output of the `fmap` functionh and this
/// constrains the output to be the same type as the source parameter.
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
/// ```
pub trait Functor<'a, T, U = ()> {
    type F: Functor<'a, U>;
    fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F;
}

/// Global `fmap` function
///
/// Calls the `fmap` implementation for type A.  
///
/// The `fmap` function accepts two arguments.  The first is a source object of typer `A` which
/// is parameterized on a single type `T`.  The second is a function or closure which takes
/// a piece of data of type `T` and returns data of type `U`.  The result of `fmap` is
/// an object parameterized on type `U` where the data of type `U` is determined by calling
/// the function/closure parameter using the source object's data of type `T`.
///
/// How the function applies to the source object is entirely dependent on the source object's
/// implementation for `fmap`, however, the `fmap` function is generally set to only run
/// on valid data owned by the source object.  Null, error, or empty data will not be altered,
/// making the returning object equivalent to the source object (meaning type `U` == type `T`
/// in that case).
///
/// All types can usually be inferred, making annotation unecessary.
///
/// Examples:
///
/// ```rust
/// use rust_effects::typeclasses::functor::fmap;
///  assert_eq!(fmap(Some(3), |i| i + 4), Some(7));
///  assert_eq!(fmap(vec![3, 4], |i| i + 4), vec![7, 8]);
/// ```
pub fn fmap<'a, A: Functor<'a, T, U>, T, U>(a: A, func: impl Fn(T) -> U + Send + 'a) -> A::F {
    A::fmap(a, func)
}

#[cfg(test)]
mod test {}
