use std::{collections::HashMap, hash::Hash};

use crate::CFuture;
use futures::FutureExt;

/// The Semigroup Typeclass
///
/// Semigroup describes the combination ability for two values of the implementing
/// type (for both addition and multiplication).  
///
/// To implement Semigroup, a type must implement the `combine` and `combine_m` function.  
/// The default implementation of combine_m just runs combine, in case there doesn't need to
/// be a spearate implementaiton for multiplicative combine.
///
/// ```rust
/// use rust_effects::semigroup::Semigroup;
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
/// let a = MyType(3);
/// let b = MyType(4);
/// assert_eq!(MyType::combine(a, b).0, 7);
///
/// let a = MyType(3);
/// let b = MyType(4);
/// assert_eq!(MyType::combine_m(a, b).0, 12);
/// ```
pub trait Semigroup: Sized {
    fn combine(a: Self, b: Self) -> Self;
    fn combine_m(a: Self, b: Self) -> Self {
        combine(a, b)
    }
}

#[macro_export]
macro_rules! sg_num_impl {
    ($m:ty) => {
        impl Semigroup for $m {
            fn combine(a: Self, b: Self) -> Self {
                a + b
            }
            fn combine_m(a: Self, b: Self) -> Self {
                a * b
            }
        }
    };
}

sg_num_impl! { u64 }
sg_num_impl! { u32 }
sg_num_impl! { u16 }
sg_num_impl! { u8 }
sg_num_impl! { i64 }
sg_num_impl! { i32 }
sg_num_impl! { i16 }
sg_num_impl! { i8 }

sg_num_impl! { f32 }
sg_num_impl! { f64 }
sg_num_impl! { usize }

impl Semigroup for () {
    fn combine(_a: Self, _b: Self) -> Self {
        ()
    }
}
impl Semigroup for String {
    fn combine(a: Self, b: Self) -> Self {
        a + &b
    }
}

impl<A: Semigroup> Semigroup for Option<A> {
    fn combine(a: Self, b: Self) -> Self {
        match (a, b) {
            (Some(t), Some(u)) => Some(combine(t, u)),
            (Some(t), None) => Some(t),
            (None, Some(u)) => Some(u),
            (None, None) => None,
        }
    }
    fn combine_m(a: Self, b: Self) -> Self {
        match (a, b) {
            (Some(t), Some(u)) => Some(combine_m(t, u)),
            (Some(t), None) => Some(t),
            (None, Some(u)) => Some(u),
            (None, None) => None,
        }
    }
}

impl<A: Semigroup, E: Semigroup> Semigroup for Result<A, E> {
    fn combine(a: Self, b: Self) -> Self {
        match (a, b) {
            (Ok(t), Ok(u)) => Ok(combine(t, u)),
            (Ok(t), Err(_)) => Ok(t),
            (Err(_), Ok(u)) => Ok(u),
            (Err(e), Err(e2)) => Err(combine(e, e2)),
        }
    }
    fn combine_m(a: Self, b: Self) -> Self {
        match (a, b) {
            (Ok(t), Ok(u)) => Ok(combine_m(t, u)),
            (Ok(t), Err(_)) => Ok(t),
            (Err(_), Ok(u)) => Ok(u),
            (Err(e), Err(e2)) => Err(combine_m(e, e2)),
        }
    }
}

impl<A> Semigroup for Vec<A> {
    fn combine(mut a: Self, b: Self) -> Self {
        a.extend(b);
        a
    }
}

impl<A: Eq + Hash + Clone, V: Semigroup + Clone> Semigroup for HashMap<A, V> {
    fn combine(a: Self, b: Self) -> Self {
        b.iter().fold(a, |mut a, (b_key, b_val)| {
            let res = a.insert(b_key.clone(), b_val.clone());
            if let Some(prev_val) = res {
                if let Some(v) = a.get_mut(&b_key) {
                    *v = V::combine(prev_val, b_val.clone());
                }
            }
            a
        })
    }
}

impl<'a, A: 'a + Send + Sync + Clone + Semigroup> Semigroup for CFuture<'a, A> {
    fn combine(a: Self, b: Self) -> Self {
        let new_fut = a
            .inner
            .then(move |a_res| b.inner.map(move |b_res| A::combine(a_res, b_res)));
        CFuture::new_fut(new_fut)
    }
    fn combine_m(a: Self, b: Self) -> Self {
        let new_fut = a
            .inner
            .then(move |a_res| b.inner.map(move |b_res| A::combine_m(a_res, b_res)));
        CFuture::new_fut(new_fut)
    }
}

/// Global `combine` function to combine two values of type T into a resulting T.
///
/// Type inference can almost always determine T from the parameters, so it's rarely
/// necessary to specify manually.
/// Example:
///
/// ```rust
/// use rust_effects::semigroup::combine;
/// assert_eq!(combine(3u32, 4u32), 7);
/// assert_eq!(combine("Hello ".to_owned(), "world".to_owned()), "Hello world");
pub fn combine<T: Semigroup>(a: T, b: T) -> T {
    T::combine(a, b)
}

/// Global `combine_m` function to multiplicatively combine two values of type T into
/// a resulting T.
///
/// Type inference can almost always determine T from the parameters, so it's rarely
/// necessary to specify manually.
/// Example:
///
/// ```rust
/// use rust_effects::semigroup::combine_m;
/// assert_eq!(combine_m(3u32, 4u32), 12);
/// assert_eq!(combine_m("Hello ".to_owned(), "world".to_owned()), "Hello world");
pub fn combine_m<T: Semigroup>(a: T, b: T) -> T {
    T::combine_m(a, b)
}

#[cfg(test)]
mod test {
    use crate::{
        CFuture,
        semigroup::{combine, combine_m},
    };

    #[test]
    fn test_combine_option() {
        assert_eq!(combine(Some(3), Some(4)), Some(7));
        assert_eq!(combine(Some(3), None), Some(3));
        assert_eq!(combine(None, Some(4)), Some(4));
        assert_eq!(combine(Option::<u32>::None, None), None);
        assert_eq!(combine_m(Some(3), Some(4)), Some(12));
        assert_eq!(combine_m(Some(3), None), Some(3));
        assert_eq!(combine_m(None, Some(4)), Some(4));
        assert_eq!(combine_m(Option::<u32>::None, None), None);
    }

    #[test]
    fn test_combine_result() {
        assert_eq!(combine(Ok::<_, u32>(3), Ok(4)), Ok(7));
        assert_eq!(combine(Ok(3), Err(4)), Ok(3));
        assert_eq!(combine(Err(3), Ok(4)), Ok(4));
        assert_eq!(combine(Err::<u32, _>(3), Err(4)), Err(7));
        assert_eq!(combine_m(Ok::<_, u32>(3), Ok(4)), Ok(12));
        assert_eq!(combine_m(Ok(3), Err(4)), Ok(3));
        assert_eq!(combine_m(Err(3), Ok(4)), Ok(4));
        assert_eq!(combine_m(Err::<u32, _>(3), Err(4)), Err(12));
    }
    #[test]
    fn test_combine_vec() {
        assert_eq!(combine(vec![3], vec![4]), vec![3, 4]);
        assert_eq!(combine(vec![3], vec![]), vec![3]);
        assert_eq!(combine(vec![], vec![4]), vec![4]);
        assert_eq!(combine::<Vec<u32>>(vec![], vec![]), vec![]);
    }
    #[tokio::test]
    async fn test_combine_future() {
        assert_eq!(combine(CFuture::lazy(3), CFuture::lazy(4)).await, 7);
    }
}
