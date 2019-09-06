pub mod option;
pub mod result;
pub mod vec;

trait F<X> {}

trait Semigroup<X> {
    fn combine(a: X, b: X) -> X;
}

trait Monoid {
    fn empty() -> Self;
}

trait Functor<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn fmap(f: FX, func: fn(X) -> Y) -> FY;
}

trait Functor2<FX, FY, FZ, X, Y, Z>
    where FX: F<X>,
          FY: F<Y>,
          FZ: F<Z> {
    fn fmap2(fa: FX, fb: FY, func: fn(&X, &Y) -> Z) -> FZ;
}

trait Productable<FX, FY, FXY, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          FXY: F<(X, Y)> {
    fn product(fa: FX, fb: FY) -> FXY;
}

trait Applicative<FX, X> {
    fn pure(x: X) -> FX;
}

trait Monad<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn flat_map(f: FX, func: fn(X) -> FY) -> FY;
}

trait Foldable<FX, X, Y>
    where FX: F<X> {
    fn fold(f: FX, init: Y, func: impl Fn(Y, X) -> Y) -> Y;
}

trait Traverse<FX, FY, AY, AR, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          AY: F<Y> + Applicative<AY, Y>,
          AR: F<FY> + Applicative<AR, FY>
{
    fn traverse(f: FX, func: fn(X) -> AY) -> AR;
}

impl Semigroup<String> for String {
    fn combine(a: String, b: String) -> String {
        format!("{}{}", a, b)
    }
}


//impl<X, Y> Functor<Vec<X>, Vec<Y>, X, Y> for Vec<X> {
//    fn fmap(f: Vec<X>, func: fn(X) -> Y) -> Vec<Y> {
//
//    }
//fn fmap(f: F<X>, func: fn(X) -> Y)
