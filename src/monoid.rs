use paste::paste;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{CFuture, semigroup::Semigroup};
use futures::future::lazy;

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

pub fn empty<T: Monoid>() -> T {
    T::empty()
}

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
