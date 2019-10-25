# Rust Effects for Functional Programming

Rust is a terrific language for close-to-the-metal programming, with memory tracking, borrow mechanics,
and easy threading.  It also has a very healthy suite of functional programming utilities, such as `map` and 
`fold` (commonly known as map-reduce) for most iterable data structures, but also `Option`/`Result` for error
and null handling and `map` and `and_then` for manipulating data in a context (such as `Option`, `Result`, 
`Future`, etc.) and short-circuit chaining repeated operations which may fail or return null values.  This
combined with by-default immutable variables, Scala-style type assignments, and functions as parameters
make Rust a fairly strong language for functional programming.

One big part of FP Rust lacks is higher-kinded types.  These are included in Haskell and available in Scala
(the `cats` library for Scala makes extensive use of HKTs), but an implementation in Rust would necessarily be
quite complex, due to the staticly-typed nature of Rust (every type must be statically known upon instantiation),
which is an important requirement of the memory-tracking system and the borrow-checker. 

Without Higher-Kinded Types, can we implement a full Monadic typeclass system in Rust?

As this library shows, yes!  Well, mostly, and hopefully sufficiently.

This crate and the functions provided are largely based on the `cats` and `cats-effects` libraries for Scala,
hence the name `rust-effects`.

## A Short Intro to Higher-Kinded Types

In Haskell parlance, there is a distinction between "value", "type", and "kind."  

A "value" is any concrete data value like `2`, `3.14`, `test_string`, `Color::Blue`, etc.  

A "type" is basically a description for a set of acceptable values.  An "Integer" type allows 
(..., -1, 0, 1, 2, ...) while "String" allows for a list of any alpha-numeric bytes (or a UTF-8 encoded 
list of bytes, in Rust's case, or even others), and so on.  Types can also take other types, rather than take
values.  For instance, a `List` takes another type to make a concrete type (which will then describe which 
values are acceptable).

A "kind" describes the meta-type system and how the types interact with each other.  Kinds are stated using "\*" 
syntax.  A type which can take a value to become "concrete" (i.e. instantiable as a specific, known member of
the set of possible valus in the type), is labeled as "\*".  A List of Integers is still "\*" because it 
needs no further information to be ready to take a value.

Kinds which take a type to generate another type are known as "type constructors."  There are many common
type constructors in most modern languages.  Lists, vectors, maps, options, results (Either in Haskell and 
Scala), are all type constructors.  These are specified as "\* -> \*".  This means that this type constructor 
takes one type to generate a type ready to take a value.  A List is "\* -> \*", if we give it "Integer" (itself
a "\*"), we get List[Integer], which has a kind of "\*".  If a type takes more than one type to become concrete,
that is represented with "\* -> \* -> \*" (and so on).  A Rust `Result` has a kind of "\* -> \* -> \*".  It takes
one type (u32) to make a Result<u32, E>, which has a kind of "\* -> \*".  It takes another type (String) to make
a concrete Result<u32, String> with a kind "\*" and ready to take Ok(2), which is a concrete value.  Chaining
type together still doesn't affect this syntax.  An Option<List<X>> is still "\* -> \*", because it still
needs a concrete type to make another concrete type (giving it a `u32` makes a concrete `Option<List<u32>>`).
In the end, Option still just needs one concrete type (represented by "\*", such as `List<u32>) to make
it concrete.

The next level of abstraction up takes us to a type which takes a type constructor to form a type constructor
(much like a type constructor takes a concrete type to form another concrete type).  These are called
"higher-kinded types" or "higher-order type operators", formally.  Rust does not implement higher-kinded types,
however Scala can define them with the [_] generic syntax.  Defining a Foo[F[_]] is to state that this type 
"Foo" must take a type constructor (which itself takes a disregarded concrete type), and that the exact type
constructor used isn't too important as long as it can fill the shape required (i.e. trait bounds, if any).
Trying to instantiate `Foo[Int]` plum won't work, because `Int` isn't a type constructor.

This brings us to type classes.  Typeclasses in their basic sense are the same as traits in Scala or Rust.
They merely define a set of behaviors for a type that implements them.  In Rust, using traits is the only way
we can implement some of the typeclasses for higher-kinded types.

Since Rust does not support higher-kinded types, this means we cannot enforce the idea of a generic type which
must take a type constructor for its type parameter at a compiler level.  This must be enforced at an 
implementation level.  For `Functor`, for example, a type constructor is needed because the whole idea of a 
`Functor` is to manipulate and transform the type inside a context (i.e. the concrete type a type constructor
is declared with) in a general way.  So it makes no sense for an Integer to also be a Functor, because it has no
internal type to be mapping to a different type.

In the `rust-effects` crate, the `typeclasses` module represents higher-kinded typeclasses used in functional
programming.  They are governed through implementation of a special trait: `F<X>`.  By defining this trait
on a type constructor (where `X` is the concrete type parameter), we can enforce the idea that only types
with this trait implemented can be used in higher-order type operators, like `Functor` and `Monad`.
    
## Higher-Kinded Typeclases in Rust-Effects

The following typeclasses have been implemented.  They generally follow the functionality from the typeclasses
in the `cats` library for Scala, with some differences to make them work effectively in Rust.

### Semigroup

The Semigroup is a category of types which can have values "combined" to form a new value of the same type.
The `Semigroup` trait defines a single function: 

```
fn combine(a: X1, b: X2) -> XR
```` 

which takes two values of type `X` and returns a new value of type `X`.  In the Rust implementation, the
types are actually relaxed to take two values of different types (X1 and X2) and return a third type (XR), 
but this is purely for implementations to allow for functionally equivalent, but statically-different
types.  Since all types in Rust are static at instantiation-time, even a wrapper type won't help
when dealing with two types which are slightly different.  A `ConcreteFuture<AndThen<...>>` and a 
`ConcreteFuture<Map<...>>`, for example, are completely different static types in Rust, even though they 
are both `ConcreteFuture<impl Future>`.  To this end, in order to allow for a combination of any 
`ConcreteFuture<T>` or for a `String` to be combined with a `&str` (or two `&str`s to be combined 
into a `String`), the types are relaxed and the implementor must take care to ensure that the types are 
actually abstractly (if not statically) equivalent.

Semigroup can, and is, defined to allow combination of concrete types, such as `String` and all 
integral/floating-point types.  These concrete types do not implement `Semigroup`.  Rather, there is a struct 
which has `Semigroup` implemented to combine the types (see below for 
[why to implement a type operator](#Type-Operator-vs-Direct-Implementation) instead of a direct implementation).
For numeric types, there are generally two semigroup structs defined, one for additive combination, the other for
multiplicative.

### Monoid

The Monoid typeclass represents types which can be empty.  The `Monoid` trait has a single function: 

```
fn empty() -> X
```
  
Like, Semigroup, this is implemented on structs for concrete types, such as String and numeric types (again,
which have a different monoid to return 0 for additive and another to return 1 for multiplicative sitations).

### Functor

The Functor is the most basic category transformation type class.  The mathematical concept of a Functor
is simply a mapping from one category (i.e. type) to another.  The `Functor` trait defines a single function:

```
fn fmap(fx: FX, func: Fn(X) -> Y) -> FY [where FX: F<X>, FY: F<Y> is enforced on the associated types]
``` 

This is the most basic type class which enforces the idea that its operating element is a type constructor, and
cannot be a concrete type.  This enforcement is handled via the `F<X>` trait, which must be implemented on any
type to be used in any Functor-based typeclass.  This trait can technically be implemented on any type, 
including concrete types, however, the implementor should be aware at that moment that they are committing a 
breach of the laws at their own peril.  Without this implementation, the type cannot be used as the `fx`
parameter in the `fmap` function.

This module also defines a `Functor2` trait, which defines fmap in the case of 2 type constructors (of 
potentially different types) being mapped into a new type constructor:

```
fn fmap2(fxa: FX, fxb: FY: Fn(X, Y) -> Z) -> FZ [where FZ: F<Z> is enforced on the associated types]
```
 
As with all of the typeclasses from here on up, `Functor2` inherits both its parent's types (`X`, `Y`, `FX`, and 
`FY` in the case of `Functor`), but also the constraints (that `FX` must implement `F<X>` for example, same for `FY` 
and `Y`), even though it can define more on top of its parent.

### Applicative

The Applicative typeclass is a Functor where a type constructor can be created from an inner type.
The `Applicative` trait defines the function:

```
fn pure(x: X) -> FX
```

It has the same types and constraints as its `Functor` parent.

This function should be thought of as "greedy", meaning it will consume and perform evaluation on its 
parameters when called, rather than defering its execution.

### Monad

This is the class Monad typeclass of "What is a Monad?" fame.  Monads are actually quite simple.  They are 
essentially just a typeclass which describes a context (i.e. type constructor) which can perform an operation
which returns another context.  This is commonly encountered when calling functions which return nulls or errors.  
Each call will return an Option or Result (in Rust) or equivalent.  Each Option/Result may have different 
valid types, although we might want to chain them together.  A Monad is how we map that chaining.

A Functor might seem ideal, since it is a mapping of categories, but it is insufficient in this case, because
a Functor will just map data inside a context, but in this case, we are getting a new context returned by the
functions we are calling.  This would result in a more-and-more complex nesting of contexts, as a 
`Option<u32>` would map to a `Option<Option<String>>` and so on.

Instead, we just want that `Option<u32>` to be passed to a new function, acted upon, and then an 
`Option<String>` to get spit out, which can then be chained to another function, and so on.  To accomplish this,
we can call`fmap`, using a function that returns a new context (like an `Option`) and then "flatten" the
resulting structure (`Option<Option<T>>` becomes `Option<T>`, for example) because the interim contexts
are ultimately not useful.  This "map + flatten" action is often called "flat_map", which is actually the
function defined in the `Monad` trait:

```
fn flat_map(fx: FX, func: Fn(X) -> FY) -> FY
```

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

## Type Operator vs Direct Implementation
