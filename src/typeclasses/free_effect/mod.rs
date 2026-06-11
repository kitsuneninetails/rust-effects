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
    fn fold(&self, source: Self::In) -> Self::Out;
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
                empty::<Vec<_>>()
            }
        };

        let startv = vec!["fox", "horse", "ox", "crow", "mouse", "donkey"];
        let std_step1 = fmap(startv, str::to_string);
        let std_step2 = fmap(std_step1, |a| a.len());
        let standard_out = bind(std_step2, only_evens);

        let free = Free::<Vec<_>>::new();
        let free = free.map(str::to_string);
        let free = free.map(|a: String| a.len());
        let free = free.bind(only_evens);
        let out = free.fold_map(vec!["fox", "horse", "ox", "crow", "mouse", "donkey"]);

        println!("Standard out = {:?}", standard_out);
        println!("Free out = {:?}", out);
        assert_eq!(standard_out, out);
        // Nothing has been done until the wrap-up call:
        // let standard_out: Vec<usize> = freec.fold_map();
    }

    fn only_evens<M: Monad<MonadT = u32> + Monoid>(a: M::AppT) -> M {
        if a % 2 == 0 { M::pure(a) } else { M::empty() }
    }

    fn free_function<M, A>() -> Free<M, u32, impl FreeEffect<In = M, Out = A>>
    where
        M: Monad<u32, MonadT = String> + Send + 'static,
        M::MonadT: Send + 'static,
        M::MonadOut: Monad<u32, MonadT = u32, MonadOut = A>,
        A: Monad<MonadT = u32> + Monoid + Send + 'static,
    {
        let free = Free::<M, u32>::new();
        let free = free.map(|a: String| a.len() as u32);
        let free = free.bind(only_evens);
        free
    }

    #[test]
    fn test_free_function() {
        let free = free_function::<Vec<String>, Vec<u32>>();
        let input = vec!["fox", "horse", "ox", "crow", "mouse", "donkey"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        let out = free.fold_map(input);

        println!("Free out = {:?}", out);
        assert_eq!(out, vec![2, 4, 6]);

        let free = free_function::<Option<String>, Option<u32>>();
        let out = free.fold_map(Some("rabbit".to_string()));
        assert_eq!(out, Some(6));
        let out = free.fold_map(Some("fox".to_string()));
        assert_eq!(out, None);
        let out = free.fold_map(None);
        assert_eq!(out, None);
    }
}
