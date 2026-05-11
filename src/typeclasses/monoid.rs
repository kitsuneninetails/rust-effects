use crate::typeclasses::semigroup::Semigroup;
use paste::paste;

/// The Monoid Typeclass
///
/// Monoid describes the identity value for an implementing type.  Like Semigroup, a separate
/// identity element for multiplication is available, in case the additive and multiplicative
/// combination and identities are not identical.
///
/// To implement Monoid, a type must first implement the Semigroup typeclass as Monoid is an
/// extension of Monoid.  Then, implement the `empty` and `empty_m` function.  The default
/// implementation of empty_m just runs empty, in case there doesn't need to be a spearate
/// implementaiton for multiplicative identity.
///
/// ```rust
/// use rust_effects::prelude::*;
/// struct MyType(u32);
///
/// impl Semigroup for MyType {
///   fn combine(a: Self, b: Self) -> Self {
///     MyType(a.0 + b.0)
///   }
///
///   fn combine_m(a: Self, b: Self) -> Self {
///     MyType(a.0 * b.0)
///   }
/// }
///
/// impl Monoid for MyType {
///     fn empty() -> Self {
///         MyType(u32::empty())
///     }
///     fn empty_m() -> Self {
///         MyType(u32::empty_m())
///     }
/// }
///
/// assert_eq!(MyType::empty().0, 0);
/// assert_eq!(MyType::empty_m().0, 1);
/// assert_eq!(MyType::combine(MyType(3), MyType::empty()).0, 3);
/// assert_eq!(MyType::combine_m(MyType(3), MyType::empty_m()).0, 3);
/// ```
pub trait Monoid: Semigroup {
    fn empty() -> Self;
    fn empty_m() -> Self {
        Self::empty()
    }
}

#[macro_export]
macro_rules! monoid_num_impl {
    ($m:ty) => {
        paste! {
            impl Monoid for $m {
                fn empty() -> Self {
                    [<0 $m>]
                }
                fn empty_m() -> Self {
                    [<1 $m>]
                }
            }
        }
    };
}

monoid_num_impl! { u64 }
monoid_num_impl! { u32 }
monoid_num_impl! { u16 }
monoid_num_impl! { u8 }
monoid_num_impl! { i64 }
monoid_num_impl! { i32 }
monoid_num_impl! { i16 }
monoid_num_impl! { i8 }

monoid_num_impl! { f32 }
monoid_num_impl! { f64 }
monoid_num_impl! { usize }

impl Monoid for () {
    fn empty() -> Self {
        ()
    }
}

impl Monoid for String {
    fn empty() -> Self {
        "".to_string()
    }
}

/// Global function for `empty`
///
/// Calls the empty implementation for the type T.  T can be inferred if `empty` is called
/// and the result stored in a variable with type annotations or if empty is called as a
/// function parameter (in which case, the type inference would know based on the parameter's
/// type), otherwise, the type must be passed to `empty` when called.
///
/// Example:
/// ```rust
/// use rust_effects::typeclasses::monoid::empty;
///
/// assert_eq!(empty::<u32>(), 0);
///
/// let a: String = empty();
/// assert_eq!(a, "");
///
/// fn add(a:u32, b: u32) -> u32 { a + b }
/// assert_eq!(add(empty(), 3), 3);
/// ```
pub fn empty<T: Monoid>() -> T {
    T::empty()
}

/// Global function for `empty_m`.  
///
/// Calls the `empty_m` implementation for type T, returning the multiplicative identity
/// for T.  As with `empty`, the type must be provided to the call unless the result is being
/// used in such a way that the type can be inferred in reverse.
/// /// ```rust
/// use rust_effects::typeclasses::monoid::empty;
///
/// assert_eq!(empty_m::<u32>(), 01);
///
/// let a: String = empty_m();
/// assert_eq!(a, "");
///
/// fn mul(a:u32, b: u32) -> u32 { a * b }
/// assert_eq!(mul(empty_m(), 3), 3);
/// ```
pub fn empty_m<T: Monoid>() -> T {
    T::empty_m()
}

#[cfg(test)]
mod test {}
