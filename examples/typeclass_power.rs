extern crate serde;
#[macro_use] extern crate serde_derive;
extern  crate serde_json;

use rust_effects::prelude::*;
use rust_effects::futures::prelude::*;
use std::ops::Add;
use serde_json::{to_string, from_str};
use rust_effects::futures::executor::block_on;

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
struct TestData {
    data: u32
}

fn db_call() -> u32 {
    println!("Simulating a long-running service, please wait...");
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
    10
}

fn main_caller<'a, FX, FR, A>(a: A) -> FR
    where
        A: Applicative<u32, FX=FX> + Monad<'a, u32, u32, FX=FX, FY=FX> + Functor<'a, u32, TestData, FX=FX, FY=FR>,
        FX: F<u32>,
        FR: F<TestData> {
    let f: FX = A::pure(IntAddMonoid::empty());
    let f = A::flat_map(f, |_| A::pure(db_call()));

    A::fmap(f, |data| TestData { data })
}

fn main() {
    println!("Running an effectful function should be the same whether we run in it inline or \
              async from a Future.  Running inline might be preferable for testing, etc. but we \
              don't want to change the code, only the calling 'effect'.");

    block_on(async {
        println!("Calling an effectful as a future, and then awaiting:");
        let f: ConcreteFuture<TestData> = main_caller(FutureEffect::apply());
        println!("Waiting on future now");
        println!("Output = {:?}", f.await);
    });

    println!("Calling an effectful as a result immediately:");
    let f: Result<TestData, String> = main_caller(ResultEffect::apply());
    println!("Output = {:?}", f.unwrap());


}

