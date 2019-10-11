use super::{F, Effect};

pub trait Productable<FX, FY, FXY, X, Y>: Effect
    where FX: F<X>,
          FY: F<Y>,
          FXY: F<(X, Y)> {
    fn product(fa: FX, fb: FY) -> FXY;
}

pub trait ProductableEffect<FX, FY, FXY, X, Y>
    where
        FX: F<X>,
        FY: F<Y>,
        FXY: F<(X, Y)> {
    type Fct: Productable<FX, FY, FXY, X, Y> + Effect;
}

pub fn product<FX, FY, FXY, X, Y>(fa: FX,
                                  fb: FY) -> FXY
    where FX: F<X> + ProductableEffect<FX, FY, FXY, X, Y>,
          FY: F<Y>,
          FXY: F<(X, Y)>{
    FX::Fct::product(fa, fb)
}
