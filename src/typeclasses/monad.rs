#![allow(refining_impl_trait)]
use crate::typeclasses::applicative::Applicative;

/// The `Monad` type class
///
/// The good ole Monad, the contributor to a thousand confusions.  The dread
/// of functional programming trainees everywhere.  However, it still represents
/// a very simple concept.  Monads basically encapsulate the contept that the same
/// data can be realized or obtained in many different ways, depending on the
/// context of how the data is to be retrieved, and how the data might have context
/// associated with it as well: data retrieved in the future; data which may be null;
/// data which may not be a single, deterministic value; data which may contain
/// errors; and so on.
///
/// In addition, Monads also allow chaining functions which pass this data with
/// context and allow the chain of events to be shortcut should the context demand
/// it (such as null data no longer having functions run, etc.).
///
/// To implement the `Monad` type class, a deriving type must first declare the
/// `M` type, which will be the output of the `bind` function.  This is almost always
/// the deriving type parameterized on `U` instead of `T`.  Then, the `bind` function
/// must be implemented.  The first argument is the source type constructor, which
/// will be the deriving type parameterized on `T`.  The second argument is the
/// function to bind to the source, which should take data of type `T` and return
/// the type constructor parameterized on `U` (the `Self::M`` type defined above).
/// The behavior of the `bind` should be to apply to function to the source type
/// constructor should the source's state allow it.  The return of the `bind` will
/// then replace the source in the chain of calls, meaning the original source will
/// disappear as the `bind` functions are applied.
///
/// `Monad` derivations must also implement `Applicative` (and by extension `Functor`).
///
/// There are two other functions `lift_m1` and `lift_m2`, however, these are generally
/// defined in terms of `fmap` and `bind` calls and rarely need to be implemented by a
/// specific deriving type.
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
/// impl<'a, T: Send + 'a, U: Send + 'a> Monad<'a, T, U> for MyStruct<T> {
///     type M = MyStruct<U>;
///     fn bind(m: Self, func: impl FnOnce(T) -> Self::M + Send + 'a) -> Self::M {
///         func(m.0)
///     }
/// }
/// ```
///
/// The other two functions provided in the type class, `lift_m1` and `lift_m2`, are
/// used to `lift` regular functions into the context of the related `Monad`
/// implementation.  A normal function would just take data of `T` and return `U`,
/// but once lifted with `lift_m1`, it would take `M<T>` and return `M<U>`.  
/// Similarly, the `lift_m2`, the normal function would take two arguments and return
/// a new type, but the lifted function would take two pieces of data wrapped in the
/// `Monad` implementation and return an answer also wrapped.
///
/// This makes it easy to take a pre-defined function and make it run in the `Monad`
/// implementaiton's context, whether that be delayed data, nillable data, etc.
///
/// Note: The `lift_m1` and `lift_m2` functions provided as part of the `Monad` trait should
/// not be used.  Instead, use the global `lift_m1` and `lift_m2` functions/macros, as
/// they will have better ability for type inference and be much easier to use
/// effectively.
pub trait Monad<'a, T, U = ()>: Sized + Applicative<'a, T, U, F = Self::M> {
    type M: Monad<'a, U> + Send;
    fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M;
    fn lift_m1<S: Send + 'a, In: Monad<'a, S, T, M = Self>>(
        func: impl Fn(S) -> T + Send + Clone + 'a,
    ) -> impl Fn(In) -> Self {
        move |n: In| In::fmap(n, func.clone())
    }
    fn lift_m2<
        S1: Send + Clone + 'a,
        In1: Monad<'a, S1, T, M = Self> + Send + 'a,
        S2: Send + 'a,
        In2: Monad<'a, S2, T, M = Self> + Send + Clone + 'a,
    >(
        func: impl Fn(S1, S2) -> T + Send + Clone + 'a,
    ) -> impl Fn(In1, In2) -> Self {
        move |in1: In1, in2: In2| {
            let fnc_tmp = func.clone();
            In1::bind(in1, move |s1: S1| {
                let tmp = fnc_tmp.clone();
                In2::fmap(in2.clone(), move |s2: S2| tmp.clone()(s1.clone(), s2))
            })
        }
    }
}

pub fn bind<'a, M: Monad<'a, T, U>, T: Send + 'a, U: Send + 'a>(
    m: M,
    func: impl Fn(T) -> M::M + Send + 'a,
) -> M::M {
    M::bind(m, func)
}

/// The global `lift_m1` function
///
/// Calls the `lift_m1` implementation for type `In`.
///
/// The `lift_m1` function takes one argument: a function which converts a type `T` to
/// `U` (`T` and `U` can be the same).  The return will be a function which converts `In<T>`
/// to `In<U>` (declared as the trait type `In::M`) using the rules of `In`'s `fmap`
/// operation.
///
/// Using the function does mean the type parameters will need to be supplied, as it's not
/// possible for the type inference to figure it out without explicit hints (such as typing the
/// variable which received the result):
///
/// ```rust
/// use rust_effects::prelude::lift_m1;
/// let nilable_add4 = lift_m1::<Option<_>, _, _>(|a| a + 4);
/// assert_eq!(nilable_add4(Some(3)), Some(7));
/// assert_eq!(nilable_add4(None), None);
/// ```
///
/// Instead, using the `lift_m1!` macro eliminates all but the `Monad` implementation
/// desired, cutting down on redundant type declarations:
///
/// ```rust
/// use rust_effects::prelude::lift_m1;
/// let nilable_add4 = lift_m1![Option](|a| a + 4);
/// assert_eq!(nilable_add4(Some(3)), Some(7));
/// assert_eq!(nilable_add4(None), None);
/// ```
pub fn lift_m1<'a, In, S, T>(func: impl Fn(S) -> T + Send + Clone + 'a) -> impl Fn(In) -> In::M
where
    In: Monad<'a, S, T>,
    S: Send + 'a,
{
    In::M::lift_m1(func)
}

#[macro_export]
macro_rules! lift_m1 {
    ($m:tt) => {
        lift_m1::<$m<_>, _, _>
    };
}

/// The global `lift_m2` function
///
/// Calls the `lift_m2` implementation for type `In`.
///
/// The `lift_m2` function operates exactly like `lift_m1` except the provided function
/// takes two arguments, typed `S` and `T` and returns a third type `U`.  The lifted
/// function, likewise, takes two arguments, typed as `In1` and `In2`, which must be
/// the samne `Monad` implementation, typed as `In<S>` and `In<T>`.  The return is the
/// type `In1<U>` which was declared as the `In1::M` trait type.  Thus, the three `Monad`
/// implementations should be the same container, although the contained type is
/// different.
///
/// Like `lift_m1`, the type parameters will need to be supplied, as it's not possible
/// for the type inference to figure it out without explicit hints (such as typing the
/// variable which received the result), and there are more of them:
///
/// ```rust
/// use rust_effects::prelude::lift_m2;
/// let nilable_add = lift_m2::<Option<_>, _, _, _, _>(|a, b| a + b);
/// assert_eq!(nilable_add(Some(3), Some(4)), Some(7));
/// assert_eq!(nilable_add(Some(3), None), None);
/// ```
///
/// Instead, using the `lift_m2!` macro eliminates all but the `Monad` implementation
/// desired, cutting down on redundant type declarations:
///
/// ```rust
/// use rust_effects::prelude::lift_m2;
/// let nilable_add = lift_m2![Option](|a, b| a + b);
/// assert_eq!(nilable_add(Some(3), Some(4)), Some(7));
/// assert_eq!(nilable_add(Some(3), None), None);
/// ```
/// 
/// Note that the `lift_m2` function does NOT provide `combine` mechanics, even though
/// it has two parameters.  The point is to provide the context of the `Monad` to the
/// function, meaning that it will act more like a `bind` then `fmap` (in fact, this
/// is the default implementation of `lift_m2`), instead of a `combine`.
pub fn lift_m2<'a, In1, In2, S2, S1, T>(
    func: impl Fn(S1, S2) -> T + Send + Clone + 'a,
) -> impl Fn(In1, In2) -> In1::M
where
    In1: Monad<'a, S1, T> + Send + Clone + 'a,
    In2: Monad<'a, S2, T, M = In1::M> + Send + Clone + 'a,
    S2: Send + Clone + 'a,
    S1: Send + Clone + 'a,
{
    In1::M::lift_m2(func)
}

#[macro_export]
macro_rules! lift_m2 {
    ($m:tt) => {
        lift_m2::<$m<_>, _, _, _, _>
    };
}

#[cfg(test)]
mod test {
    use super::*;

    fn add4(x: u32) -> u32 {
        x + 4
    }

    fn add2(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test]
    fn test_lift1_macro() {
        let new_func = lift_m1![Option](add4);
        assert_eq!(new_func(Some(3)), Some(7));
        assert!(new_func(None).is_none());
    }
    #[test]
    fn test_lift2_macro() {
        let new_func = lift_m2![Option](add2);
        assert_eq!(new_func(Some(3), Some(4)), Some(7));
        assert!(new_func(Some(3), None).is_none());
        assert!(new_func(None, Some(4)).is_none());
        assert!(new_func(None, None).is_none());
    }
}
