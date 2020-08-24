// /// Option Typeclass Behaviors
// ///
// /// Semigroup (result-type must be a monoid for identity and parameters must be semigroup
// /// for combination)
// ///     `combine(Some(X), Some(Y)) => Some(combine(X, Y))`
// ///     `combine(Some(X), None) => Some(X)`
// ///     `combine(None, Some(Y)) => Some(Y)`
// ///     `combine(None, None) => None`
// /// Monoid
// ///     `empty() => None`
// /// Applicative
// ///     `pure(X) => Some(X)`
// /// ApplicativeApply
// ///     `apply(Some(fn X -> X2), Some(X)) => Some(fn(X)) => Some(X2)`
// ///     `apply(Some(fn X -> X2), None) => None`
// ///     `apply(None, Some(X)) => None`
// ///     `apply(None, None) => None`
// /// Functor
// ///     `fmap(Some(X), fn X -> X2) => Some(fn(X)) => Some(X2)`
// ///     `fmap(None, fn X -> X2) => None`
// /// Functor2
// ///     `fmap2(Some(X), Some(Y), fn X, Y -> Z) => Some(fn(X, Y)) => Some(Z)`
// ///     `fmap2(Some(X), None, fn X, Y -> Z) => None`
// ///     `fmap2(None, Some(Y), fn X, Y -> Z) => None`
// ///     `fmap2(None, None, fn X, Y -> Z) => None`
// /// Monad
// ///     `flat_map(Some(X), fn X -> Some(Y)) => fn(X) => Some(Y)`
// ///     `flat_map(Some(X), fn X -> None) => None`
// ///     `flat_map(None, fn X -> Some(Y)) => None`
// ///     `flat_map(None, fn X -> None) => None`
// /// MonadError
// ///     `raise_error(E) => None`
// ///     `handle_error(Some(X), fn E -> Some(X2)) => Some(X)`
// ///     `handle_error(Some(X), fn E -> None) => Some(X)`
// ///     `handle_error(None, fn E -> Some(X2)) => Some(X2)`
// ///     `handle_error(None, fn E -> None) => None`
// ///     `attempt(Some(X)) => Ok(X)`
// ///     `attempt(None) => Err(())`
// /// Foldable
// ///     `fold(Some(X), init, fn Y, X -> Y2) => fn(Y, X) => Y2`
// ///     `fold(None, init, fn Y, X -> Y2) => Y`
// ///     Note: Y and Y2 are the same type, just possibly two different values.
// /// Productable -
// ///     `product(Some(X), Some(Y)) => Some((X, Y))`
// ///     `product(Some(X), None) => None`
// ///     `product(None, Some(Y)) => None`
// ///     `product(None, None) => None`
// /// Traverse
// ///     `Not implemented`
//
// use super::prelude::*;
//
// use std::marker::PhantomData;
//
// impl<X> F<X> for Option<X> {}
//
// pub struct OptionSemigroup {}
//
// pub struct OptionMonad<X=(), Y=(), Z=()> {
//     _a: PhantomData<X>,
//     _b: PhantomData<Y>,
//     _c: PhantomData<Z>
// }
// impl<X, Y, Z> OptionMonad<X, Y, Z> {
//     pub fn new(_: Z) -> Self {
//         OptionMonad {
//             _a: PhantomData,
//             _b: PhantomData,
//             _c: PhantomData
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! option_semigroup {
//     () => (OptionSemigroup {})
// }
// #[macro_export]
// macro_rules! option_monad {
//     () => (OptionMonad::new(()))
// }
//
// impl Effect for OptionSemigroup {}
// impl<X, Y, Z> Effect for OptionMonad<X, Y, Z> {}
//
// impl<X, X2, XR> Semigroup<Option<X>, Option<X2>, Option<XR>> for OptionSemigroup {
//     fn combine<MX>(a: Option<X>, b: Option<X2>) -> Option<XR>
//         where
//             MX: Semigroup<X, X2, XR> {
//         match (a, b) {
//             (None, None) => None,
//             (Some(a), None) => Some(MX::combine(a, MX::empty())),
//             (None, Some(b)) => Some(MX::combine(MX::empty(), b)),
//             (Some(a), Some(b)) => Some(MX::combine(a, b))
//         }
//     }
// }
//
// impl<X> Monoid<Option<X>> for OptionSemigroup {
//     fn empty() -> Option<X> {
//         None
//     }
// }
//
// impl<'a, X, Y, Z> Functor<'a> for OptionMonad<X, Y, Z> {
//     type X = X;
//     type Y = Y;
//     type FX = Option<X>;
//     type FY = Option<Y>;
//     fn fmap(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::Y + Send + Sync) -> Self::FY {
//         f.map(func)
//     }
// }
//
// impl<'a, X, Y, Z> Applicative<'a> for OptionMonad<X, Y, Z> {
//     fn pure(x: X) -> Self::FX {
//         Some(x)
//     }
// }
//
// impl<'a, X, Y, Z, M> ApplicativeApply<'a, M> for OptionMonad<X, Y, Z>
//     where
//         M: 'a + Fn(Self::X) -> Self::Y + Send + Sync {
//     type FMapper = Option<M>;
//     fn apply(func: Self::FMapper, x: Self::FX) -> Self::FY {
//         func.and_then(|f| x.map(|x_in| f(x_in)))
//     }
// }
//
// impl<'a, X, Y, Z> Functor2<'a> for OptionMonad<X, Y, Z> {
//     type Z = Z;
//     type FZ = Option<Z>;
//     fn fmap2(fa: Self::FX,
//              fb: Self::FY,
//              func: impl 'a + Fn(Self::X, Self::Y) -> Self::Z + Send + Sync) -> Self::FZ {
//         fa.and_then(|i| fb.map(|j| func(i, j)))
//     }
// }
//
// impl<'a, X, Y, Z> Monad<'a> for OptionMonad<X, Y, Z> {
//     fn flat_map(f: Self::FX, func: impl 'a + Fn(Self::X) -> Self::FY + Send + Sync) -> Self::FY {
//         f.and_then(func)
//     }
// }
//
// impl<'a, X, Y, Z> Foldable<'a> for OptionMonad<X, Y, Z> {
//     type Y2=Y;
//     fn fold(f: Self::FX, init: Self::Y, func: impl 'a + Fn(Self::Y, Self::X) -> Y + Send + Sync) -> Self::Y2 {
//         match f {
//             Some(i) => func(init, i),
//             None => init
//         }
//     }
// }
//
// impl<'a, X, Y, Z> MonadError<'a> for OptionMonad<X, Y, Z> {
//     type E=();
//     fn raise_error(_err: Self::E) -> Self::FX {
//         None
//     }
//
//     fn handle_error(f: Self::FX, recovery: impl 'a + Fn(Self::E) -> Self::FX) -> Self::FX {
//         f.or_else(|| recovery(()))
//     }
//
//     fn attempt(f: Self::FX) -> Result<Self::X, Self::E> {
//         f.ok_or(())
//     }
// }
//
// impl<'a, X, Y> Productable<'a> for OptionMonad<X, Y, (X, Y)> {}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_semigroup() {
//         let a = Some(3);
//         let b = Some(5);
//
//         let out = OptionMonad::combine(a, b);
//         assert_eq!(Some(8), out);
//
//         let a = Some(3);
//         let b = None;
//
//         let out = OptionMonad::combine(a, b);
//         assert_eq!(Some(3), out);
//
//         let a = Some("Hello");
//         let b = Some(" World".to_string());
//
//         let out = OptionMonad::combine(a, b);
//         assert_eq!("Hello World", out.unwrap());
//
//         let a = Some(3);
//         let b = Some(4);
//
//         let out = OptionMonad::combine_inner::<IntMulMonoid>(a, b);
//         assert_eq!(Some(12), out);
//     }
//
//     #[test]
//     fn test_monoid() {
//         let out: Option<u32> = OptionMonad::empty();
//         assert_eq!(None, out);
//     }
//
//     #[test]
//     fn test_applicative() {
//         let out = OptionMonad::<_>::pure(3);
//         assert_eq!(Some(3), out);
//
//         let out: Option<_> = OptionMonad::pure("test");
//         assert_eq!(Some("test"), out);
//     }
//
//     #[test]
//     fn test_apply() {
//         let input: Option<u32> = OptionMonad::pure(3);
//         let out: Option<String> = OptionMonad::apply(Some(|i| format!("{} beans", i)), input);
//         assert_eq!(Some("3 beans".to_string()), out);
//
//         let input: Option<_> = OptionMonad::pure(3);
//         let input2: Option<_> = OptionMonad::pure(4);
//         let func = OptionMonad::pure(|i| { move |x| i + x });
//         let out2: Option<_> = OptionMonad::apply(OptionMonad::apply(func, input), input2);
//
//         assert_eq!(Some(7), out2);
//
//         let input: Option<i32> = OptionMonad::empty();
//         let input2: Option<_> = OptionMonad::pure(4);
//         let func = OptionMonad::pure(|i| { move |x| i + x });
//         let out2: Option<_> = OptionMonad::apply(OptionMonad::apply(func, input), input2);
//
//         assert_eq!(None, out2);
//     }
//
//     #[test]
//     fn test_functor() {
//         let out: Option<u32> = OptionMonad::pure(3);
//         let res = OptionMonad::fmap(out, |i| i + 4);
//         assert_eq!(Some(7), res);
//
//         let out: Option<String> = OptionMonad::pure(format!("Hello"));
//         let res = OptionMonad::fmap(out, |i| format!("{} World", i));
//         assert_eq!("Hello World", res.unwrap());
//
//         let out: Option<u32> = OptionMonad::empty();
//         let res = OptionMonad::fmap(out, |i| format!("{} World", i));
//         assert_eq!(None, res);
//
//         let out1: Option<u32> = OptionMonad::pure(3);
//         let out2: Option<String> = OptionMonad::pure(format!("Bowls"));
//         let res = OptionMonad::fmap2(out1, out2, |i, j| format!("{} {} of salad", i + 4, j));
//         assert_eq!("7 Bowls of salad", res.unwrap());
//     }
//
//     #[test]
//     fn test_monad() {
//         let out: Option<u32> = OptionMonad::pure(3);
//         let res = OptionMonad::flat_map(out, |i| Some(i + 4));
//         assert_eq!(Some(7), res);
//
//         let out: Option<u32> = OptionMonad::empty();
//         let res = OptionMonad::flat_map(out, |i| Some(i + 4));
//         assert_eq!(None, res);
//
//         let out: Option<u32> = OptionMonad::pure(2);
//         let res = OptionMonad::fold(out, 0, |init, i| init + i);
//         assert_eq!(2, res);
//
//         let out: Option<u32> = OptionMonad::empty();
//         let res = OptionMonad::fold(out, 0, |init, i| init + i);
//         assert_eq!(0, res);
//     }
//
//     #[test]
//     fn test_monaderror() {
//         let out: Option<u32> = OptionMonad::pure(3);
//         let res: Option<u32> = OptionMonad::flat_map(out, |_| OptionMonad::raise_error(()));
//         assert_eq!(None, res);
//     }
//
//     #[test]
//     fn test_product() {
//         let out1: Option<u32> = OptionMonad::pure(3);
//         let out2: Option<u32> = OptionMonad::pure(5);
//         let res = OptionMonad::product(out1, out2);
//         assert_eq!(Some((3, 5)), res);
//
//         let out1: Option<u32> = OptionMonad::pure(3);
//         let out2: Option<u32> = OptionMonad::empty();
//         let res = OptionMonad::product(out1, out2);
//         assert_eq!(None, res);
//     }
// }
