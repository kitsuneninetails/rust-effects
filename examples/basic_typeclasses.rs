extern crate rust_typeclasses;

use rust_typeclasses::prelude::*;
use std::ops::Add;

fn func_which_takes_monad_and_addables<'a, FX, X, T>(item: FX, add: X) -> FX
where
    X: 'a +  Add<Output=X> + Clone + Send + Sync,
    FX: F<X> + MonadEffect<'a, FX, FX, X, X> + ApplicativeEffect<X=X, Fct=T>,
    T: Applicative<FX, X>
{
    flat_map(item, move |x| pure(x + add.clone()))
}

fn main() {
    println!("Start with 'pure' option of 10 and result of 10");
    let o1: Option<u32> = pure(10);
    let r1: Result<u32, ()> = pure(10);

    let res = flat_map(o1, |x| r1.ok().map(|y| x + y));

    println!("Result of flatmap is {:?}", res);

    println!("If we combine it with an 'empty': {:?}",
             combine(res.clone(), empty()));

    println!("If we combine it with a Some(5) with Add: {:?}",
             combine(res.clone(), Some(5u32)));

    println!("BROKEN - If we combine it with a Some(5) with Mul: {:?}",
             combine(res.clone(), Some(5u32)));

    let op_4 = func_which_takes_monad_and_addables(Some(3), 1);
    println!("Some(3) passed to general add1 func -> {:?}", op_4);

    let r_4: Result<i32, ()> = func_which_takes_monad_and_addables(Ok(3), 1);
    println!("Ok(3) passed to general add1 func -> {:?}", r_4);

    let v_4 = func_which_takes_monad_and_addables(vec![2, 3, 4], 1);
    println!("[2,3,4] passed to general add1 func -> {:?}", v_4);


}
