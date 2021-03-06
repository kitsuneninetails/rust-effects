extern crate serde;
#[macro_use] extern crate serde_derive;
extern  crate serde_json;

use rust_effects::prelude::*;
use rust_effects::{future_monad, result_monad};
use rust_effects::futures::executor::block_on;

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
struct TestData {
    data: u32
}

fn db_call<'a, A: Applicative<'a, X=u32, FX=FR>, FR: F<u32>>() -> FR {
    println!("Simulating a long-running service, please wait...");
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
    A::pure(10)
}

fn main_caller<'a, FX, FR, F1, F2>(_: F1, _: F2) -> FR
    where
        F1: 'a + Monad<'a, X=u32, FX=FX, Y=u32, FY=FX> + Send + Sync,
        F2: Functor<'a, X=u32, FX=FX, Y=TestData, FY=FR>,
        FX: F<u32>,
        FR: F<TestData> {
    let f: FX = F1::pure(IntAddMonoid::empty());
    let f = F1::flat_map(f, move |_| db_call::<F1, FX>());
    F2::fmap(f, |data| TestData { data })
}

fn main() {
    println!("Running an effectful function should be the same whether we run in it inline or \
              async from a Future.  Running inline might be preferable for testing, etc. but we \
              don't want to change the code, only the calling 'effect'.");

    block_on(async {
        println!("Calling an effectful as a future, and then awaiting:");
        let f: ConcreteFuture<_> = main_caller(future_monad!(), FutureEffect::new(()));
        println!("Waiting on future now");
        println!("Output = {:?}", f.await);
    });

    println!("Calling an effectful as a result immediately:");
    let f: Result<_, String> = main_caller(result_monad!(), ResultEffect::new(()));
    println!("Output = {:?}", f.unwrap());


}

