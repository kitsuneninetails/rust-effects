# Rust Effects for Functional Programming

Rust is a terrific language for close-to-the-metal programming, with memory 
tracking, borrow mechanics, and easy threading.  It also has a very healthy 
suite of functional programming utilities, such as `map` and `fold` (commonly 
known as map-reduce) for most iterable data structures, but also `Option`/`Result` 
for error and null handling and `map` and `and_then` for manipulating data 
in a context (such as `Option`, `Result`, `Future`, etc.) and short-circuit 
chaining repeated operations which may fail or return null values.  This 
combined with by-default immutable variables, Scala-style type assignments, 
and functions as parameters make Rust a fairly strong language for functional 
programming.

One big part of FP Rust lacks is higher-kinded types.  These are included in 
Haskell and available in Scala (the `cats` library for Scala makes extensive 
use of HKTs), but an implementation in Rust would necessarily be quite complex, 
due to the staticly-typed nature of Rust (every type instantiated must be 
statically known at compile time), which is an important requirement of the 
memory-tracking system and the borrow-checker.  

Without Higher-Kinded Types, can we implement a full Monadic typeclass system 
in Rust?

As this library shows, yes!  Well, mostly, and hopefully sufficiently.

This crate and the functions provided are largely based on the `cats` and 
`cats-effects` libraries for Scala, hence the name `rust-effects`.

A big shoutout to https://github.com/antonsmetanin for all the help in establishing
some base approaches as well as advice and aid in implementation in various modules!

## A Short Intro to Higher-Kinded Types

In Haskell parlance, there is a distinction between "value", "type", and "kind."  

A "value" is any concrete data value like `2`, `3.14`, `test_string`, `Color::Blue`, 
etc.  

A "type" is basically a description for a set of acceptable values.  An "Integer" 
type allows (..., -1, 0, 1, 2, ...) while "String" allows for a list of any 
alpha-numeric bytes (or a UTF-8 encoded list of bytes, in Rust's case, or even 
others), and so on.  Types can also take other types, rather than take values.  
For instance, a `List` takes another type to make a concrete type (which will then 
describe which values are acceptable).

A "kind" describes the meta-type system and how the types interact with each other.  
Kinds are stated using `*` syntax.  A type which can take a value to become 
"concrete" (i.e. instantiable as a specific, known member of the set of possible 
values in the type), is labeled as `*`.  A *List* of *Integers* is still `*` 
because it needs no further information to be ready to take a value.

For example:
```text
    +------------------------+                                    ++===========++
    | Integer                |                                    || Integer   ||
    |    ..., -1, 0, 1, ...? |  => Give a concrete value "2"  =>  ||     2     || 
    +------------------------+                                    ++===========++
      Kind = *
```

Kinds which take a type to generate another type are known as "type constructors."  
There are many common type constructors in most modern languages.  Lists, vectors, maps, 
options, results (Either in Haskell and Scala), are all type constructors.  These are 
specified as `* -> *`.  This means that this type constructor takes one type to generate 
a type ready to take a value.  A Scala *List* is `* -> *`, if we give it *Integer* (itself 
a `*`), we get *List[Integer]*, which has a kind of `*`.  If a type takes more than 
one type to become concrete, that is represented with `* -> * -> *` (and so on).  

A Rust `Result` has a kind of `* -> * -> *`.  It takes one type (*u32*) to make a 
*Result<u32, E>*, which has a kind of `* -> *`.  It takes another type (String) to make a 
concrete *Result<u32, String>* with a kind `*` and ready to take *Ok(2)*, which is a 
concrete value.  Chaining type together still doesn't affect this syntax.  An 
*Option<List&lt;X>>* is still `* -> *`, because it still needs a concrete type to make
 another concrete type (giving it a *u32* makes a concrete *Option<List&lt;u32>>*). In 
 the end, Option still just needs one concrete type (represented by `*`, such as 
 *List&lt;u32>*) to make it concrete.


For example:
```text
    +---------------------+                               +---------------------+
    | Option              |                               | Option              |
    |         +--------+  |  => Specify Option[Int] =>    |         +--------+  | 
    | Some?   | Type T |  |              (kind = *)       | Some?   | Int  ? |  |
    | None?   +--------+  |                               | None?   +--------+  |
    +---------------------+                               +---------------------+
      Kind = * -> *                                           Kind = *     |
                                                                           |
                                             Can now give a concrete value | "Some(2)"
                                                                           V
                                                          ++=====================++
                                                          || Option = Some       ||
                                                          ||        ++========++ ||
                                                          ||        || Int(2) || ||
                                                          ||        ++========++ ||
                                                          ++=====================++
```

The next level of abstraction up takes us to a type which takes a type constructor to 
form a type constructor (much like a type constructor takes a concrete type to form another 
concrete type).  These are called "higher-kinded types" or "higher-order type operators", 
formally.  Rust does not implement higher-kinded types, however Scala can define them with 
the [\_] generic syntax.  Defining a Foo[F[\_]] is to state that this type "Foo" must take 
a type constructor (which itself takes a disregarded concrete type), and that the exact 
type constructor used isn't too important as long as it can fill the shape required (i.e. 
trait bounds, if any).

Trying to instantiate `Foo[Int]` won't work, because `Int` isn't a type constructor.

Higher-kinded types have a "kind" of `(* -> *) -> *`.  The `(* -> *)` part indicates the 
first type constructor which, when provided, will collapse the kind to a `* -> *`, which has 
already been seen above.


For example:
```text
    +----------------------------+                        +----------------------------+
    | Foo                        |                        | Foo                        |
    |    +--------------------+  |                        |    +--------------------+  |
    |    | Type Constructor   |  |                        |    | Option             |  | 
    |    |    ?               |  | => Supply Option[_] => |    |                    |  |
    |    |        +--------+  |  |        (kind = * -> *) |    |  Some? +--------+  |  |
    |    |        | Type T |  |  |                        |    |  None? | Type T |  |  |  
    |    |        +--------+  |  |                        |    |        +--------+  |  |         
    |    +--------------------+  |                        |    +--------------------+  | 
    +----------------------------+                        +----------------------------+ 
      Kind = (* -> *) -> *                                    Kind = * -> *     |
                                                                                |
                                         Supply Option[Int] for the type        |
                                                 (kind = *)                     V
                                                             +----------------------------+
                                                             | Foo                        |
                                                             |   +---------------------+  |
                                                             |   | Option              |  |
                                                   Kind = *  |   |         +--------+  |  |
                                                             |   | Some?   | Int  ? |  |  |
                                                             |   | None?   +--------+  |  |
                                                             |   +---------------------+  |
                                                             +----------------------------+
                                                                           |
                                             Can now give a concrete value | "Some(2)"
                                                                           V
                                                       ++==============================++
                                                       || Foo                          ||
                                                       ||   ++=====================++  ||
                                                       ||   || Option = Some       ||  ||
                                                       ||   ||        ++========++ ||  ||
                                                       ||   ||        || Int(2) || ||  ||
                                                       ||   ||        ++========++ ||  ||
                                                       ||   ++=====================++  ||
                                                       ++==============================++
```


This brings us to type classes.  Typeclasses in their basic sense are the same as traits 
in Scala or Rust. They merely define a set of behaviors for a type that implements them.  
In Rust, using traits is the only way we can implement some of the typeclasses for 
higher-kinded types.

Since Rust does not support higher-kinded types, this means we cannot enforce the idea 
of a generic type which must take a type constructor for its type parameter at a compiler 
level.  This must be enforced at an implementation level.  For `Functor`, for example, a 
type constructor is needed because the whole idea of a `Functor` is to manipulate and 
transform the type inside a context (i.e. the concrete type a type constructor is declared 
with) in a general way.  So it makes no sense for an Integer to also be a Functor, because 
it has no internal type to map to a different type (the shapes don't fit in the diagram 
above).

## CFuture
Futures in Rust aren't actually a concrete structure, but rather a large variety of 
structures that implement a trait: `Future`.  This makes things difficult when it comes 
to typeclass implementations as we cannot implement mapping functions that take or
return trait objects.  This means we need a concrete Future to hold an inner future
object, and although we can use an existing one, they all have a context and purpose 
associated, making it tacky at best to force one into the role.

Hence, the CFuture (Concrete Future):

```
pub struct CFuture<'a, A: Clone + Send + Sync> {
    inner: Shared<BoxFuture<'a, A>>,
}
```

The `Shared` allows this to be cloneable, which is necessary when mapping to another 
future and sending to a new thread (hence the inner type must also be cloneable and 
sendable).

Creating a CFuture is easy.  It can be created from an existing future:

```
CFuture::new_fut(async { call_my_async_function().await })
```

or even created directly from the contained data:
```
CFuture::lazy(3)
```

Note that this creats a lazy Future, not immediate, meaning it must be *await*ed
to return the value.  Anything in either constructor will not be evaluated until 
then.

Once created, the CFuture also implements the `Future` trait, meaning it can be 
*await*ed itself:

```
let fut = CFuture::lazy(3);
assert_eq!(fut.await, 3);
```

or mapped (with `FutureExt`):
```
use futures_util::FutureExt;
let fut = CFuture::lazy(3);
let fut = fut.then(|i| i + 5);
assert_eq!(fut.await, 8);
```

CFuture implements all of the following type classes, making it useful for mimicking 
Higher-Kinded Types as well as for performing Monadic operations asynchronously
(as Rust Futures are lazy, not greedy).

## Rust-effects
The `rust-effects` crate contains the `typeclass` definitions as well as implementations for various common data structures. `Future` in particular is 
represented by the concrete `CFuture` structure,which allows static 
storage and manipulation of the `Future` allowed by the trait itself.

The `typeclasses` defined for these contexts are:

```text

        +---------+             +-----------+
        | Functor |             | Semigroup |
        +---------+             +-----------+
             ^                        ^
             |                        |
      +-------------+             +--------+
      | Applicative |             | Monoid |
      +-------------+             +--------+
             ^
             |------------------------+
             |                        |
   +--------------------+         +-------+
   | ApplicativeFunctor |         | Monad |
   +--------------------+         +-------+    
```

### Semigroup
Define `combine` and `combine_m` functions which can combine two instances of any 
`Semigroup` implementation into a third of the same type.  The `combine` function will
use additive combinations while `combine_m` will apply multiplicative combination. 

***Function***

Each trait derivation implements these functions, but there is also a global helper function 
which can be used (Rust type inference can usually figure out the generic type parameters):

```
fn combine<T: Semigroup>(a: T, b: T) -> T
fn combine_m<T: Semigroup>(a: T, b: T) -> T
```

***Implementations***

* All numeric types - Combine the two parameters with addition (combine) or multiplication
  (combine_m).
* Unit (`()`) - Returns empty tuple `()` for any combination. The combine_m function 
  redirects to combine.
* `String` - Returns the concatenation of the two parameter strings. The combine_m 
  function redirects to combine.
* `Option<T>` - If both options are Some(T), combine the inner T data as per T's combine 
  implementation (and combine_m respectively). If one of the options being combined are None,
  the other Option's T value will be returned.  None is returned when both Options being 
  combined are None.
* `Result<T, E>` - Same as Option, only with Ok(T) and Err(E).  If both parameeters
  are Err, then the error values will be combined in the returning Err.
* `Vec<T>` - Combining two vectors will return a result with the second operand being 
  appended to the first.  The combine_m function redirects to combine.
* `CFuture<T>` - Combining two Futures will result in their eventual values being *await*ed
  and then combined (meaning the contained type T must also implement Semigroup).

### Monoid

Monoids implement the `empty` function, which establishes an identity value for that type.
Using `combine` with the identity value generated by `empty` will result in the other
operand being returned unchanged (similar to adding 0 to a value).  Likewise, using
`combine_m` with the multiplicative identity value generatede by `empty_m` will have
the same effect.

***Function***

Each trait derivation implements these functions, but there is also a global helper function 
which can be used (Rust type inference can usually figure out the generic type parameters):

```
fn empty<T: Monoid>() -> T
fn empty_m<T: Monoid>() -> T
```

***Implementations***

* All numeric types - Returns 0 for empty and 1 for empty_m.
* Unit (`()`) - Returns () as the identity value for both empty and empty_m.
* `String` - Returns the empty string for both empty and empty_m. 
* `Option<T>` - Returns None for both empty and empty_m.
* `Result<T, E>` - Returns Err(E::empty()) for empty and Err(E::empty_m()) for empty_m.
* `Vec<T>` - Returns an empty vector for both empty and empty_m.
* `CFuture<T>` - Returns a lazy future that evaluates to T::empty() for empty and 
  T::empty_m() for empty_m.

### Functor

Functors are operators which provide contextual mapping from one mathematical category 
to another.  A category in mathematics is a set of  "objects" (such as types, sets, 
ranges, shapes, etc.) and the "arrows" between them (mappings from one type to another 
would be an example), as well as how these mappings compose (like going from String to 
Int back to String can also be represented by a composed function that goes from String 
to String).  

In software programming, there is only one category to consider: the category 
of all types and type transitions a programming language supports.  In a programming 
language, the types (objects) are defined as are the methods for mapping one type to 
another (the arrows) and the composition of these mappings, meaning this is the only 
category to consider.  This makes all functors in a software development sense into 
"endofunctors" (functors which map one category, the category specified by the 
programing language's grammar, to itself).  

In practical terms, a `Functor` has an `fmap` function to map from one type to another.
The `fmap` function takes the source container and a mapping function.  This mapping 
function takes a type T and returns a U.  The source's Functor implementation decides how 
(or even whether) the meapping is applied.

One key aspect of the Functor's fmap is that it will not alter the state of the source
object.  It will only conditionally apply the mapping to the interior, contained data 
and transform it if applied (as opposed to Monad, which can change the object's state).

***Function***

Each trait derivation implements these functions, but there is also a global helper function 
which can be used (Rust type inference can usually figure out the generic type parameters):

```
fmap<'a, T, U, A: Functor<'a, T, U>>(a: A, func: impl Fn(T) -> U + Send + 'a) -> A::F
```

>*Note: Type A::F is defined by the specific Functor implementation.  A::F is the output
>Functor type and is defined as Functor\<U> (where the Functor wrapper type is the same
>as the source's).*

***Implementations***

* `Option<T>` - Applies the mapping function T -> U when the source Option is Some(T) 
  and returns Some(U), otherwise fmap returns None.
* `Result<T, E>` - Applies the mapping function T -> U when the source Result is Ok(T) and 
  returns Ok(U), otherwise fmap returns the untouched Err(E). 
* `Vec<T>` - Apples the mapping function T -> U on each element of the vector, returning
  Vec[U]. If the vector is empty, fmap returns the empty vector (also typed as Vec\<U>).
* `CFuture<T>`- Applies the mapping function T -> U to the contained value of the future, 
  but only when *await*ed.  Since the Future is lazy, no mapping is appleid until then.

Note that some Functor implementations (such as Future and Vec), the mapping function is 
alwais applied to each element, but in others, it is conditionally applied depending on
the Functor object's state.

### Applicative

Applicatives provide a `pure` function which returns a new Applicative instance given some
contained value.  This allows any usage of the Applicative to construct a new Applicative 
type very generally, allowing the same function to construct and even return different
applicative implementations with the same implementation.

The `pure` state is in contrast to the `empty` state from `Monoid`, in that pure states 
are usually the ones where the Functor and Monadic function mappings are applied, while
empty states are skipped (exceptions exist, however, such as CFuture).

***Function***

Each trait derivation implements these functions, but there is also a global helper function 
which can be used (Rust type inference can usually figure out the generic type parameters):

```
fn pure<'a, A: Applicative<'a, T>, T>(t: T) -> A
```

***Implementations***

* `Option<T>` - Creates Some(T) from the provided parameter.
* `Result<T, E>` - Creates Ok(T) from the provided parameter.
* `Vec<T>` - Creates a Vec<T> with one element set to the provided parameter.
* `CFuture<T>` - Creates a new CFuture<T> with a lazy future set to return the provided 
  parameter.

### Applicative Functor

Applicative functors build on the applicative and functor concept by introducing the <*>
operation (called "seq" and labeled as such, as Rust doesn't have user-createable infix
operators).  The `seq` (or sequence) function acts very similar to the `fmap` function
from Functor.  A source object is provided, along with a mapping function that converts 
T -> U.  The difference is this mapping function is itself contained within a context
(same as the source object's context), which makes the application of the function 
not only conditional on the source context's state, but also on the state of the function's
context as well.

In short, Functor `fmap` sill always apply if the source context state allows it, but 
Applicative Functor's `seq` applies only if the source and function's context state
allows it.  This also allows the function context to be "curryable," which makes
possible the application of a mapping function which takes two parameters:

```
// Have to have a curryable function for the example
fn add(a: u32) -> impl Fn(u32) -> u32 {
    move |b| a + b
}

// Using Option for example
let add3 = seq(Some(3u32), Some(add)); // Returns Some(impl Fn(u32) -> u32) = Some(|b| 3 + b)
let res = seq(Some(4), add3); // Since add3 = Some(fn), this will apply |b| 3 + b to 4
assert_eq!(res, Some(7));
assert_eq!(seq(Some(4), seq(Some(3), Some(add))), Some(7)); // Compact
```

This is not possible with Functor without a very un-FP-like unwrap():

```
// Have to have a curryable function for the example
fn add(a: u32) -> impl Fn(u32) -> u32 {
    move |b| a + b
}

// Using Option for example
let add3 = fmap(Some(3u32), add); // Returns Some(impl Fn(u32) -> u32) = Some(|b| 3 + b)
let res = fmap(Some(4), add3.unwrap()); // Won't compile without .unwrap()
assert_eq!(res, Some(7));
assert_eq!(fmap(Some(4), fmap(Some(3), add).unwrap()), Some(7)); // Compact
```

***Function***

Each trait derivation implements these functions, but there is also a global helper function 
which can be used (Rust type inference can usually figure out the generic type parameters):

```
fn seq<'a, N, M, T, U>(m: N, func: N::AFunc) -> N::AOut
where
    N: ApplicativeFunctor<'a, M, T, U>,
    M: Fn(T) -> U,
```
>*Note: Type N::AFunc and N::AOut are defined by the specific ApplicativeFunctor implementation.
>N::AFunc is the wrapper of the function parameter: Functor<impl Fn(T) -> U.  N::AOut
>is the wrapper of the output of seq as ApplicativeFunctor\<U>.  In implementations, these
>are always defined to be the same type constructor as the implementation (for example, Option's
>implementation sets N::AFunc to Option<M> ande N::AOut to Option\<U>).*

***Implementations***

* `Option<T>` - Same as `fmap` if function is wrapped in Some().  If the function parameter
  is None, return None.
* `Result<T, E>` - Same as `fmap` if function is wrapped in Ok().  If the function parameter
  is Err(e), the return will be the source parameter's Err value or the function's Err value
  if the source is Ok().  Note that the Error type E must be the same for both source and the
  function parameter.
* `Vec<T>` - Apply each function in the supplied vector for the function parameter to each 
  value in the source parameter (in that order), so `seq([in1, in2, in3], [f1, f2, f3])` will
  return: `[f1(in1), f1(in2), f1(in3), f2(in1), f2(in2), f2(in3), f3(in1), f3(in2), f3(in3)]`. 
* `CFuture<T>` - Call *await* on the function parameter and the source parameter, then apply
  the function and wrap the async block future in a new CFuture.  The function future will 
  not be applied until the returned CFuture is itself *await*ed.

### Monad

Monads are one of the key points behind functional programming and also present some of
the biggest learning curves to newcomers.  However, this doesn't have to be the case.
Monads are very simple in their concept and actually not complicated in implementation.
The difficulty in explaining them lies in the fact that most people try to give their
explanations grounded in mathematics.  While it is true that mathematical category
theory underpins the ideas present in functional programming, explanations using
mathematical concepts must invariably be obtuse, generalized, and almost completely
non-understandable by anyone without a lot of previous study in the topic.

Fortunately, software development narrows the focus of these mathematical concepts
to a point where we don't have to worry about the definitions and explanations for
generalized category theory, but only those which are relevant for programming.  Much
like how all the categories in mathematics reduce to a single category for any given
programming language, we can reduce the concept of a Monad similarly.

Structurally, a Monad is a "computational context" or "computatilnal container."  All
Monads must necessarily hold data of a single type, although the data may be multiply
instanced (like a List or Vector as opposed to an Option or Future).  The Monad must
also define a `bind` function (also called `flat_map` or sometimes `and_then` in Rust).
This function signature is `T -> M\<U>` (as opposed to fmap's mapping function, which 
is `T -> U`).  `M` in the `bind` function return is the Monad type, and must be the same
type as the Monad implementing the `bind` function.

The `bind` function is clearly similar to the `fmap` function:

```
fmap(source: M, func: Fn(T) -> U) -> M<U>
bind(source: M, func: Fn(T) -> M<U>) -> M<U>
```

In fact, if we set `U` in `fmap` to be `M\<U>` and used it instead, we'd get:
```
fmap(source: M, func: Fn(T) -> M<U>) -> M<M<U>>
```

The return would be a double-wrapped Monad object (such as `Option<Option<String>>` or somesuch).
So, we could get the single `M<U>` out of this by running a `flatten` function (which flattens 
out redundant containers or lists into a single copntainer/list).  This concept of `map` then
`flatten` is what gives `flat_map` its name.  However, note that this implementation calls this
same functionality as `bind` rather than `flat_map` (although both would be recognized as doing
the same thing).  The function `bind` is called such is not for the mechanics of how it would be
implemented, but rather due to the concept being presented.

In essence, a `bind` operation is attaching (or binding) the data inside the Monad to a 
mapping function which then returns the same Monad (although the state can be different, 
like an Option going from Some to None).  Because the Monad is returned, it too can have
its data bound again to another function, and so on.  This forms a "binding chain" which
starts from some input and continues on a number of functions, each with the ability to
return its own state of the Monad to reflect the Monad's contextual properties.

And this gets into this idea of context, which is what sets the Monad in functional 
programming aside from the Functor.  While each have their roots in mathametical category
theory, the practical reason a Monad exists in functional programming is due to this 
concept of context. The context of a Monad essentially governs the binding chain and 
how data flows from input to final result.  The context is related to how data is 
input, returned, and passed along the chain. 

Here are the objects which implement Monad and their attached contexts: 

* `Option` - Data can be null.  Any binding function should not act on null data and
  should pass on the null data as is.
* `Result` - Data can represent an error condition (with error information).  Any binding
  function should only act on data from successful conditions and should pass on the error
  condition and its information as is.
* `Vec` - Data can represent non-determinate or multiple states at once.  Any binding 
  function should act on all data individually, returning its own set of possibly non-
  determinate results.  Any empty result (empty vector) should be removed from the data
  set entirely and not be acted upon.
* `Future` - Data may not be available yet, but can be awaited to ensure the data's
  existence.  Any binding function should be suspended until the data is available, 
  and its own result is considered to likewise be only available at some point in the
  future.  The final resdult will only be available on demand, when the entire chain 
  is *await*ed.

These contexts govern the entire binding process (which can also be seen as "binding"
data to a particular context of computation/chaining).

A Monad also offers a "lift" function which take an ordinary `T -> U` function
and convert it to a `M<T> -> M<U>` function.  This brings a normal, pure function into
the Monad's context, allowing it act according to the Monad's rules.  Often, multiple
"lifting" functions are provided for multiple parmeters (like `S, T -> U` being lifted
as `M<S>, M<T> -> M<U>`).

An easy example of lifting would be to take an "add" function which takes two numeric 
parameters.  We could lift this into an Option's context, where the data may be null:

```
fn add(a: u32, b: u32) -> u32 { a + b }
let opt_add = lift_m2::<Option<_>, _, _, _, _>(add);
assert_eq!(opt_add(Some(4), Some(3)), Some(7));
assert_eq!(opt_add(None, Some(3)), None);
```

Note the difference to the simnple `combine` function, as a null parameter nullifies the
result, as is in line with the Optional Monadic context of binding operations not operating 
on null inputs.

***Functions***

Each trait derivation implements these functions, but there are also a global helper functions 
which can be used (Rust type inference can usually figure out the generic type parameters):

```
pub fn bind<'a, T: Send + 'a, U: Send + 'a, M: Monad<'a, T, U>>(
    m: M,
    func: impl Fn(T) -> M::M + Send + 'a,
) -> M::M;
```
>*Note: M::M is the output Monad type and is defined by the Monad implementation as Monad\<U>*

```
pub fn lift_m1<'a, In, S, T>(func: impl Fn(S) -> T + Send + Clone + 'a) 
  -> impl Fn(In) -> In::M
where
    In: Monad<'a, S, T>,
    S: Send + 'a,

pub fn lift_m2<'a, In1, In2, S2, S1, T>(
    func: impl Fn(S1, S2) -> T + Send + Clone + 'a,
) 
  -> impl Fn(In1, In2) -> In1::M
where
    In1: Monad<'a, S1, T> + Send + Clone + 'a,
    In2: Monad<'a, S2, T, M = In1::M> + Send + Clone + 'a,
    S2: Send + Clone + 'a,
    S1: Send + Clone + 'a,

```
>*Note: The In::M (or In1::M, In2::M) is defined by Inpout Monads to be set to the 
>type of the lift's output (the contained type corresponds to type T).  In the case 
>of `lift_m2`, In1 and In2 must be the same Monad, therefore the M type will be 
>identical.*

The lift functions are very verbose to use from the trait, so the general functions are
recommended.  Even though there are a lot of type parameters, most can be set as `_` as the
type inference can figure them out from the provided function:

```
lift_m2::<MonadType<_>, _, _, _, _>(func);
//        ^^^^^^^^^  Put Option, Result, etc. here
```
***Implementation***

* `Option<T>` - Run the binding function only if Option is Some(T). Lift a pure function
  into the context of Option (only run the function if the value is non-null). 
* `Result<T, E>` - Run the bindng function only if Result is Ok(T).  Pass on Err(E) through
  the binding chain as a short-circuit.  Lift a pure function into the context of Result
  (only run the function if the value is success/Ok condition).
* `Vec<T>` - Run the binding function on each item in the Vec's iterator and flatten the
  returning vector into the result.  Empty vectors amount to a no-op.  Lift a pure function
  into the context of Vec (run the function individually on all provided input data).
* `CFuture<T>` - Map the binding function onto the Future, creating a chained Future to be
  passed on (and chained, and so on until finally *await*ed).  Lift a pure function into the
  context of Future (Run the function in a suspended state, only performing the function 
  on the input value(s) when *await*ed)

  ## Examples


