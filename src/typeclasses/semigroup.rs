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

/// Calls the `combine` function to combine two values of type T into a resulting T.
///
/// Type inference can almost always determine T from the parameters, so it's rarely
/// necessary to specify manually.
/// Example:
///
/// ```rust
/// use rust_effects::typeclasses::semigroup::combine;
/// assert_eq!(combine(3u32, 4u32), 7);
/// assert_eq!(combine("Hello ".to_owned(), "world".to_owned()), "Hello world");
pub fn combine<T: Semigroup>(a: T, b: T) -> T {
    T::combine(a, b)
}

/// Calls the `combine_m` function to multiplicatively combine two values of type T into
/// a resulting T.
///
/// Type inference can almost always determine T from the parameters, so it's rarely
/// necessary to specify manually.
/// Example:
///
/// ```rust
/// use rust_effects::typeclasses::semigroup::combine_m;
/// assert_eq!(combine_m(3u32, 4u32), 12);
/// assert_eq!(combine_m("Hello ".to_owned(), "world".to_owned()), "Hello world");
pub fn combine_m<T: Semigroup>(a: T, b: T) -> T {
    T::combine_m(a, b)
}

#[cfg(test)]
mod test {}
