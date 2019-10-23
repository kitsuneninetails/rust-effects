//use rust_effects::prelude::*;
//use std::{io, fs, env};
//
//#[macro_use] use rust_effects::io;
//use std::io::Read;
//use std::fs::read_to_string;
//
//fn ask_for_file<M: Monad, O: Monad>(ev: M) -> O {
//    M::flat_map(M::pure(()), O::pure(
//         {
//            println!("Input file name (io_example is a good one):");
//            let mut out = String::new();
//            io::stdin().read_line(&mut out);
//            env::current_dir()
//                .map(|mut p| {
//                    p.push("examples");
//                    p.push(out.trim());
//                    p
//                })
//                .map(|p| format!("{}", p.display()))
//                .unwrap_or(format!(""))
//        }
//    )
//}
//
//fn open_and_read<
//    F: Monad<String>,
//    G: Monad<Result<String, String>>>(m: F) -> G {
//    flat_map(t, move |file| IO::apply(move ||
//        {
//            fs::read_to_string(file)
//                .map_err(|e| format!("error reading file: {:?}", e))
//        }
//    ))
//}
//
//fn printout<
//    F: Monad<Result<String, String>>,
//    G: Monad<()>>(m: F) -> G {
//    flat_map(t, |contents| io!({
//        print!("{:?}", contents);
//    }))
//}
//
fn main() {
//    let m = ask_for_file::<IO<()>>(ev);
//    let m = open_and_read(m);
//    let m = printout(m);
//
//    println!("Now we have set up the IO, proceeding with the example...");
//    t.run_sync();
}
