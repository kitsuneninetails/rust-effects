use rust_effects::prelude::*;
use std::{io, fs, env};

#[macro_use] use rust_effects::io_monad;
use std::io::Read;
use std::fs::read_to_string;

fn ask_for_file<'a,
    M: SyncT<'a, X=String, FX=O>,
    O: F<String>
>(ev: M) -> O {
    M::delay(|| {
        println!("Input file name (io_example is a good one):");
        let mut out = String::new();
        io::stdin().read_line(&mut out);
        env::current_dir()
            .map(|mut p| {
                p.push("examples");
                p.push(out.trim());
                p
            })
            .map(|p| format!("{}", p.display()))
            .unwrap_or(format!(""))
    })
}

fn open_and_read<'a,
    S: Monad<'a, X=String, FX=I, Y=Result<String, String>, FY=O>,
    T: SyncT<'a, X=Result<String, String>, FX=O, Y=Result<String, String>, FY=O>,
    I: F<String>,
    O: F<Result<String, String>>
>(ev: S, evt: T, input: I) -> O {
    S::flat_map(input, move |file| T::delay(move || {
        fs::read_to_string(file.clone())
            .map_err(|e| format!("error reading file: {:?}", e))
    }))
}

fn printout<'a,
    S: Monad<'a, X=Result<String, String>, FX=I, Y=(), FY=O>,
    T: SyncT<'a, X=(), FX=O>,
    I: F<Result<String, String>>,
    O: F<()>
>(ev: S, evt: T, input: I) -> O {
    S::flat_map(input, |contents| T::delay(move || {
        print!("Output: {:?}", contents);
    }))
}

fn ask_for_file_inferred<'a,
    O: F<String> + SyncTEffect<'a, X=String>
>() -> O {
    delay(|| {
        println!("Input file name (io_example is a good one):");
        let mut out = String::new();
        io::stdin().read_line(&mut out);
        env::current_dir()
            .map(|mut p| {
                p.push("examples");
                p.push(out.trim());
                p
            })
            .map(|p| format!("{}", p.display()))
            .unwrap_or(format!(""))
    })
}

fn open_and_read_inferred<'a,
    I: F<String> + MonadEffect<'a, String, Result<String, String>, FX=I, FY=O>,
    O: F<Result<String, String>> + SyncTEffect<'a, X=Result<String, String>>
>(input: I) -> O {
    flat_map(input, move |file| delay(move || {
        fs::read_to_string(file.clone())
            .map_err(|e| format!("error reading file: {:?}", e))
    }))
}

fn printout_inferred<'a,
    I: F<Result<String, String>> + MonadEffect<'a, Result<String, String>, (), FX=I, FY=O>,
    O: F<()> + SyncTEffect<'a, X=()>
>(input: I) -> O {
    flat_map(input, |contents| delay(move || {
        print!("Output: {:?}", contents);
    }))
}

fn main() {
    let m: IO<String> = ask_for_file(io_monad!());
    let m: IO<Result<String, String>> = open_and_read(io_monad!(), io_monad!(), m);
    let m: IO<()> = printout(io_monad!(), io_monad!(), m);

    let m2: IO<String> = ask_for_file_inferred();
    let m2: IO<Result<String, String>> = open_and_read_inferred(m2);
    let m2: IO<()> = printout_inferred(m2);

    println!("Now we have set up the IO, proceeding with the example...");
    m.run_sync();
    println!("Now example with inferred functions...");
    m2.run_sync();
}
