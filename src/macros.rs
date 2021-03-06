#[macro_export]
macro_rules! monoid_impl {
    ($m:ty, $v:expr, $($t:ty)+) => ($(
        impl Monoid<$t> for $m {
            fn empty() -> $t { $v }
        }
    )+)
}

#[macro_export]
macro_rules! sg_impl {
    ($m:ty, $op:tt, $($t:ty)+) => ($(
        impl Semigroup<$t, $t, $t> for $m {
            fn combine(a: $t, b: $t) -> $t { a $op b }
        }
    )+)
}

#[macro_export]
macro_rules! monoid_eff_impl {
    ($m:ty, $($t:ty)+) => ($(
        impl MonoidEffect<$t> for $t {
            type Fct = $m;
        }
    )+)
}

#[macro_export]
macro_rules! monoid_effect {
    (0, $m:ident, $eff:ident) => (
        impl MonoidEffect<$m> for $m {
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<X> MonoidEffect<$m<X>> for $m<X> {
            type Fct = $eff<X, (), ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X> MonoidEffect<$m<'a, X>> for $m<'a, X>
            where
                X: 'a + Send + Sync + MonoidEffect<X> {
            type Fct = $eff<'a, X, (), ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<X, E: Debug> MonoidEffect<$m<X, E>> for $m<X, E>
            where
                X: MonoidEffect<X> {
            type Fct = $eff<E, X, (), ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, X, E> MonoidEffect<$m<'a, X, E>> for $m<'a, X, E>
            where
                X: 'a + Send + Sync + MonoidEffect<X> ,
                E: 'a + Send + Sync + Debug {
            type Fct = $eff<'a, E, X, (), ()>;
        }
    );
}

#[macro_export]
macro_rules! sg_eff_impl {
    ($m:ty, $($t:ty)+) => ($(
        impl SemigroupEffect<$t, $t, $t> for $t {
            type Fct = $m;
        }
    )+)
}

#[macro_export]
macro_rules! semigroup_effect {
    (0, $m:ident, $eff:ident) => (
        impl SemigroupEffect<$m, $m, $m> for $m {
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<X, X2, XR> SemigroupEffect<$m<X>, $m<X2>, $m<XR>> for $m<X>
            where
                X: SemigroupEffect<X, X2, XR> + SemigroupEffect<X, XR, XR>,
                X2: SemigroupEffect<X2, XR, XR>,
                XR: MonoidEffect<XR> {
            type Fct = $eff<X, X2, XR>;
        }
    );
    (1A, $m:ident, $eff:ident) => (
        impl<'a, X> SemigroupEffect<$m<X>, $m<X>, $m<X>> for $m<X> {
            type Fct = $eff<X, X, X>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X, X2, XR> SemigroupEffect<$m<'a, X>, $m<'a, X2>, $m<'a, XR>> for $m<'a, X>
            where
                X: 'a + SemigroupEffect<X, X2, XR>  + SemigroupEffect<X, XR, XR> + Send + Sync,
                X2: 'a + SemigroupEffect<X2, XR, XR> + Send + Sync,
                XR: 'a + MonoidEffect<XR> + Send + Sync {
            type Fct = $eff<'a, X, X2, XR>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, E: Debug, X, X2, XR> SemigroupEffect<$m<X, E>, $m<X2, E>, $m<XR, E>> for $m<X, E>
            where
                X: SemigroupEffect<X, X2, XR> + SemigroupEffect<X, XR, XR>,
                X2: SemigroupEffect<X2, XR, XR>,
                XR: MonoidEffect<XR> {
            type Fct = $eff<E, X, X2, XR>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, E, X, X2, XR> SemigroupEffect<$m<'a, X, E>, $m<'a, X2, E>, $m<'a, XR, E>> for $m<'a, X, E>
            where
                X: 'a + SemigroupEffect<X, X2, XR>  + SemigroupEffect<X, XR, XR> + Send + Sync,
                X2: 'a + SemigroupEffect<X2, XR, XR> + Send + Sync,
                XR: 'a + MonoidEffect<XR> + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type Fct = $eff<'a, E, X, X2, XR>;
        }
    )
}

#[macro_export]
macro_rules! applicative_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> ApplicativeEffect<'a> for $m {
            type X = ();
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X> ApplicativeEffect<'a> for $m<X> {
            type X = X;
            type Fct = $eff<X, (), ()>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X: Clone> ApplicativeEffect<'a> for $m<X> {
            type X = X;
            type Fct = $eff<X, (), ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X> ApplicativeEffect<'a> for $m<'a, X>
            where
                X: 'a + Send + Sync {
            type X = X;
            type Fct = $eff<'a, X, (), ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X, E: Debug> ApplicativeEffect<'a> for $m<X, E> {
            type X = X;
            type Fct = $eff<E, X, (), ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, X, E> ApplicativeEffect<'a> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type X = X;
            type Fct = $eff<'a, E, X, (), ()>;
        }
    );
}

#[macro_export]
macro_rules! applicativeapply_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a, M> ApplicativeApplyEffect<'a, M, (), ()> for $m
            where
                M: 'a + Fn(()) -> () + Send + Sync {
            type FM = $m;
            type FX = $m;
            type FY = $m;
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, M, X, Y> ApplicativeApplyEffect<'a, M, X, Y> for $m<X>
            where
                M: 'a + Fn(X) -> Y + Send + Sync {
            type FM = $m<M>;
            type FX = $m<X>;
            type FY = $m<Y>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, M, X, Y> ApplicativeApplyEffect<'a, M, X, Y> for $m<X>
            where
                X: Clone,
                M: 'a + Fn(X) -> Y + Send + Sync {
            type FM = $m<M>;
            type FX = $m<X>;
            type FY = $m<Y>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, M, X, Y> ApplicativeApplyEffect<'a, M, X, Y> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                M: 'a + Fn(X) -> Y + Send + Sync {
            type FM = $m<'a, M>;
            type FX = $m<'a, X>;
            type FY = $m<'a, Y>;
            type Fct = $eff<'a, X, Y, ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, M, X, Y, E: Debug> ApplicativeApplyEffect<'a, M, X, Y> for $m<X, E>
            where
                M: 'a + Fn(X) -> Y + Send + Sync {
            type FM = $m<M, E>;
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, M, E, X, Y> ApplicativeApplyEffect<'a, M, X, Y> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug,
                M: 'a + Fn(X) -> Y + Send + Sync {
            type FM = $m<'a, M, E>;
            type FX = $m<'a, X, E>;
            type FY = $m<'a, Y, E>;
            type Fct = $eff<'a, E, X, Y, ()>;
        }
    );
}

#[macro_export]
macro_rules! functor_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> FunctorEffect<'a, (), ()> for $m {
            type FX = $m;
            type FY = $m;
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y> FunctorEffect<'a, X, Y> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X, Y> FunctorEffect<'a, X, Y> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync {
            type FX = $m<'a, X>;
            type FY = $m<'a, Y>;
            type Fct = $eff<'a, X, Y, ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X, Y, E: Debug> FunctorEffect<'a, X, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, E, X, Y> FunctorEffect<'a, X, Y> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type FX = $m<'a, X, E>;
            type FY = $m<'a, Y, E>;
            type Fct = $eff<'a, E, X, Y, ()>;
        }
    );
}

#[macro_export]
macro_rules! functor2_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> Functor2Effect<'a, (), (), ((), ())> for $m {
            type FX = $m;
            type FY = $m;
            type FZ = $m;
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y, Z> Functor2Effect<'a, X, Y, Z> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type FZ = $m<Z>;
            type Fct = $eff<X, Y, Z>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone, Z> Functor2Effect<'a, X, Y, Z> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type FZ = $m<Z>;
            type Fct = $eff<X, Y, Z>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X, Y, Z> Functor2Effect<'a, X, Y, Z> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                Z: 'a + Send + Sync {
            type FX = $m<'a, X>;
            type FY = $m<'a, Y>;
            type FZ = $m<'a, Z>;
            type Fct = $eff<'a, X, Y, Z>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X, Y, Z, E: Debug> Functor2Effect<'a, X, Y, Z> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type FZ = $m<Z, E>;
            type Fct = $eff<E, X, Y, Z>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, E, X, Y, Z> Functor2Effect<'a, X, Y, Z> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                Z: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type FX = $m<'a, X, E>;
            type FY = $m<'a, Y, E>;
            type FZ = $m<'a, Z, E>;
            type Fct = $eff<'a, E, X, Y, Z>;
        }
    );
}

#[macro_export]
macro_rules! monad_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> MonadEffect<'a, (), ()> for $m {
            type FX = $m;
            type FY = $m;
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y> MonadEffect<'a, X, Y> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> MonadEffect<'a, X, Y> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X, Y> MonadEffect<'a, X, Y> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync {
            type FX = $m<'a, X>;
            type FY = $m<'a, Y>;
            type Fct = $eff<'a, X, Y, ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X, Y, E: Debug> MonadEffect<'a, X, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, E, X, Y> MonadEffect<'a, X, Y> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type FX = $m<'a, X, E>;
            type FY = $m<'a, Y, E>;
            type Fct = $eff<'a, E, X, Y, ()>;
        }
    );
}

#[macro_export]
macro_rules! foldable_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> FoldableEffect<'a, (), (), ()> for $m {
            type FX = $m;
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y> FoldableEffect<'a, X, Y, Y> for $m<X> {
            type FX = $m<X>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> FoldableEffect<'a, X, Y, Y> for $m<X> {
            type FX = $m<X>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> FoldableEffect<'a, X, Y, Y> for $m<X> {
            type FX = $m<X>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X, Y: Clone> FoldableEffect<'a, X, Y, $m<'a, Y>> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync {
            type FX = $m<'a, X>;
            type Fct = $eff<'a, X, Y, ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X, Y: Clone, E: Debug> FoldableEffect<'a, X, Y, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, E, X, Y: Clone> FoldableEffect<'a, X, Y, $m<'a, Y, E>> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type FX = $m<'a, X, E>;
            type Fct = $eff<'a, E, X, Y, ()>;
        }
    );
}

#[macro_export]
macro_rules! monaderror_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> MonadErrorEffect<'a, ()> for $m {
            type X = ();
            type E = ();
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X> MonadErrorEffect<'a, X> for $m<X> {
            type X = X;
            type E = ();
            type Fct = $eff<X, (), ()>;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X: Clone> MonadErrorEffect<'a, X> for $m<X> {
            type X = X;
            type E = ();
            type Fct = $eff<X, (), ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X> MonadErrorEffect<'a, X> for $m<'a, X>
            where
                X: 'a + Send + Sync {
            type X = X;
            type E = ();
            type Fct = $eff<'a, X, (), ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X, E: Debug> MonadErrorEffect<'a, X> for $m<X, E> {
            type X = X;
            type E = E;
            type Fct = $eff<E, X, (), ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, E, X> MonadErrorEffect<'a, X> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type X = X;
            type E = E;
            type Fct = $eff<'a, E, X, (), ()>;
        }
    );
}

#[macro_export]
macro_rules! productable_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> ProductableEffect<'a, (), ()> for $m {
            type FX = $m;
            type FY = $m;
            type FZ = $m;
            type Fct = $eff;
        }
    );
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y> ProductableEffect<'a, X, Y> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type FZ = $m<(X, Y)>;
            type Fct = $eff<X, Y, (X, Y)>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> ProductableEffect<'a, X, Y> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type FZ = $m<(X, Y)>;
            type Fct = $eff<X, Y, (X, Y)>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> ProductableEffect<'a, X, Y> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync {
            type FX = $m<'a, X>;
            type FY = $m<'a, Y>;
            type FZ = $m<'a, (X, Y)>;
            type Fct = $eff<'a, X, Y, (X, Y)>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone, E: Debug> ProductableEffect<'a, X, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type FZ = $m<(X, Y), E>;
            type Fct = $eff<E, X, Y, (X, Y)>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone, E> ProductableEffect<'a, X, Y> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type FX = $m<'a, X, E>;
            type FY = $m<'a, Y, E>;
            type FZ = $m<'a, (X, Y), E>;
            type Fct = $eff<'a, E, X, Y, (X, Y)>;
        }
    );
}

#[macro_export]
macro_rules! synct_effect {
    (0, $m:ident, $eff:ident) => (
        impl<'a> SyncTEffect<'a> for $m {
            type E = ();
            type X = ();
            type Fct = $eff;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X> SyncTEffect<'a> for $m<'a, X>
            where
                X: 'a + Send + Sync {
            type E = ();
            type X = X;
            type Fct = $eff<'a, X, X, ()>;
        }
    );
    (2S, $m:ident, $eff:ident) => (
        impl<'a, X, E> SyncTEffect<'a> for $m<'a, X, E>
            where
                X: 'a + Send + Sync,
                E: 'a + Send + Sync + Debug {
            type E = E;
            type X = X;
            type Fct = $eff<'a, E, X, X, ()>;
        }
    );
}
