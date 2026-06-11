use std::marker::PhantomData;

use crate::typeclasses::{
    free_effect::{free_bind::FreeBind, free_map::FreeMap},
    monad::Monad,
};

use super::{FreeEffect, effect_list::EffectList, identity::Identity};

pub struct Free<M, U = (), Eff = Identity<M, U>>
where
    Eff: FreeEffect,
    U: Send,
    M: Monad<U>,
{
    pub start_effect: Eff,
    _ph1: PhantomData<U>,
    _ph2: PhantomData<M>,
}

impl<M, U, Eff> Free<M, U, Eff>
where
    Eff: FreeEffect<In = M>,
    U: Send,
    M: Monad<U>,
{
    pub fn new() -> Free<M, U, Identity<M, U>>
    where
        U: Send,
        M: Monad<U>,
    {
        Free::<M, U> {
            start_effect: Identity::new(),
            _ph1: PhantomData,
            _ph2: PhantomData,
        }
    }
    pub fn new_effect(start_effect: Eff) -> Free<M, U, Eff>
    where
        U: Send,
        M: Monad<U>,
        Eff: FreeEffect,
    {
        Free::<M, U, Eff> {
            start_effect,
            _ph1: PhantomData,
            _ph2: PhantomData,
        }
    }

    pub fn fold_map(&self, start_monad: Eff::In) -> Eff::Out {
        self.start_effect.fold(start_monad)
    }

    pub fn add<NewEff>(self, effect: NewEff) -> Free<M, U, EffectList<NewEff, Eff>>
    where
        NewEff: FreeEffect<In = Eff::Out>,
    {
        Free::<M, U, _> {
            start_effect: EffectList::from_pair(effect, self.start_effect),
            _ph1: PhantomData,
            _ph2: PhantomData,
        }
    }

    pub fn map<V, W>(
        self,
        func: impl Fn(V) -> W + Send + Clone + 'static,
    ) -> Free<M, U, EffectList<FreeMap<V, W, Eff::Out>, Eff>>
    where
        V: Send + 'static,
        W: Send + 'static,
        Eff::Out: Monad<W, MonadT = V> + Send,
    {
        self.add(FreeMap::<V, W, Eff::Out>::new(func))
    }

    pub fn bind<V, W, MOut>(
        self,
        func: impl Fn(V) -> MOut + Send + Clone + 'static,
    ) -> Free<M, U, EffectList<FreeBind<V, W, Eff::Out>, Eff>>
    where
        V: Send + 'static,
        W: Send + 'static,
        Eff::Out: Monad<W, MonadT = V, MonadOut = MOut> + Send + 'static,
        MOut: Monad<MonadT = W> + Send + 'static,
    {
        self.add(FreeBind::<V, W, Eff::Out>::new(func))
    }
}

#[cfg(test)]
mod test {
    use crate::typeclasses::free_effect::{
        effect_list::EffectList, free::Free, free_bind::FreeBind, free_map::FreeMap,
        identity::Identity,
    };

    #[test]
    fn test_new_with_identity() {
        let input = Some(34u32);
        let free = Free::<Option<_>>::new();
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_new_with_effect_identity() {
        let input = Some(34u32);
        let ident = Identity::<Option<_>, u32>::new();
        let free = Free::<Option<_>, u32, _>::new_effect(ident);
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_new_with_effect_list() {
        let input = Some(34u32);
        let list = EffectList::from_pair(
            Identity::<Option<_>, u32>::new(),
            Identity::<Option<_>, u32>::new(),
        );
        let free = Free::<Option<_>, u32, _>::new_effect(list);
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_add_effect_from_new() {
        let input = Some(34u32);
        let free = Free::<Option<_>>::new();
        let new_effect = Identity::<Option<_>, u32>::new();
        let free = free.add(new_effect);
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_add_effect_from_list() {
        let input = Some(34u32);
        let list = EffectList::from_pair(
            Identity::<Option<_>, u32>::new(),
            Identity::<Option<_>, u32>::new(),
        );
        let free = Free::<Option<_>, u32, _>::new_effect(list);

        let new_effect = Identity::<Option<_>, u32>::new();
        let free = free.add(new_effect);
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_new_with_effect_map_same_type() {
        let input = Some(34u32);
        let mapping = FreeMap::<u32, u32, Option<_>>::new(|t| t);
        let free = Free::<Option<_>, u32, _>::new_effect(mapping);
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_new_with_effect_map_change_type() {
        let input = Some("dog".to_string());
        let mapping = FreeMap::<String, u32, Option<_>>::new(|t| t.len() as u32);
        let free = Free::<Option<_>, (), _>::new_effect(mapping);
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some(3))
    }

    #[test]
    fn test_add_map_effect_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<_>, u32>::new();
        let mapping = FreeMap::<String, u32, Option<_>>::new(|t| t.len() as u32);
        let free = free.add(mapping);
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some(3))
    }

    #[test]
    fn test_add_two_map_effects_changing_type() {
        let input = Some("dog".to_string());

        let free = Free::<Option<_>, ()>::new();
        let mapping = FreeMap::<String, u32, Option<_>>::new(|t| t.len() as u32);
        let free = free.add(mapping);
        let mapping2 = FreeMap::<u32, String, Option<_>>::new(|t| t.to_string());
        let free = free.add(mapping2);
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some("3".to_string()))
    }

    #[test]
    fn test_map_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<_>>::new();
        let free = free.map(|t: String| t.len());
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some(3))
    }

    #[test]
    fn test_two_maps_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<String>, ()>::new();
        let free = free.map(|t| t.len());
        let free = free.map(|t| t.to_string());
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some("3".to_string()))
    }

    #[test]
    fn test_new_with_effect_bind() {
        let input = Some(34u32);
        let mapping = FreeBind::<u32, u32, Option<_>>::new(|t| Some(t));
        let free = Free::<Option<_>, u32, _>::new_effect(mapping);
        let out = free.fold_map(input.clone());
        assert_eq!(out, input)
    }

    #[test]
    fn test_add_bind_effect_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<_>, ()>::new();
        let mapping = FreeBind::<String, u32, Option<_>>::new(|t| Some(t.len() as u32));
        let free = free.add(mapping);
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some(3))
    }

    #[test]
    fn test_add_two_bind_effects_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<_>, ()>::new();
        let mapping = FreeBind::<String, u32, Option<_>>::new(|t| Some(t.len() as u32));
        let free = free.add(mapping);
        let mapping2 = FreeBind::<u32, String, Option<_>>::new(|t| Some(t.to_string()));
        let free = free.add(mapping2);
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some("3".to_string()))
    }

    #[test]
    fn test_add_two_bind_effects_empty_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<_>, ()>::new();
        let mapping = FreeBind::<String, u32, Option<_>>::new(|t| Some(t.len() as u32));
        let free = free.add(mapping);
        let mapping2 = FreeBind::<u32, String, Option<_>>::new(|_t| None);
        let free = free.add(mapping2);
        let out = free.fold_map(input.clone());
        assert!(out.is_none())
    }

    #[test]
    fn test_bind_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<String>, ()>::new();
        let free = free.bind(|t| Some(t.len()));
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some(3))
    }

    #[test]
    fn test_two_binds_from_new() {
        let input = Some("dog".to_string());

        let free = Free::<Option<String>, ()>::new();
        let free = free.bind(|t| Some(t.len()));
        let free = free.bind(|t| Some(t.to_string()));
        let out = free.fold_map(input.clone());
        assert_eq!(out, Some("3".to_string()))
    }
}
