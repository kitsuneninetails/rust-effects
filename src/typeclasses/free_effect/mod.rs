pub mod effect_list;
pub mod free;
pub mod free_bind;
pub mod free_map;
pub mod identity;

use crate::typeclasses::monad::Monad;

pub trait FreeEffect {
    /// Output type to the effect's input monad
    type InU: Send;
    /// Output type to the effect's output monad
    type OutU: Send;
    /// Input monad for the effect
    type In: Monad<Self::InU>;
    /// Output monad for the effect
    type Out: Monad<Self::OutU>;
    /// Effect's Conversion function from input to output monad
    fn fold(self, source: Self::In) -> Self::Out;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use free::Free;

    #[test]
    fn test_vec_free_monad() {
        let only_evens = |a: usize| {
            if a % 2 == 0 {
                pure![Vec](a)
            } else {
                Vec::empty()
            }
        };

        let startv = vec!["fox", "horse", "ox", "crow", "mouse", "donkey"];
        let std_step1 = fmap(startv, str::to_string);
        let std_step2 = fmap(std_step1, |a| a.len());
        let standard_out = bind(std_step2, only_evens);

        let free = Free::<Vec<_>>::new(vec!["fox", "horse", "ox", "crow", "mouse", "donkey"]);
        let free = free.map(str::to_string);
        let free = free.map(|a: String| a.len());
        let free = free.bind(only_evens);
        let out = free.fold_map();

        println!("Standard out = {:?}", standard_out);
        println!("Free out = {:?}", out);
        assert_eq!(standard_out, out);
        // Nothing has been done until the wrap-up call:
        // let standard_out: Vec<usize> = freec.fold_map();
    }
}
