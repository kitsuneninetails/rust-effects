use super::{F,
            applicative::*};

pub trait Traverse<FX, FY, AY, AR, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          AY: F<Y> + Applicative<AY, Y>,
          AR: F<FY> + Applicative<AR, FY>
{
    fn traverse(f: FX, func: fn(X) -> AY) -> AR;
}

pub fn traverse<FX, FY, AY, AR, X, Y>(f: FX ,
                                      func: fn(X) -> AY,) -> AR
    where FX: F<X> + Traverse<FX, FY, AY, AR, X, Y>,
          FY: F<Y>,
          AY: F<Y> + Applicative<AY, Y>,
          AR: F<FY> + Applicative<AR, FY> {
    FX::traverse(f, func)
}
