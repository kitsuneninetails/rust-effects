use rust_effects::prelude::*;
use rust_effects::{option_monad, result_monad, vec_monad};

fn example_lib_func<'a, FX, X, FY, Y, M>(_: M, item: X, func: impl 'a + Fn(X) -> FY + Send + Sync) -> FY
    where
        FX: F<X>,
        FY: F<Y>,
        M: Monad<'a, FnctX=X, FnctY=Y, FctForX=FX, FctForY=FY>
{
    M::flat_map(M::pure(item), func)
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

    let r_4: Result<i32, ()> = example_lib_func(result_monad!(), 3, |x| Ok(x + 1));
    println!("Ok(3) passed with general add1 func -> {:?}", r_4);

    let op_4: Option<i32> = example_lib_func(option_monad!(), 3, |x| Some(x + 1));
    println!("Some(3) passed with general add1 func -> {:?}", op_4);

    let v_4 = example_lib_func(vec_monad!(), 2, |x| vec![x, x + 1]);
    println!("[2] passed with general add1 func -> {:?}", v_4);


}
