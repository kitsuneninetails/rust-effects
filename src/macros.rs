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
    ($m:ty, $me:expr, $($t:ty)+) => ($(
        impl MonoidEffect<$t> for $t {
            type Fct = $m;
        }
    )+)
}

#[macro_export]
macro_rules! monoid_effect {
    (1, $m:ident, $eff:ident) => (
        impl<X> MonoidEffect<$m<X>> for $m<X> {
            type Fct = $eff<X, (), ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X> MonoidEffect<$m<'a, X>> for $m<'a, X>
            where
                X: 'a + Send + Sync + Default {
            type Fct = $eff<'a, X, (), ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<X, E: Default> MonoidEffect<$m<X, E>> for $m<X, E> {
            type Fct = $eff<E, X, (), ()>;
        }
    )
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
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, X2, XR> SemigroupEffect<$m<X>, $m<X2>, $m<XR>> for $m<X>
            where
                X: SemigroupEffect<X, X2, XR> {
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
                X: 'a + SemigroupEffect<X, X2, XR> + Send + Sync,
                X2: 'a + Send + Sync,
                XR: 'a + Send + Sync {
            type Fct = $eff<'a, X, X2, XR>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, E, X, X2, XR> SemigroupEffect<$m<X, E>, $m<X2, E>, $m<XR, E>> for $m<X, E>
            where
                X: SemigroupEffect<X, X2, XR> {
            type Fct = $eff<E, X, X2, XR>;
        }
    )
}

#[macro_export]
macro_rules! applicative_effect {
    (1, $m:ident, $eff:ident) => (
        impl<'a, X> ApplicativeEffect<'a> for $m<X> {
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
        impl<'a, X, E> ApplicativeEffect<'a> for $m<X, E> {
            type X = X;
            type Fct = $eff<E, X, (), ()>;
        }
    )
}

#[macro_export]
macro_rules! functor_effect {
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
        impl<'a, X, Y, E> FunctorEffect<'a, X, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    )
}

#[macro_export]
macro_rules! functor2_effect {
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
        impl<'a, X, Y, Z, E> Functor2Effect<'a, X, Y, Z> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type FZ = $m<Z, E>;
            type Fct = $eff<E, X, Y, Z>;
        }
    )
}

#[macro_export]
macro_rules! monad_effect {
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y> MonadEffect<'a, X, Y> for $m<X> {
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
        impl<'a, X, Y, E> MonadEffect<'a, X, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    )
}

#[macro_export]
macro_rules! foldable_effect {
    (1, $m:ident, $eff:ident) => (
        impl<'a, X, Y> FoldableEffect<'a, X, Y, Y> for $m<X> {
            type FX = $m<X>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (1C, $m:ident, $eff:ident) => (
        impl<'a, X, Y: Clone> FoldableEffect<'a, X, Y, Y> for $m<X> {
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
        impl<'a, X, Y: Clone, E> FoldableEffect<'a, X, Y, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    )
}

#[macro_export]
macro_rules! productable_effect {
    (1, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> ProductableEffect<'a, X, Y> for $m<X> {
            type FX = $m<X>;
            type FY = $m<Y>;
            type FXY = $m<(X, Y)>;
            type Fct = $eff<X, Y, ()>;
        }
    );
    (S, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone> ProductableEffect<'a, X, Y> for $m<'a, X>
            where
                X: 'a + Send + Sync,
                Y: 'a + Send + Sync {
            type FX = $m<'a, X>;
            type FY = $m<'a, Y>;
            type FXY = $m<'a, (X, Y)>;
            type Fct = $eff<'a, X, Y, ()>;
        }
    );
    (2, $m:ident, $eff:ident) => (
        impl<'a, X: Clone, Y: Clone, E> ProductableEffect<'a, X, Y> for $m<X, E> {
            type FX = $m<X, E>;
            type FY = $m<Y, E>;
            type FXY = $m<(X, Y), E>;
            type Fct = $eff<E, X, Y, ()>;
        }
    );
}
