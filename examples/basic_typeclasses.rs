extern crate rust_typeclasses;

use rust_typeclasses::prelude::*;
use std::ops::Add;

fn func_which_takes_monad_and_addables<'a, FX, X, M>(tc: &'a M, item: FX, add: X) -> FX
where
    X: 'a +  Add<Output=X> + Clone + Send + Sync,
    M: Applicative<FX, X> + Send + Sync,
    FX: F<X> + MonadEffect<'a, FX, FX, X, X>
{
    flat_map(item, move |x| pure(tc, x + add.clone()))
}

fn main() {
    println!("Start with 'pure' option of 10 and result of 10");
    let o1 = pure(OP_EV, 10u32);
    let r1: Result<u32, ()> = pure(RES_EV, 10u32);

    let res = flat_map(o1, |x| r1.ok().map(|y| x + y));

    println!("Result of flatmap is {:?}", res);

    println!("If we combine it with an 'empty': {:?}",
             combine(OP_EV.sg(IADD_SG), res.clone(), empty(OP_EV)));

    println!("If we combine it with a Some(5) with Add: {:?}",
             combine(OP_EV.sg(IADD_SG), res.clone(), Some(5u32)));

    println!("If we combine it with a Some(5) with Mul: {:?}",
             combine(OP_EV.sg(IMUL_SG), res.clone(), Some(5u32)));

    let op_4 = func_which_takes_monad_and_addables(OP_EV, Some(3), 1);
    println!("Some(3) passed to general add1 func -> {:?}", op_4);

    let r_4: Result<i32, ()> = func_which_takes_monad_and_addables(RES_EV, Ok(3), 1);
    println!("Ok(3) passed to general add1 func -> {:?}", r_4);

    let v_4 = func_which_takes_monad_and_addables(VEC_EV, vec![2, 3, 4], 1);
    println!("[2,3,4] passed to general add1 func -> {:?}", v_4);


}
