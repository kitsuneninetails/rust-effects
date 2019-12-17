//use super::{F, Effect};
//
///// Foldable is a typeclass which can provide a folding feature, which "rolls" up a type into a
///// new type.  This is accomplished via an initial value which is then iterated through the type,
///// accumulating a result value via the provided function (which takes the accumulated value and
///// the item in the iteration).  At the end, this accumulated value is returned.
/////
///// Typically, the result of a fold is the same type as the initial value, due to init and function
///// both operating on this value as the fold is accumulated.  However, a type `Y2` is provided here
///// to differentiate the final return from the accumulation function.  This allows types like
///// `Future` to accumulate values inside, yet still return a `Future` for that accumulated value
///// rather than blocking for the Future's completion.
//pub trait Foldable<'a> {
//    type FldInner;
//    type Fld: F<Self::FldInner>;
//    type Folded;
//    type Folded2;
//    fn fold(f: Self::Fld,
//            init: Self::Folded,
//            func: impl 'a + Fn(Self::Folded, Self::FldInner) -> Self::Folded2 + Send + Sync) -> Self::Folded2;
//}
//
//pub trait FoldableEffect<'a, X, Folded, Folded2> : F<X> + Sized
//    where <<Self as FoldableEffect<'a, X, Folded, Folded2>>::Fct as Foldable<'a>>::Fld: F<X> {
//    type Fct: Foldable<'a, FldInner=X, Folded=Folded, Folded2=Folded2, Fld=Self> + Effect;
//}
//
//pub fn fold<'a, FX, X, Folded, Folded2>(f: FX,
//                                        init: Folded,
//                                        func: impl 'a + Fn(Folded, X) -> Folded2 + Send + Sync) -> Folded2
//    where FX: FoldableEffect<'a, X, Folded, Folded2>,
//          <<FX as FoldableEffect<'a, X, Folded, Folded2>>::Fct as Foldable<'a>>::Fld: F<X>{
//    FX::Fct::fold(f, init, func)
//}
//
//
//
