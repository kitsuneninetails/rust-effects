use std::marker::PhantomData;

use crate::{
    prelude::Functor,
    typeclasses::{applicative::Applicative, monad::Monad},
};

pub trait FreeEffect {
    /// Input type to the effect's input monad
    type InT: Send;
    /// Output type to yhe effect's input monad
    type InU: Send;
    /// Input type of the output monad
    type OutT: Send;
    /// Output type of the output monad
    type OutU: Send;
    /// Input monad for the effect
    type In: Monad<Self::InT, Self::InU>;
    /// Output monad for the effect
    type Out: Monad<Self::OutT, Self::Out::MonadOut, Self::MonadOutT>;
    /// Effect's Conversion function from input to output monad
    fn fold(self, source: Self::In) -> Self::Out;
}

pub struct Free<T, U, M, Eff>
where
    Eff: FreeEffect,
    T: Send,
    U: Send,
    M: Monad<T, U, U>,
{
    start_monad: M,
    start_effect: Eff,
    _ph: PhantomData<T>,
    _ph2: PhantomData<U>,
}

impl<T, U, M, Eff> Free<T, U, M, Eff>
where
    Eff: FreeEffect,
    T: Send,
    U: Send,
    M: Monad<T, U>,
    V: Send,
    W: Send,
    N: Monad<V, W>,
{
    pub fn new(start_monad: M) -> Free<T, U, M, Identity<T, U, M>> {
        Free {
            start_monad,
            start_effect: Identity::new(),
            _ph: PhantomData,
            _ph2: PhantomData,
        }
    }
    pub fn fold_map(self) -> Eff::Out {
        todo!()
    }

    pub fn add<NewEff>(self, effect: NewEff) -> Free<T, U, M, EffectList<NewEff, Eff>>
    where
        NewEff: FreeEffect<In = Eff::Out>,
    {
        Free {
            start_monad: self.start_monad,
            start_effect: EffectList::from_pair(effect, self.start_effect),
            _ph: PhantomData,
            _ph2: PhantomData,
        }
    }

    pub fn map<V, NewMonad>(
        self,
        func: impl Fn(Eff::InU) -> V + Send + 'static,
    ) -> Free<T, U, M, EffectList<FreeMap<Eff::InU, V, NewMonad>, Eff>>
    where
        V: Send + 'static,
        NewMonad: Monad<Eff::InU, V> + Send,
    {
        let new_map = FreeMap::new(func);

        self.add(new_map)
    }
    pub fn bind<B, C>(
        self,
        func: impl Fn(Eff::InU) -> C::MonadOut + Send + 'static,
    ) -> Free<T, U, M, EffectList<FreeBind<B, C, C, Eff::Out>, Eff>>
    where
        B: Send + 'static,
        C: Monad<Eff::Out, B> + Send + 'static,
    {
        let new_map = FreeBind::new(func);
        self.add(new_map)
    }
}

// impl<T, U, M> Functor<T, U> for Suspended<T, U, M>
// where
//     M: Monad<T, U> + Send,
//     T: Send,
//     U: Send,
// {
//     type F = Suspended<U, (), M::M>;
//     fn fmap(m: Self, func: impl Fn(T) -> U + Send + 'a) -> Self::F {
//         m.map(func)
//     }
// }

// impl<T, U, M> Applicative<T, U> for Suspended<T, U, M>
// where
//     M: Monad<T, U> + Send,
//     T: Send,
//     U: Send,
// {
//     fn pure(t: T) -> Self {
//         Self::pure(t)
//     }
// }

// impl<T, U, M> Monad<T, U> for Suspended<T, U, M>
// where
//     M: Monad<T, U> + Send,
//     T: Send,
//     U: Send,
// {
//     type M = Suspended<T, U, M>;
//     fn bind(m: Self, func: impl Fn(T) -> Self::M + Send + 'a) -> Self::M {
//         m.flat_map(func)
//     }
// }

struct Identity<T, U, M>
where
    T: Send,
    U: Send,
    M: Monad<T, U, U>,
{
    _ph0: PhantomData<T>,
    _ph1: PhantomData<U>,
    _ph2: PhantomData<M>,
}

impl<T, U, M> Identity<T, U, M>
where
    T: Send,
    U: Send,
    M: Monad<T, U, U>,
{
    pub fn new() -> Self {
        Identity {
            _ph0: PhantomData,
            _ph1: PhantomData,
            _ph2: PhantomData,
        }
    }
}

impl<T, U, M> FreeEffect for Identity<T, U, M>
where
    T: Send,
    U: Send,
    M: Monad<T, U, U>,
{
    type InT = T;
    type InU = U;
    type MonadOutT = U;
    type In = M;
    type Out = M::MonadOut;
    fn fold(self, _source: Self::In) -> Self::Out {
        todo!()
    }
}

pub struct FreeMap<T, U, V, In>
where
    T: Send,
    U: Send,
    V: Send,
    In: Monad<T, U, V> + Send,
{
    func: Box<dyn Fn(T) -> U + Send>,
    _ph: PhantomData<In>,
    _ph2: PhantomData<V>,
}

impl<T, U, V, In> FreeMap<T, U, V, In>
where
    T: Send,
    U: Send,
    V: Send,
    In: Monad<T, U, V> + Send,
{
    pub fn new(func: impl Fn(T) -> U + Send + 'static) -> Self {
        FreeMap {
            func: Box::new(func),
            _ph: PhantomData,
            _ph2: PhantomData,
        }
    }
}

impl<T, U, V, In> FreeEffect for FreeMap<T, U, V, In>
where
    T: Send + 'static,
    U: Send + 'static,
    V: Send + 'static,
    In: Monad<T, U, V> + Send,
{
    type InT = T;
    type InU = U;
    type MonadOutT = V;
    type In = In;
    type Out = In::MonadOut;
    fn fold(self, source: Self::In) -> Self::Out {
        Self::In::fmap(source, self.func)
    }
}

pub struct FreeBind<T, U, V, In>
where
    In: Monad<T, U, V> + Send,
    T: Send,
    U: Send,
    V: Send,
{
    func: Box<dyn Fn(T) -> In::MonadOut + Send>,
}

impl<T, U, V, In> FreeBind<T, U, V, In>
where
    In: Monad<T, U, V> + Send,
    T: Send,
    U: Send,
    V: Send,
{
    pub fn new(func: impl Fn(T) -> In::MonadOut + Send + 'static) -> FreeBind<T, U, V, In> {
        FreeBind {
            func: Box::new(func),
        }
    }
}

impl<T, U, V, In> FreeEffect for FreeBind<T, U, V, In>
where
    In: Monad<T, U, V> + Send + 'static,
    T: Send + 'static,
    U: Send + 'static,
    V: Send + 'static,
{
    type InT = T;
    type InU = U;
    type MonadOutT = V;
    type In = In;
    type Out = In::MonadOut;
    fn fold(self, source: Self::In) -> Self::Out {
        Self::In::bind(source, self.func)
    }
}

pub struct EffectList<CurrEff, NestEff>
where
    CurrEff: FreeEffect,
    NestEff: FreeEffect,
{
    curr_effect: CurrEff,
    next_effect: NestEff,
}

impl<CurrEff, NestEff> EffectList<CurrEff, NestEff>
where
    <CurrEff as FreeEffect>::Out: Monad<
            <CurrEff as FreeEffect>::MonadOutT,
            <CurrEff as FreeEffect>::MonadOutT,
            <CurrEff as FreeEffect>::MonadOutT,
        >,
    CurrEff: FreeEffect,
    NestEff: FreeEffect<
            InT = CurrEff::InU,
            InU = CurrEff::MonadOutT,
            MonadOutT = CurrEff::MonadOutT,
            Out = CurrEff::In,
        >,
{
    pub fn from_pair(effect2: CurrEff, effect1: NestEff) -> EffectList<CurrEff, NestEff> {
        EffectList {
            curr_effect: effect2,
            next_effect: effect1,
        }
    }
    pub fn add<NewF: FreeEffect<In = CurrEff::Out>>(
        self,
        effect: NewF,
    ) -> EffectList<NewF, EffectList<CurrEff, NestEff>> {
        EffectList {
            curr_effect: effect,
            next_effect: self,
        }
    }
}

impl<CurrEff, NestEff> FreeEffect for EffectList<CurrEff, NestEff>
where
    <CurrEff as FreeEffect>::Out: Monad<
            <CurrEff as FreeEffect>::MonadOutT,
            <CurrEff as FreeEffect>::MonadOutT,
            <CurrEff as FreeEffect>::MonadOutT,
        >,
    CurrEff: FreeEffect,
    NestEff: FreeEffect<
            InT = CurrEff::InU,
            InU = CurrEff::MonadOutT,
            MonadOutT = CurrEff::MonadOutT,
            Out = CurrEff::In,
        >,
{
    type InT = NestEff::InT;
    type InU = NestEff::InU;
    type MonadOutT = CurrEff::MonadOutT;
    type In = NestEff::In;
    type Out = CurrEff::Out;
    fn fold(self, source: Self::In) -> Self::Out {
        let folded_monad = self.next_effect.fold(source);
        let out = self.curr_effect.fold(folded_monad);
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_vec_free_monad() {
        let to_s = lift_m1![Vec](str::to_string);
        let len = lift_m1![Vec](String::len);
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

        let startv2 = vec!["fox", "horse", "ox", "crow", "mouse", "donkey"];
        let free = Free::new(startv2);
        let freea = free.map(str::to_string);
        let freeb = freea.map(|a: String| a.len());
        let freec = freeb.bind(only_evens);

        // Nothing has been done until the wrap-up call:
        let standard_out: Vec<usize> = freec.fold_map();
    }
}
