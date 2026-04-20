use std::collections::HashMap;

use crate::CFuture;
use futures::future::lazy;

pub trait Monoid {
    fn empty() -> Self;
}

impl Monoid for u32 {
    fn empty() -> Self {
        0
    }
}

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

impl<A> Monoid for Option<A> {
    fn empty() -> Self {
        None
    }
}

impl<A, E: Monoid> Monoid for Result<A, E> {
    fn empty() -> Self {
        Err(E::empty())
    }
}

impl<A, V> Monoid for HashMap<A, V> {
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
}

pub fn empty<T: Monoid>() -> T {
    T::empty()
}

#[cfg(test)]
mod test {
    use crate::{
        CFuture,
        monoid::{Monoid, empty},
    };

    #[test]
    fn test_empty_option() {
        assert_eq!(Option::<u32>::empty(), None);
        assert_eq!(empty::<Option<u32>>(), None);
    }
    #[test]
    fn test_empty_result() {
        assert_eq!(Result::<u32, u32>::empty(), Err(0));
    }
    #[test]
    fn test_empty_vec() {
        assert_eq!(Vec::<u32>::empty(), []);
    }
    #[tokio::test]
    async fn test_empty_future() {
        assert_eq!(CFuture::<u32>::empty().await, 0);
    }
}
