# Rust Effects for Functional Programming


## Typeclasses

### Monoid

### Semigroup

### Applicative

### Functor

### Monad

### Product

### Traverse



## Implementing a new type

```rust
   use rust_effects::prelude::*;
   use std::fmt::Debug;
   struct Pair<X> { a: X, b: X }
   impl<X> Pair<X> {
       pub fn new(a: X, b: X) -> Self { Pair { a, b } }
   }
   impl<X> F<X> for Pair<X> {}
   impl<'a, X, Y> FunctorEffect<'a, X, Y> for Pair<X> {
      type FX = Pair<X>;
      type FY = Pair<Y>;
      type Fct = Pair<X>;
   }
   
   impl<T> Effect for Pair<T> {}
   impl<'a, X, Y> Functor<'a, X, Y> for Pair<X> {
       type FX = Pair<X>;
       type FY = Pair<Y>;
       fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
           Pair::new(func(f.a), func(f.b))
       }
   }
   
   fn usage<'a, T: Functor<'a, X, String>, X: Debug>(input: T::FX) -> T::FY {
       T::fmap(input, |x| format!("{:?}", x))
   }
   
   fn main() {
       let out = usage::<Pair<u32>, u32>(Pair::new(1, 1));
       let out2: Pair<String> = fmap(Pair::new(2, 2), |x| format!("{:?}", x));
   }
```

or
 
```rust
   use rust_effects::prelude::*;
   use std::fmt::Debug;

   struct Pair<X> { a: X, b: X }
   impl<X> Pair<X> {
       pub fn new(a: X, b: X) -> Self { Pair { a, b } }
   }
   impl<X> F<X> for Pair<X> {}
   impl<'a, X, Y> FunctorEffect<'a, X, Y> for Pair<X> {
      type FX = Pair<X>;
      type FY = Pair<Y>;
      type Fct = PairFunctor;
   }
   
   struct PairFunctor;
   impl Effect for PairFunctor {}
   impl<'a, X, Y> Functor<'a, X, Y> for PairFunctor {
       type FX = Pair<X>;
       type FY = Pair<Y>;
       fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
           Pair::new(func(f.a), func(f.b))
       }
   }
   
   fn usage<'a, T: Functor<'a, X, String>, X: Debug>(input: T::FX) -> T::FY {
       T::fmap(input, |x| format!("{:?}", x))
   }

   fn main() {
       let out = usage::<PairFunctor, u32>(Pair::new(1, 1));
       let out2: Pair<String> = fmap(Pair::new(2, 2), |x| format!("{:?}", x));
   }
```
