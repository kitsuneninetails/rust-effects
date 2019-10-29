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
The `Applicative` trait requires `Functor` and defines the function:

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
function defined in the `Monad` trait (which requires `Applicative`):

```
fn flat_map(fx: FX, func: Fn(X) -> FY) -> FY
```

Rust often defines this as an `and_then` function, which makes sense for certain contexts, like Option and 
Result, because the idea is "do this function which returns a Result *and then* do this next function which 
also returns a Result." 

This library, for the sake of consistency, just uses "flat_map" for every context, even though thinking of
it as an "and then" might be an easy way to picture why this is useful.

### Foldable

The `fold` function is one of the most commonly used data transformation functions in functional 
programming.  It is equivalent to the "reduce" in "map/reduce".  Basically, a collection is reduced to a 
single value by starting from some initial value and then each value being combined ("folded") one-by-one
into the initial value (called an "accumulator" as its job is to "accumulate" the results and pass a
new "initial" state on to the next combination).  At the end, a single value is returned based on the 
combinations of each item.

The easiest "fold" to picture is a `sum`:
```
[1, 2, 3, 4, 5] => fold, starting with 0, add items => 0 + 1 => 1 + 2 => 3 + 4 => 7 + 5 => 12
```

This trait is declared as `Foldable`, requires `Monad`, and defines the following function:
```
fn fold(fx: FX, init: Y, func: Fn(Y, X) -> Y) -> Z
```

The `Z` parameter, as in the `combine` function for `Semigroup` is different from `Y` solely because
of static typing in Rust, but they should be functionally equivalent types.  Having them separate just
allows for different `Future` implementations, or `&str` to be passed in, but a `String` to be returned.

The sum above can be easily implemented as (`Foldable` is implemented on `VecEffect`):
```
let sum = VecEffect::fold(vec![1, 2, 3, 4, 5], 0, |y, x| y + x);
```

### Product

Products aren't a standard typeclass, but it was easier to separate it out in Rust into its own trait, 
as implementations tend to be very different.  Basically, a Productable defines a typeclass which can take
two contexts and combine them into a third, where the result is a cross-product of the two inputs.  For
vectors, this returns a vector of N vectors (N = number of elements in the first input), each of which
has M (number of elements in the second input) 2-tuple elements: 

```
[ [ (X1, Y1), (X1, Y2), ..., (X1, YN) ], [ (X2, Y1), ..., (X2, YN) ], ..., [ (XN, Y1),, ..., (XN, YN) ] ]
```

Other contexts may define the "cross-product" differently.  They all override the function in the trait
`Productable`, which also requires `Monad`:

```
fn product(x: FX, y: FY) -> FXY
```

`FXY` is defined as deriving the `F<(X, Y)>` trait, and should be the same context as the two inputs.

### Traverse

A Traverse typeclass describes the ability to take a traversable collection of items and turn it into
a single context holding the traversable.  It is particularly useful when the traversable
collection is filled with items that can each then be transformed into a context holding some final 
results via a provided function.  

The best example to picture this typeclass is with a vector or list of some items and a function which 
turns those items into Futures or Results (like a login processor, making a variety of network/DB calls, 
any of which can fail, etc.).  Running this function on the traversable as a mapping, will just give a collection
of Results, but then the user has to go through and check each item and get the final result.

Instead, it would be more useful to just have a single Result/Future with a collection of the final answers.
Now, the user can just check or await once and then have the answers ready in a collection.  Traverse provides
this functionality in the standalone `Traverse` trait:

```
fn traverse(fa: FX, func: impl Fn(X) -> E) -> FR
```

The new types: `E` and `FR` are defines as:

`E` - The interim context returned by the function. For example, a series of calls to a DB might all
return Result<String, String>, so `E` is `Result<String, String>`

`FR` - The same type as `E`, but now containing the collection type (`F`), so if the original `FX`
in the above example was a Vec<X>, `FR` would be `Result<Vec<String>, String>`.

Implementations should traverse the given collection `fa` and create an interim collection of objects
returned from the function (so a collection of `E`s).  Then these should be `fold`ed with a `combine`
function to come out with a single result, which is of the `E` type, but now holding the collection
instead.  This is why the `E` type must implement the `Semigroup` and `Applicative` trait, because
it needs to create a context from a raw value and then combine that context with others as the given 
collection is folded up.

Since `Traverse` really only makes sense for "traverse-able" types, it is only generally implemented 
for collections (such as `Vec`). 

## Type Inference

Some of the definitions for functions have quite a few type parameters, meaning any call for these 
would nominally have to declare the type parameters for each call.  Fortunately, type inference
saves the headache in most cases, as long as it is clear an unambiguous which types are to be used
for a particular call.  For a compelling case, we can look at an example.

Given an implementation for Monad:
```
impl<'a, X> Monad<'a> for XyzMonad<X> {
  XyzMonad::flat_map(x: Xyz, func: Fn(X) -> Xyz) -> Xyz {
    ...
  }
}
```

We can define some global functions which can call this function for any Monad, as long as the system
can figure out which one is being targeted:

```
flat_map<'a, T: Monad<'a>, FX: F<X>, X, FY: F<Y>, Y>(monad: T, x: FX, func: Fn(X) -> FY) -> FY {
    T::flat_map(x, func)
}
let fy: Xyz<String> = flat_map(XyzMonad, Xyz::new(3), |x| Xyz::new(format!("{}", x))

flat_map<'a, T: Monad<'a>, FX: F<X>, X, FY: F<Y>, Y>(x: FX, func: Fn(X) -> FY) -> FY {
    T::flat_map(x, func)
}
let fy = flat_map::<XyzMonad<u32>, 
                    Xyz<u32>,  
                    u32,  
                    Xyz<String>,  
                    String>(Xyz::new(3), |x| Xyz::new(format!("{}", x))
```

Both of these are fairly clunky, although the first is pretty close to the Scala+Cats method (although Rust
has no implicits, so the evidence must be specified directly).  The latter, though, is essentially unusable 
when every type parameter must be declared. 

Instead, we can define a trait `MonadEffect` which, when implemented for a type, means that the type has a 
default Monad associated with it, which should be used in any Monad operation.  Most types have only a 
single Monad defined for them, so this definition can be fairly comprehensive.

This is done by implementing the Effect and specifying which structure should be used to represent the
effect for a given type.  For example, here is `MonadEffect` being defined for `Option<X>`:

```
    impl<'a, X, Y> MonadEffect<'a, X, Y> for Option<X> {
        type FX = Option<X>;
        type FY = Option<Y>;
        type Fct = OptionEffect<X, Y, ()>;
    }
```

This is providing a structure (`OptionEffect`) which will take an `Option<X>` and turn it into an
`Option<Y>` via a `flat_map` function.

This allows the following function definition and call to work:

```
fn flat_map<'a, FX, X, FY: F<Y>, Y)(fx: FX, func: Fn(X) -> FY) -> FY 
    where
        FX: F<X> + MonadEffect<'a, X, Y, FX=FX, FY=FY>
{
  FX::Fct::flat_map(fx, func)
}
let fy: Xyz<String> = flat_map(fx, format("{}", x));
```
 
This format requires that the type be declared on the receiving variable (if the type isn't put into
a variable, then the return type/parameter type is generally enough for the type inference to work).
The rest can be unambiguously calculated from the statement.  The type of `fx` will be known, thus fixing
the `FX` and `X` types.  The `FY` and `Y` are known from the variable declaration.  Since `FX` is known
`Fx::Fct` is also known, since `FX` must be a `MonadEffect`, thus allowing it to call the correct `flat_map`.

This is available for all typeclasses and is so generalized the following macros are implemented to
provide *Effect trait implementations for types:

* `semigroup_effect!(?, BaseType, EffectType)`
* `monoid_effect!(?, BaseType, EffectType)`
* `applicative_effect!(?, BaseType, EffectType)`
* `functor_effect!(?, BaseType, EffectType)`
* `functor2_effect!(?, BaseType, EffectType)`
* `monad_effect!(?, BaseType, EffectType)`
* `foldable_effect!(?, BaseType, EffectType)`
* `productable_effect!(?, BaseType, EffectType)`

Due to the way the implementation has to be differentiated for multiple type parameters, lifetimes, etc, the 
`?` above supports several options to provide a slightly different implementation depending on the needs
of the types:

* "1" -> Implement for a BaseType with a single, non-lifetimed type paremters (Option and Vec, for example)
* "1C" -> Same as "1", except the "output" type (`Y` in the above documentation) will derive from `Clone`.
    * This is only available for `functor2_effect!` and `foldable_effect!`. 
* "S" -> Implement for a BaseType with one parameter with a `\`a` lifetime (Future and Io for example).
* "2" -> Implement for a BaseType that takes two parameters (Result, for example)

New ones must be provided if further type differentiations are needed.

## Implementing a new type

The ebst way to illustrate implementing typeclasses for a new type is by example.  Let's define a new type
called "Pair" which takes two values of the same type (so, only one type parameter):

```rust
   struct Pair<X> { a: X, b: X }
   impl<X> Pair<X> {
       pub fn new(a: X, b: X) -> Self { Pair { a, b } }
   }
```

Now, we must provide some basic implementations which all typeclasses require:

```rust
   use rust_effects::prelude::*;
   use std::fmt::Debug;
   impl<X> F<X> for Pair<X> {}
```

Next, let's implement the Functor typeclass to provide for a mapping of types.  This is done by creating
an empty struct to serve as the "effect object":

```rust   
    struct PairFunctor<X, Y, Z> {
         _a: PhantomData<X>,
         _b: PhantomData<Y>,
         _c: PhantomData<Z>
    }
    impl<X, Y, Z> PairFunctor<X, Y, Z> {
        pub fn apply() -> Self {
            PairFunctor {
                _a: PhantomData,
                _b: PhantomData,
                _c: PhantomData
            }
        }
    }
```

So, why the extra type parameters when all we need is `X`?  Furthermore, `X` is only used on the `Pair`
itself, so why does the "effect object" need it at all?  The answer will be made clear when we start
implementing the different typeclasses, so let's start with a simple `Functor`, the base of the
context-based typeclasses.  We just need to implement `Functor` on this struct (see the below section for 
why we do this rather than implementing on `Pair` directly):

```rust
    impl<X, Y, Z> Effect for PairFunctor<X, Y, Z> {}
    impl<'a, X, Y> Functor<'a> for PairFunctor {
        type FX = Pair<X>;
        type FY = Pair<Y>;
        fn fmap(f: Self::FX, func: impl 'a + Fn(X) -> Y + Send + Sync) -> Self::FY {
            Pair::new(func(f.a), func(f.b))
        }
    }
```

First, the `Effect` trait must also be implemented for these "effect objects" so the system can check 
and uphold certain laws regarding implementation as necessary.

Now we can see the reason for the `Y` type parameter.  It is needed by the implementation for the `FY` type
declaration.  Rust currently has a limitation in its generic types where the type parameters must all 
be used in the type being implemented, or in one of the predicates ("where" clause, if present).  This
is an intentional restriction in Rust, but the fact that this restriction doesn't extend to the associated
types (`FX` and `FY` above) makes our job more difficult.  This means that using `X` and `Y` in the 
associated types `FX` and `FY` won't be sufficient, because we don't have them appear in the implementation
target or prediate (we don't have a predicate, and we have no relationships to specify which would allow
us to limit based on `X` and `Y` in a predicate anyway).

So, in order to sidestep this restriction, we have to provide phantom type parameters, which we can then use 
to restrict our `FX` and `FY` types.  `Z` will be used in the `Functor2` trait implementation, so we provide
it now, rather than add it in later.  There are only a maximum of three base types currently used by any 
typeclass currently (in the aforementioned `Functor2`), so `X`, `Y`, and `Z` should be sufficient.

These type parameters allow the `PairEffect` class to be instantiated on any X, Y, and Z types, separately (i.e.
they don't have to be the same type).

Providing an `apply` function for `PairEffect` which sets the `PhatomData` markers is helpful, but not 
required.  Providing a macro to easily create a new `PairEffect` will come in handy later.

```rust
#[macro_export]
macro_rules! pair_functor {
    () => (VecEffect::apply(()))
}
```  

You will find these under other implementations as `xyz_monad!()` because the effect objects being returned
implement all of the typeclasses up to Monad (also including Foldable and Productable), so the effect object
returned will serve as a Monad when needed (in addition to anything Monad is based on, such as Functor and
Applicative).

Last thing is to implement the `FunctorEffect`, so the type inference can be used for the global functions.
We can do this easily via the macro:

```
functor_effect! { 1, Pair, PairEffect }
```

The "1" is stating that we only require a single type parameter for our `Pair` type, and that we don't
use a lifetime.

If you wish to implement this manually without the macro, just implement `FunctorEffect` for `Pair`:

```
    impl<'a, X, Y> FunctorEffect<'a, X, Y> for Pair<X> {
        type FX = Pair<X>;
        type FY = Pair<Y>;
        type Fct = PairEffect<X, Y, ()>;
    }
```

So, now we have a new type and its associated effect object which implements our Functor.  Let's see how 
we would use it.  Given some function with generic types which require a Functor (one using an evidence object, 
and the other using type inference through a `FunctorEffect`):

```rust
    fn usage<'a, T: Functor<'a, X, String>, X: Debug>(_ev: T, input: T::FX) -> T::FY {
        T::fmap(input, |x| format!("{:?}", x))
    }
    fn usage_inferred<'a, X, Y, FX: F<X>, FY: F<Y>>(_ev: T, input: T::FX) -> T::FY 
        where
            FX: F<X> + FunctorEffect<'a, X, Y, FX=FX, FY=FY> 
    {
        T::fmap(input, |x| format!("{:?}", x))
    }
```

We'd use it in a function like this:
```rust
   fn main() {
        // Use fmap straight, using type inference
        let out: Pair<String> = fmap(Pair::new(2, 2), |x| format!("{:?}", x));

        // Use our defined function which requires an "evidence" object.
        let out: Pair<String> = usage(pair_functor!(), Pair::new(1, 1));

        // Use our defined function which can infer based on the FunctorEffect
        let out: Pair<String> = usage_inferred(Pair::new(2, 2), |x| format!("{:?}", x));
   }
```

## Type Operator vs Direct Implementation

One of the main questions in implementation of any of these typeclasses would be: "why implement a separate
effect object when you can just implement `Functor` on `Pair` itself?"

The answer is a bit complex and has a lot more to do with structure and style rather than functionality.

In Scala, with Cats, implementing `Monad`, for example, on an object like `Either` or `Future` means that 
when writing a function which takes a `Monad`, a trait bound must be used:

```scala
def foo[X: Monad](x: X): Y = ??
``` 

This will actually lead to many problems in more complex situations, where the program will just not compile.
Furthermore, in the definitive typeclass definition in Haskell, a Monad has the "kind": `(\* -> \*) -> \*`
which means a type which takes a "`(\* -> \*)`" and returns a type of the kind `\* -> \*`.  This returned
type just needs a concrete type (like Int or String) to form a concrete, usable type.

That first step is saying that we need a type that accepts a type constructor.  The input to a Monad should itself
be a type which accepts a concrete type.  Option, Vector (Rust), List (Scala), Result (Rust), Either (Scala), all
fit this shape.  So, we can picture Monad's definition to be something like (in pseudo-code):

```text
trait TypeConstructor {
    def apply[X: ConcreteType](t: X) -> ConcreteType 
}

trait Monad { 
    def apply[X: TypeConstructor](tc: X) -> TypeConstructor
}

impl TypeConstructor for Option[_] {
    def apply[X: ConcreteType](x: X) -> Option[X]
}

impl Monad for ??? {
    def apply[X: TypeConstructor](x: X) -> ???[X]

//use Monad
let x = ???::apply(Option); // x is a type constructor 
let y: x = x::apply(Integer); // give x a concrete type and y is a concrete type
let z: y = y::Ok(2);

```

What's the `???`?  It's something that can take an Option (of any concrete type, to be determined later) and return
a type which can then take that concrete type and fill it in to make a usable type. 

## Why is this Useful?
