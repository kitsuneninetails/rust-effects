//extern crate serde;
//#[macro_use] extern crate serde_derive;
//extern  crate serde_json;
//
//use rust_effects::prelude::*;
//use rust_effects::futures::future::lazy;
//use serde_json::from_str;
//
//#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
//struct TestData {
//    data: u32
//}
//
//fn long_running_service() {
//    println!("Simulating a long-running service, please wait...");
//    ::std::thread::sleep(::std::time::Duration::from_secs(2));
//}
//
//fn parallel_call1(i: u32) -> Box<dyn Fn() -> Result<TestData, String> + Send + Sync> {
//    Box::new(
//        move || Ok(TestData {
//            data: i + 10
//    }))
//}
//
//fn parallel_call2(i: u32) -> Box<dyn Fn() -> Result<TestData, String> + Send + Sync> {
//    Box::new(
//        move || Ok(TestData {
//            data: i + 100
//    }))
//}
//
//fn parallel_call3(i: String) -> Box<dyn Fn() -> Result<TestData, String> + Send + Sync> {
//    Box::new(
//        move || from_str::<TestData>(&i).map_err(|e| format!("{:?}", e))
//    )
//}
//
//fn fut_caller<'a, T>(f: T) -> ConcreteFuture<'a, Result<TestData, String>>
//    where
//        T: 'a + Fn() -> Result<TestData, String> + Send + Sync {
//    fut(lazy(move |_| {
//        long_running_service();
//        f()
//    }))
//}

fn main() {
//    block_on(async {
//        let calls = vec![
//            parallel_call1(10),
//            parallel_call2(20),
//            parallel_call3(format!("{{ \"data\": 30 }}"))
//        ];
//
//        let combined_fut = traverse(calls, fut_caller);
//        println!("The vector of futures is now a future of a vector of all the pending results \
//                 (will be set once the future is run); running future now...");
//        let res_vec = combined_fut.await;
//        println!("The output vector is 0={:?}, 1={:?}, 2={:?}", res_vec[0], res_vec[1], res_vec[2]);
//    })

}
