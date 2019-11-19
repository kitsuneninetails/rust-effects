use rust_effects::prelude::*;
use std::{io, fs, env};

#[macro_use] use rust_effects::io_monad;
use std::io::{Read, Error as IOError};
use std::fs::read_to_string;

fn ask_for_file<'a,
    S: SyncT<'a, FX=O, X=String>,
    O: F<String>
>(ev: S) -> S::FX {
    S::delay(|| {
        println!("Input file name (io_example is a good one):");
        let mut out = String::new();
        io::stdin().read_line(&mut out);
        out
    })
}

//fn open_and_read<'a,
//    S: SyncT<'a, X=String, FX=I, Y=String, FY=O>,
//    E: MonadError<'a, X=S::Y, FX=S::FY, E=IOError>,
//    I: F<String>,
//    O: F<String>
//>(ev: S, ev_out: E, input: S::FX) -> S::FY {
//    S::flat_map(input, move |out| {
//        S::suspend(move || {
//            match env::current_dir()
//                .map(|mut p| {
//                    p.push("examples");
//                    p.push(out.trim());
//                    p
//                })
//                .map(|p| format!("{}", p.display()))
//                .and_then(|f| {
//                    fs::read_to_string(f.clone())
//                }) {
//                Ok(s) => E::pure(s),
//                Err(e) => E::raise_error(e)
//            }
//        })
//    })
//}

//fn printout<'a,
//    S: Monad<'a, X=Result<String, String>, FX=I, Y=(), FY=O>,
//    T: SyncT<'a, X=(), FX=O>,
//    I: F<Result<String, String>>,
//    O: F<()>
//>(ev: S, evt: T, input: I) -> Result<O, IOError> {
//    S::flat_map(input, |contents| T::delay(move || {
//        print!("Output: {:?}", contents);
//        Ok(())
//    }))
//}
//
//fn ask_for_file_inferred<'a,
//    O: F<String> + SyncTEffect<'a, X=String>
//>() -> Result<O, IOError> {
//    flat_map(delay(|| String::new()), |_| {
//        println!("Input file name (io_example is a good one):");
//        let mut out = String::new();
//        io::stdin().read_line(&mut out);
//        out
//    })
//}
//
//fn open_and_read_inferred<'a,
//    I: F<String> + MonadEffect<'a, String, Result<String, String>, FX=I, FY=O>,
//    O: F<Result<String, String>> + SyncTEffect<'a, X=Result<String, String>>
//>(input: I) -> Result<O, IOError> {
//    flat_map(input, move |out| delay(move || {
//        let file = env::current_dir()
//            .map(|mut p| {
//                p.push("examples");
//                p.push(out.trim());
//                p
//            })
//            .map(|p| format!("{}", p.display()));
//        fs::read_to_string(file.clone())
//            .map_err(|e| format!("error reading file: {:?}", e))
//    }))
//}
//
//fn printout_inferred<'a,
//    I: F<Result<String, String>> + MonadEffect<'a, Result<String, String>, (), FX=I, FY=O>,
//    O: F<()> + SyncTEffect<'a, X=()>
//>(input: I) -> Result<O, IOError> {
//    flat_map(input, |contents| delay(move || {
//        print!("Output: {:?}", contents);
//        Ok(())
//    }))
//}

fn main() {
    let m: IO<String, IOError> = ask_for_file(io_monad!());
//    let m: IO<String, IOError> = open_and_read(io_monad!(), m);
//    let m: IO<(), IOError> = printout(io_monad!(), io_monad!(), m);
//
//    let m2: IO<String, IOError> = ask_for_file_inferred();
//    let m2: IO<Result<String, String>, IOError> = open_and_read_inferred(m2);
//    let m2: IO<(), IOError> = printout_inferred(m2);

//    let m3: IOResult<String> = IO::get_line();
//    let m3: IOResult<String> = flat_map(m3, IO::get_file(path));

    println!("Now we have set up the IO, proceeding with the example...");
    let output = m.run_sync();
    println!("Example 1 output = {:?}", output);
//    println!("Now example with inferred functions...");
//    m2.run_sync();
}
