///// Vector Typeclass Behaviors
/////
///// Semigroup
/////     `combine(vec![A, B, ...], vec![X, Y, ...]) => vec![A, B, ..., X, Y, ...]`
///// Monoid
/////     `empty() => vec![]`
///// Applicative
/////     `pure(X) => vec![X]`
///// Functor
/////     `fmap(vec![X, Y, ..], fn T1 -> T2) => vec![fn(X), fn(Y), ...]`
///// Functor2
/////     `fmap(vec![A, B, ...], vec![X, Y, ...], fn T1 T2 -> T3) => vec![fn(A, X), fn(A, Y), ..., fn(B, X), ...]`
///// Monad
/////     `flat_map(vec![X, Y, ...], fn T1 -> Vec T2 => vec![fn(X)[0], fn(X)[1], ..., fn(Y)[0], ...]`
/////     Note: The brackets [] denote that the returned vectors are flattened in place.
///// Foldable
/////     `fold(vec![X, Y, ...], init, fn TI T1 -> TI) => fn...(fn(fn(init, X), Y), ...)`
///// Productable
/////     `product(vec![A, B, ...], vec![X, Y, ...]) => vec![(A, X), (A, Y), ... (B, X), ...]`
///// Traverse -
/////     `traverse(vec![X, Y, ...], fn T1 -> F<T2>) => F<vec![*fn(X), *fn(Y), ...]>`
/////     Note: The `*` means the inner item from the F<_> returned from the function call.
//
//use crate::prelude::*;
//use crate::*;
//
//use std::marker::PhantomData;
//
//impl<X> F<X> for Vec<X> {}
//
//semigroup_effect! { 1A, Vec, VecEffect }
//monoid_effect! { 1, Vec, VecEffect }
//applicative_effect! { 1C, Vec, VecEffect }
//functor_effect! { 1, Vec, VecEffect }
//functor2_effect! { 1C, Vec, VecEffect }
//monad_effect! { 1C, Vec, VecEffect }
//foldable_effect! { 1C, Vec, VecEffect }
//productable_effect! { 1C, Vec, VecEffect }
//
//
//pub trait TraverseEffect<'a, Efct, TRet, EfctRet, X, Y>: F<X> + Sized
//    where Efct: F<Y>,
//          TRet: F<Y>,
//          EfctRet: F<TRet> {
//    type Fct = VecEffect<X, Y, ()>;
//}
//
//#[derive(Clone)]
//pub struct VecEffect<X=(), Y=(), Z=()> {
//    _a: PhantomData<X>,
//    _b: PhantomData<Y>,
//    _c: PhantomData<Z>
//}
//
//impl<X, Y, Z> VecEffect<X, Y, Z> {
//    pub fn new(_: Z) -> Self {
//        VecEffect {
//            _a: PhantomData,
//            _b: PhantomData,
//            _c: PhantomData
//        }
//    }
//}
//
//#[macro_export]
//macro_rules! vec_monad {
//    () => (VecEffect::new(()))
//}
//
//impl<X, Y, Z> Effect for VecEffect<X, Y, Z> {}
//
//impl<X> Semigroup<Vec<X>, Vec<X>, Vec<X>> for VecEffect<X, X, X> {
//    fn combine(a: Vec<X>, b: Vec<X>) -> Vec<X> {
//        let mut ret = a;
//        ret.extend(b);
//        ret
//    }
//}
//
//impl<X, Y, Z> Monoid for VecEffect<X, Y, Z> {
//    type M = Vec<X>;
//    fn empty() -> Self::M {
//        vec![]
//    }
//}
//
//impl<'a, X, Y, Z> Functor<'a> for VecEffect<X, Y, Z> {
//    type FnctX = X;
//    type FnctY = Y;
//    type FnctZ = Z;
//    type FctForX = Vec<X>;
//    type FctForY = Vec<Y>;
//    type FctForZ = Vec<Z>;
//    fn fmap(f: Self::FctForX, func: impl 'a + Fn(Self::FnctX) -> Self::FnctY + Send + Sync) -> Self::FctForY {
//        f.into_iter().map(func).collect()
//    }
//    fn fmap2(fa: Self::FctForX,
//             fb: Self::FctForY,
//             func: impl 'a + Fn(Self::FnctX, Self::FnctY) -> Self::FnctZ + Send + Sync) -> Self::FctForZ {
//        fa.into_iter().flat_map(|i| {
//            let ret: Vec<Z> = fb.iter().map(|j| func(i.clone(), j.clone())).collect();
//            ret
//        }).collect()
//    }
//}
//
//impl<'a, X, Y, Z> Functor2<'a> for VecEffect<X, Y, Z>
//    where X: Clone,
//          Y: Clone {
//}
//
//impl<'a, X, Y, Z> Applicative<'a> for VecEffect<X, Y, Z>
//    where
//        X: Clone,
//        Y: Clone {
//    fn pure(x: X) -> Self::FX {
//        vec![x]
//    }
//}
//
//impl<'a, X, Y, Z, M> ApplicativeApply<'a, M> for VecEffect<X, Y, Z>
//    where
//        X: Clone,
//        Y: Clone,
//        M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
//    type FMapper = Vec<M>;
//    fn apply(_func: Self::FMapper, _x: Self::FX) -> Self::FY {
//        vec![]
//    }
//}
//
//impl<'a, X, Y, Z> Monad<'a> for VecEffect<X, Y, Z>
//    where
//        X: Clone,
//        Y: Clone {
//    fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//        f.into_iter().flat_map(func).collect()
//    }
//}
//
//impl<'a, X, Y, Z> Foldable<'a> for VecEffect<X, Y, Z>
//    where
//        X: Clone,
//        Y: Clone {
//    type Fld = Vec<X>;
//    type FldInner = X;
//    type Folded = Y;
//    type Folded2 = Y;
//    fn fold(f: Self::Fld,
//            init: Self::Folded,
//            func: impl 'a + Fn(Self::Folded, Self::FldInner) -> Self::Folded2 + Send + Sync) -> Self::Folded2 {
//        f.into_iter().fold(init, func)
//    }
//}
//
//impl<'a, X: Clone, Y: Clone> Productable<'a> for VecEffect<X, Y, (X, Y)> {
//    fn product(fa: Self::FX, fb: Self::FY) -> Self::FZ {
//        fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
//    }
//}
//
////impl<'a, X, Y, Z> Traverse<'a> for VecEffect<X, Y, Z>
////    where
////        X: Clone,
////        Y: Clone {
////    fn traverse(f: Self::FX,
////                func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FZ {
////        // Initialize the fold to the pure value of the resulting effect (Future, Option, IO, etc.)
////        // Takes an empty vector of Y to start with
////        let emt: Vec<Self::Y> = empty();
////        let init: Self::FZ = pure(emt);
////
////        // Fold on the initial list (Vec<X>) and start with initial accumulator set to
////        // A basic E<Vec<Y>> where E is the effect that will be returned from the specified
////        // function (Vec, Future, Either, etc.).
////        fold(f, init, |y: Self::FZ, x: X| {
////            // The folding function should take this effect (Vec, Future, etc.) and
////            // "combine" the results with the accumulated value.  This is what determines
////            // whether the accumulated value turns into a "negative" result (like a None,
////            // or a Future::fail(), or a Either::Err, etc.)
////
////            // Apply a map between the returned value and the accumulated value.  The
////            // mapping function should know how to put the two together (they are the same
////            // effect type, but they each hold a different type inside).
////            let ret_vec: Self::FZ = fmap2(
////                func(x), // Type = Self::FY = F<Y>
////                y, // Type = Self::FZ = F<Vec<Y>>
////                |efct_inner: Y, y_inner: Vec<Y>| {
////                    // This function adds the returned inner value onto the accumulating list
////                    // inside the effect.  Functors know how to only allow this
////                    // combination if both the accumulated effect and the returned
////                    // effect both match up to "positive" values (like success or Some()).
////                    // These next lines won't even get called unless that is the case.
////                    let efct_vec: Vec<Y> = pure(efct_inner);
////                    VecEffect::combine(y_inner, efct_vec)
////                });
////            ret_vec
////        })
////    }
////}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_semigroup() {
//        let a = vec![3, 4, 5];
//        let b = vec![6, 7, 8];
//
//        let out = combine(a, b);
//        assert_eq!(vec![3, 4, 5, 6, 7, 8], out);
//
//        let a = vec![3, 4, 5];
//        let b = vec![];
//
//        let out = combine(a, b);
//        assert_eq!(vec![3, 4, 5], out);
//
//        let a = vec!["Hello".to_string()];
//        let b = vec!["World".to_string()];
//
//        let out = combine(a, b);
//        assert_eq!(vec![format!("Hello"), format!("World")], out);
//    }
//
//    #[test]
//    fn test_monoid() {
//        let out: Vec<u32> = empty();
//        assert!(out.is_empty());
//    }
//
//    #[test]
//    fn test_applicative() {
//        let out = <Vec::<u32> as ApplicativeEffect>::Fct::pure(3);
//        assert_eq!(vec![3], out);
//        let out: Vec<&str> = pure("test");
//        assert_eq!(vec!["test"], out);
//    }
//
//    #[test]
//    fn test_functor() {
//        let out: Vec<u32> = pure(3);
//        let res = fmap(out, |i| i + 4);
//        assert_eq!(vec![7], res);
//
//        let out: Vec<String> = pure(format!("Hello"));
//        let res = fmap(out, |i| format!("{} World", i));
//        assert_eq!(vec![format!("Hello World")], res);
//
//        let out: Vec<String> = empty();
//        let res = fmap(out, |i| format!("{} World", i));
//        assert!(res.is_empty());
//
//        let out1: Vec<u32> = pure(3);
//        let out2: Vec<String> = pure(format!("Bowls"));
//        let res = fmap2(out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
//        assert_eq!(vec![format!("7 Bowls of salad")], res);
//    }
//
//    #[test]
//    fn test_monad() {
//        let out: Vec<u32> = pure(3);
//        let res = flat_map(out, |i| vec![i + 1, i + 2, i + 3]);
//        assert_eq!(vec![4, 5, 6], res);
//
//        let out: Vec<u32> = vec![3, 4];
//        let res = flat_map(out, |i| vec![i + 1, i + 2, i + 3]);
//        assert_eq!(vec![4, 5, 6, 5, 6, 7], res);
//
//        let out: Vec<u32> = empty();
//        let res = flat_map(out, |i| vec![i + 1, i + 2, i + 3]);
//        assert!(res.is_empty());
//
//        let out: Vec<u32> = vec![3, 4, 5];
//        let res: Vec<u32> = flat_map(out, |_i| empty());
//        assert!(res.is_empty());
//
//        let out: Vec<u32> = vec![2, 3, 4];
//        let res = fold(out, 0, |init, i| init + i);
//        assert_eq!(9, res);
//
//        let out: Vec<u32> = empty();
//        let res = fold(out, 0, |init, i| init + i);
//        assert_eq!(0, res);
//    }
//
//    #[test]
//    fn test_product() {
//        let out1: Vec<u32> = vec![2, 3];
//        let out2: Vec<u32> = vec![4, 5];
//        let res = product(out1, out2);
//        assert_eq!(vec![(2, 4), (2, 5), (3, 4), (3, 5)], res);
//
//        let out1: Vec<u32> = vec![2, 3];
//        let out2: Vec<u32> = empty();
//        let res = product(out1, out2);
//        assert!(res.is_empty());
//    }
//
////    #[test]
////    fn test_traverse() {
////        let o = traverse(vec![2, 4, 6], |x| match x % 2 == 0 {
////            true => Some(format!("{}", x)),
////            false => None
////        });
////        assert!(o.is_some());
////        let v = o.unwrap();
////        assert!(v.contains(&"2".to_string()));
////        assert!(v.contains(&"4".to_string()));
////        assert!(v.contains(&"6".to_string()));
////    }
////
////    #[test]
////    fn test_traverse_err() {
////        let o = traverse(vec![2, 5, 6], |x| match x % 2 == 0 {
////            true => Some(format!("{}", x)),
////            false => None
////        });
////        assert!(o.is_none());
////    }
//
//}
