use super::F;

pub trait Productable<FX, FY, FXY, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          FXY: F<(X, Y)> {
    fn product(&self, fa: FX, fb: FY) -> FXY;
}

pub fn product<FX, FY, FXY, X, Y>(ev: &impl Productable<FX, FY, FXY, X, Y>,
                                  fa: FX,
                                  fb: FY) -> FXY
    where FX: F<X>,
          FY: F<Y>,
          FXY: F<(X, Y)>{
    ev.product(fa, fb)
}
