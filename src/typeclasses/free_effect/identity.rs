use std::marker::PhantomData;

use crate::typeclasses::monad::Monad;

use super::FreeEffect;

pub struct Identity<M, U>
where
    M: Monad<U>,
{
    _ph1: PhantomData<U>,
    _ph2: PhantomData<M>,
}

impl<M, U> Identity<M, U>
where
    U: Send,
    M: Monad<U>,
{
    pub fn new() -> Self {
        Identity {
            _ph1: PhantomData,
            _ph2: PhantomData,
        }
    }
}

impl<M, U> FreeEffect for Identity<M, U>
where
    U: Send,
    M: Monad<U>,
{
    type InU = U;
    type OutU = U;
    type In = M;
    type Out = M;
    fn fold(&self, source: Self::In) -> Self::Out {
        source
    }
}

#[cfg(test)]
mod test {
    use crate::typeclasses::free_effect::{FreeEffect, identity::Identity};

    #[test]
    fn test_identity() {
        let id = Identity::<Option<u32>, u32>::new();
        let input = Some(33u32);
        let out = id.fold(input);
        assert_eq!(input, out);
    }
}
