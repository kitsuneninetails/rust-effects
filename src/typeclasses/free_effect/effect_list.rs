use crate::typeclasses::free_effect::FreeEffect;

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
    CurrEff: FreeEffect<In = NestEff::Out>,
    NestEff: FreeEffect,
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
    CurrEff: FreeEffect<In = NestEff::Out>,
    NestEff: FreeEffect,
{
    type InU = NestEff::InU;
    type OutU = CurrEff::OutU;
    type In = NestEff::In;
    type Out = CurrEff::Out;
    fn fold(self, source: Self::In) -> Self::Out {
        let folded_monad = self.next_effect.fold(source);
        let out = self.curr_effect.fold(folded_monad);
        out
    }
}
