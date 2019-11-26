use rust_effects::prelude::*;
use std::{io, fs, env};

use rust_effects::io_monad;
use std::io::Error as IOError;

fn ask_for_file<'a,
    S: SyncT<'a, FX=O, X=String, E=IOError>,
    O: F<S::X>
>(_: S) -> O {
    S::suspend(|| {
        println!("Input file name (io_example is a good one):");
        let mut out = String::new();
        io::stdin().read_line(&mut out)
            .map(|_| S::pure(out))
            .unwrap_or_else(|e| S::raise_error(e))
    })
}

fn open_and_read<'a,
    S: SyncT<'a, X=String, FX=T, Y=String, FY=T, E=IOError>,
    T: F<S::X>
>(_: S, input: T) -> T {
    S::flat_map(input, move |out: String| {
        S::suspend(move || {
            match env::current_dir()
                .map(|mut p| {
                    p.push("examples");
                    p.push(out.trim());
                    p
                })
                .map(|p| format!("{}", p.display()))
                .and_then(|f| {
                    fs::read_to_string(f.clone())
                }) {
                Ok(s) => S::pure(s),
                Err(e) => S::raise_error(e)
            }
        })
    })
}

fn printout<'a,
    M: MonadError<'a, X=String, FX=I, Y=(), FY=O>,
    S: SyncT<'a, X=M::Y, FX=M::FY>,
    I: F<M::X>,
    O: F<M::Y>
>(_: S, _: M, input: I) -> O {
    M::flat_map(input, |contents: String| S::delay(move || {
        println!("Formatted Output: {:?}", contents);
    }))
}

fn ask_for_file_inferred<'a,
    O: Effectful<'a, String, IOError>
>() -> O {
    suspend(|| {
        println!("INFERRED: Input file name (io_example is a good one):");
        let mut out = String::new();
        io::stdin().read_line(&mut out)
            .map(|_| pure(out))
            .unwrap_or_else(|e| raise_error(e))
    })
}

fn open_and_read_inferred<'a,
    I: Effectful<'a, String, IOError, String, O>,
    O: Effectful<'a, String, IOError>
>(input: I) -> O {
    flat_map(input, move |out: String| {
        suspend(move || {
            match env::current_dir()
                .map(|mut p| {
                    p.push("examples");
                    p.push(out.trim());
                    p
                })
                .map(|p| format!("{}", p.display()))
                .and_then(|f| {
                    fs::read_to_string(f.clone())
                }) {
                Ok(s) => pure(s),
                Err(e) => raise_error(e)
            }
        })
    })
}

fn printout_inferred<'a,
    I: Effectful<'a, String, IOError, (), O>,
    O: Effectful<'a, (), IOError>
>(input: I) -> O {
    let f: O = flat_map(input, |contents| {
        let o = delay(move || {
            println!("Output: {:?}", contents);
        });
        o
    });
    f

}

type IoType<'a> = IO<'a, String, IOError>;
type IoUnitType<'a> = IO<'a, (), IOError>;

fn main() {
    let m: IoType = ask_for_file(io_monad!());
    let m: IoType = open_and_read(io_monad!(), m);
    let m: IoType = handle_error(m, |e| {
        println!("Error opening file: {:?}, try one more time", e);
        open_and_read(io_monad!(), ask_for_file(io_monad!()))
    });
    let m: IoUnitType = printout(io_monad!(), io_monad!(), m);

    let m2: IoType = ask_for_file_inferred();
    let m2: IoType = open_and_read_inferred(m2);
    let m2: IoUnitType = printout_inferred(m2);

    println!("Now we have set up the IO, proceeding with the example...");
    let output = attempt(m);

    println!("Example 1 output = {:?}", output);

    println!("Now example with inferred functions...");
    let output = attempt(m2);
    println!("Example 2 output = {:?}", output);

}
