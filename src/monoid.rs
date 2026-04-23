use paste::paste;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{CFuture, semigroup::Semigroup};
use futures::future::lazy;

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
/// use rust_effects::{monoid::Monoid, semigroup::Semigroup};
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

impl<A: Semigroup> Monoid for Option<A> {
    fn empty() -> Self {
        None
    }
}

impl<A: Semigroup, E: Monoid> Monoid for Result<A, E> {
    fn empty() -> Self {
        Err(E::empty())
    }
    fn empty_m() -> Self {
        Err(E::empty_m())
    }
}

impl<A: Eq + Hash + Clone, V: Semigroup + Clone> Monoid for HashMap<A, V> {
    fn empty() -> Self {
        HashMap::new()
    }
}

impl<A> Monoid for Vec<A> {
    fn empty() -> Self {
        vec![]
    }
}

impl<'a, A: Monoid + Send + Sync + Clone + 'a> Monoid for CFuture<'a, A> {
    fn empty() -> Self {
        CFuture::new_fut(lazy(|_| A::empty()))
    }
    fn empty_m() -> Self {
        CFuture::new_fut(lazy(|_| A::empty_m()))
    }
}

/// Global function for `empty`
///
/// Returns the empty implementation for the type T.  T can be inferred if `empty` is called
/// and the result stored in a variable with type annotations or if empty is called as a
/// function parameter (in which case, the type inference would know based on the parameter's
/// type), otherwise, the type must be passed to `empty` when called.
///
/// Example:
/// ```rust
/// use rust_effects::monoid::empty;
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
/// Returns the `empty_m` implementation for type T, returning the multiplicative identity
/// for T.  As with `empty`, the type must be provided to the call unless the result is being
/// used in such a way that the type can be inferred in reverse.
/// /// ```rust
/// use rust_effects::monoid::empty;
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
mod test {
    use crate::{
        CFuture,
        monoid::{Monoid, empty, empty_m},
        semigroup::{combine, combine_m},
    };

    #[test]
    fn test_empty_option() {
        assert_eq!(Option::<u32>::empty(), None);
        assert_eq!(empty::<Option<u32>>(), None);
        assert_eq!(empty_m::<Option<u32>>(), None);
    }
    #[test]
    fn test_empty_result() {
        assert_eq!(Result::<u32, u32>::empty(), Err(0));
        assert_eq!(Result::<u32, u32>::empty_m(), Err(1));
    }
    #[test]
    fn test_empty_vec() {
        assert_eq!(Vec::<u32>::empty(), []);
    }
    #[tokio::test]
    async fn test_empty_future() {
        assert_eq!(CFuture::<u32>::empty().await, 0);
        assert_eq!(CFuture::<u32>::empty_m().await, 1);
    }

    #[test]
    fn test_identity_option() {
        assert_eq!(combine(Some(3u32), empty()), Some(3u32));
        assert_eq!(combine(empty(), Some(3u32),), Some(3u32));
        assert_eq!(combine(empty::<Option<u32>>(), empty::<Option<_>>()), None);
        assert_eq!(combine_m(Some(3u32), empty_m()), Some(3u32));
        assert_eq!(combine_m(empty_m(), Some(3u32),), Some(3u32));
        assert_eq!(combine_m(empty_m::<Option<u32>>(), empty_m()), None);
    }
    #[test]
    fn test_identity_result() {
        assert_eq!(combine(Ok(3u32), Result::<u32, ()>::empty()), Ok(3u32));
        assert_eq!(combine(Result::<u32, ()>::empty(), Ok(3u32),), Ok(3u32));
        assert_eq!(
            combine(Result::<u32, u32>::empty(), Result::<u32, u32>::empty()),
            Err(0u32)
        );
        assert_eq!(combine_m(Ok(3u32), Result::<u32, ()>::empty_m()), Ok(3u32));
        assert_eq!(combine_m(Result::<u32, ()>::empty_m(), Ok(3u32),), Ok(3u32));
        assert_eq!(
            combine_m(Result::<u32, u32>::empty_m(), Result::<u32, u32>::empty_m()),
            Err(1u32)
        );
    }
    #[test]
    fn test_identity_vec() {
        assert_eq!(combine(vec![0, 1, 2], Vec::<u32>::empty()), vec![0, 1, 2]);
        assert_eq!(combine(Vec::<u32>::empty(), vec![0, 1, 2]), vec![0, 1, 2]);
        assert!(combine(Vec::<u32>::empty(), Vec::<u32>::empty()).is_empty());
    }
    #[tokio::test]
    async fn test_identity_future() {
        assert_eq!(
            combine(CFuture::lazy(3u32), CFuture::<u32>::empty()).await,
            3u32
        );
        assert_eq!(
            combine(CFuture::<u32>::empty(), CFuture::lazy(3u32)).await,
            3u32
        );
        assert_eq!(
            combine(CFuture::<u32>::empty(), CFuture::<u32>::empty()).await,
            0u32
        );
        assert_eq!(
            combine_m(CFuture::lazy(3u32), CFuture::<u32>::empty_m()).await,
            3u32
        );
        assert_eq!(
            combine_m(CFuture::<u32>::empty_m(), CFuture::lazy(3u32)).await,
            3u32
        );
        assert_eq!(
            combine_m(CFuture::<u32>::empty_m(), CFuture::<u32>::empty_m()).await,
            1u32
        );
    }
}
