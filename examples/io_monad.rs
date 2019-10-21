use rust_effects::prelude::*;
use std::{io, fs};

#[macro_use] use rust_effects::io;
use std::io::Read;

fn main() {
    let t = io! (
        {
            println!("Input file name (io_example is a good one):");
            let mut out = String::new();
            io::stdin().read_to_string(&mut out);
            out
        });

    let t = flat_map(t, move |file| io!({
        fs::read_to_string(file)
    }));

    let t = flat_map(t, |contents| io!({
        print!("{:?}", contents);
    }));

    t.run_sync();
}
