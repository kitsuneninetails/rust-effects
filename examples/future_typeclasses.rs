extern crate serde;
#[macro_use] extern crate serde_derive;
extern  crate serde_json;

use rust_effects::prelude::*;
use serde_json::{to_string, from_str};
use rust_effects::futures::executor::block_on;

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
struct TestData {
    data: u32
}

fn long_running_service() {
    println!("Simulating a long-running service, please wait...");
    ::std::thread::sleep(::std::time::Duration::from_secs(5));
}

fn for_comp_chain<'a>(a: u32) -> ConcreteFuture<'a, Result<u32, String>> {
    let r: Result<u32, String> = Ok(a);
    let f1: ConcreteFuture<Result<u32, String>> = pure(r);
    let f1 = flat_map(
        f1,
        |i| pure(
            i.and_then(|j| call_netcall_good(j))));
    let f1 = flat_map(
        f1,
        |i| pure(
            i.and_then(|j| from_str::<TestData>(j.as_ref()).map_err(|e| format!("Parse err: {:?}", e)))));
    let f1 = flat_map(
        f1,
        |i| pure(
            i.and_then(|j| call_db_good(j.data))));
    fmap(
        f1,
        |i| i.map(|j| j.data))
}

fn for_comp_chain_err<'a>(a: u32) -> ConcreteFuture<'a, Result<u32, String>> {
    let f1: ConcreteFuture<Result<u32, String>> = pure(Ok(a));
    let f1 = flat_map(
        f1,
        |i| pure(
            i.and_then(|j| call_netcall_bad(j))));
    let f1 = flat_map(
        f1,
        |i| pure(
            i.and_then(|j| from_str::<TestData>(&j).map_err(|e| format!("Parse err: {:?}", e)))));
    let f1 = flat_map(
        f1,
        |i| pure(
            i.and_then(|j| call_db_good(j.data))));
    fmap(
        f1,
        |i| i.map(|j| j.data))
}

fn call_db_good(a: u32) -> Result<TestData, String> {
    long_running_service();
    Ok(TestData {
        data: a + 20
    })
}

fn call_db_bad(_a: u32) -> Result<TestData, String> {
    long_running_service();
    Err("Server is down".to_string())
}

fn call_netcall_good(a: u32) -> Result<String, String> {
    long_running_service();
    let d = TestData {
        data: a + 100
    };
    to_string(&d).map_err(|e| format!("Serialize error: {:?}", e))
}

fn call_netcall_bad(_a: u32) -> Result<String, String> {
    long_running_service();
    Err("Network disconnected".to_string())
}

fn main() {
    block_on(async {
        println!("Using 'pure' on a function that takes a while will execute the function right \
                   away, as 'pure' for Future will use 'ready', which evaluates the value and \
                   prepares a Future just for that value (unlike 'lazy')");
        let f1: ConcreteFuture<Result<TestData, String>> = pure(call_db_good(10));
        println!("The future is now prepared, but we already had to wait (we haven't called 'await' \
                  yet!), so instead let's use a function that creates a chain starting with a pure \
                  on a real, concrete value before chaining a flat_map to the functions which take \
                  a while to execute.");
        println!("Run essentially a 'for' comprehension through a fake netcall, db call, and \
                  yield the 'data' member of the resulting struct");
        let f1: ConcreteFuture<Result<u32, String>> = for_comp_chain(10);
        println!("We have the future ready, but no code should have executed yet, so from now \
                  there should be a 5 second gap, followed by the output 'Ok(130)'.");
        println!("'for' comprehension output = {:?}", f1.await);

        println!("Here is the same, but with an error introduced early, leading to the error \
                  trickling down through: Err('Network disconnected')");
        let f1 = for_comp_chain_err(10);
        println!("'for' comprehension output = {:?}", f1.await);
    })

}

