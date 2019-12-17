//use super::{Effect,
//            foldable::*,
//            functor::*};
//
///// Basically take a Traversable type constructor T, wrapping a set of concrete types X, and a
///// function which maps the X into a compatible type constructor E (usually an effect, like Future,
///// IO, Result, etc.) which wraps a concrete type Y.
/////   _____T_____
///// /            \
///// (* -> *) -> *
/////     E       X
/////
///// Traverse calls the function on each item in the traversable, to generate an interim structure
///// with the Traversable holding the results of the computation.  Then, Traverse will flip the
///// structure, with a single effect E wrapping the Traversable of concrete Y values.
/////
///// e.g. T<X> => traverse (gets a T<E<Y>> as an interim value) => E<T<Y>>
//fn foobar() {}
////pub trait Traverse<'a>: Functor<'a> + Foldable<'a> {
////    fn traverse(f: Self::FX,
////                func: impl 'a + Fn(<Self as Functor>::X) -> Self::FY + Send + Sync) -> Self::FZ;
////}
////
////pub trait TraverseEffect<'a, Y, Z>: FunctorEffect<'a, Y> + FoldableEffect<'a, <Self as FunctorEffect<'a, Y>>::X, Y, Z> + Sized {
////    type Fct: Traverse<'a, FX=Self, FY=Self::FY, FZ=Self::FZ, X=Self::X, Y=Self::Y, Z=Self::Z> + Effect;
////}
////
////pub fn traverse<'a, FX, Y, Z>(
////    f: FX,
////    func: impl 'a + Fn(FX::X) -> FX::FY + Send + Sync) -> FX::FZ
////where
////    FX: TraverseEffect<'a, Y, Z> {
////    FX::Fct::traverse(f, func)
////}
