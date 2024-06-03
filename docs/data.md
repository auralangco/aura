# Aura: Data

The way Aura treats data and memory depends on the target, but doesn't affect the result of the computation.

## Targets

### C

When targeting C there are three possibilities:

- Data is used only once, so it's moved
- Data is shared by reference when passed as arguments to calls
- Data is copied when passed as argument to data constructors

### JS

WIP

### HVM

WIP

## Immutability

The important thing to notice is: data is always immutable. So everytime you pass data to somewhere else, the original data is preserved.

## Primitive Data Types

### Numbers

- `Int8`, `Int16`, `Int`/`Int32`, `Int64`: Signed integers
- `UInt8`, `UInt16`, `UInt`/`UInt32`, `UInt64`: Unsigned integers
- `Float`/`Float32`, `Float64`: floating point numbers

### Text

- `Char`: utf-8 char
- `String`: sequence of utf-8 chars

### Bool

- `Bool`

### Atom

- `Atom` `:[a-z]([a-z0-9]|-)*`: it is its own value

### Void

- `Void` `:null`

## Collection Types

### `Array(T)`

- A sequence of data of type T

### `List(T)`

- A linked list of type T

### `Object(T)`

- A sequence of data of type `T` indexed by `Atom`

### `Map(#eq, T)`

- A sequence of data of type `T` indexed by `#eq`

## Algebraic Data Types

### Enum: Sum type

Each variant is named after an Atom and may have associated data

```elixir
( :variant-1 T, :variant-2 U, :variant-3 V )
```

A `match` can be used to safely handle each variant

```
link (println) = aura/io

type Foo = (:foo Int, :bar Bool, :foobar)

fn foo {
    f = Foo:bar(:false);

    str = match f with
        :foo f => f $> #printable,
        :bar b => b $> #printable,
        :foobar => "null" $> #printable;
    
    println(str);
}
```

### Struct: Product type

Each field is named after an Atom and must have a type

```elixir
( field_1 T, field_2 U, field_3 V )
```

Each field can be accessed by the `.` operator

```
type Foo = (foo Int, bar Bool)

pure Foo::invert(foo Foo) -> Foo {
    (-foo.foo, !foo.bar)
}
```

Also `Foo.bar` is a function like `pure (foo Foo) -> Bool { foo.bar }`

### Compound: Product type

Similar to struct but the fields are named after natural number literals

```elixir
(T, U, V, W)
```

## Monads: computations wrapped by types

### `Nullable(T)`

Brings null-safety to Aura

- `:some T `: a non-null value
- `:null`: represents an empty value

### `Failable(T)`

Makes failures recoverable

- `:succ T`: the value that represents success
- `:fail #failure`: the value that represents a failure (a `#failure`)

### `Action(T)`

Wraps the result of `act` functions to encapsulate side effects

### `Async(T)`

Produces a handler to deal with potentially long running calls

```
handle Async(String) = Async::new(fn () -> udp::recv("0.0.0.0:4000"));
// Do your stuff
res Result(String) = Async::await(handle);
```

## Functional Types

### Closures

Closures are a way to write functions as values, non-pure functions may capture their environment

```
fn (args...) -> expr
pure (args...) -> expr
act (args...) -> expr
```

### Functions

Functional types receives an input and produces an output

```
fn (arg1 T, arg2 U) -> V
```

### Pure Functions

Functions that doesn't produce any side-effects and the only data it captures is static data. The function type consists of two parts (the input and output) divided by a `->`

```
pure (arg1 T, arg2 U) -> V
```

### Actions

Functions that do produce side-effects (most likely IO operations)

```
act (arg1 T, arg2 U) -> V
```
