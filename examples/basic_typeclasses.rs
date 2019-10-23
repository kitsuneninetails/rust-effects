use rust_effects::prelude::*;
use std::ops::Add;
use rust_effects::{option_monad, result_monad, vec_monad};

fn func_which_takes_monad_and_addables<'a, FX, X, M>(M: M, item: FX, add: X) -> FX
where
    X: 'a +  Add<Output=X> + Clone + Send + Sync,
    FX: F<X>,
    M: Monad<'a, X=X, FX=FX, Y=X, FY=FX>
{
    M::flat_map(item, move |x| M::pure(x + add.clone()))
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

    println!("If we combine it with a Some(5) with Mul: {:?}",
             combine_inner::<_, _, IntMulSemigroup>(res.clone(), Some(5u32)));

    let r_4: Result<i32, ()> = func_which_takes_monad_and_addables(result_monad!(), Ok(3), 1);
    println!("Ok(3) passed to general add1 func -> {:?}", r_4);

    let op_4: Option<i32> = func_which_takes_monad_and_addables(option_monad!(), Some(3), 1);
    println!("Some(3) passed to general add1 func -> {:?}", op_4);

    let v_4 = func_which_takes_monad_and_addables(vec_monad!(), vec![2, 3, 4], 1);
    println!("[2,3,4] passed to general add1 func -> {:?}", v_4);


}
