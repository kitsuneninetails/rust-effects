use super::F;


pub trait Productable<FX, FY, FXY, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          FXY: F<(X, Y)> {
    fn product(fa: FX, fb: FY) -> FXY;
}

pub fn product<FX, FY, FXY, X, Y>(fa: FX, fb: FY) -> FXY
    where FX: F<X> + Productable<FX, FY, FXY, X, Y>,
          FY: F<Y>,
          FXY: F<(X, Y)>{
    FX::product(fa, fb)
}
