extern crate rust_typeclasses;

use rust_typeclasses::typeclasses::*;
use rust_typeclasses::*;
use rust_typeclasses::typeclasses::monad::flat_map;

fn main() {
    let o_ev = &option::OptionEffect;
    let r_ev = &result::ResultEffect;
    let v_ev = &vec::VecEffect;

    let o1 = applicative::pure(o_ev, 10u32);
    let r1 = applicative::pure(r_ev, 10u32);

    flat_map(o_ev, |x| r1.ok())

}
