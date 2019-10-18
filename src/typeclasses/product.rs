use super::{F, Effect};

pub trait Productable<X, Y>: Effect {
    type FX: F<X>;
    type FY: F<Y>;
    type FXY: F<(X, Y)>;
    fn product(fa: Self::FX, fb: Self::FY) -> Self::FXY;
}

pub trait ProductableEffect<X, Y> {
    type FX: F<X>;
    type FY: F<Y>;
    type FXY: F<(X, Y)>;
    type Fct: Productable<X, Y, FX=Self::FX, FY=Self::FY, FXY=Self::FXY> + Effect;
}

pub fn product<FX, FY, FXY, X, Y>(fa: FX,
                                  fb: FY) -> FXY
    where FX: F<X> + ProductableEffect<X, Y, FX=FX, FY=FY, FXY=FXY>,
          FY: F<Y>,
          FXY: F<(X, Y)>{
    FX::Fct::product(fa, fb)
}
