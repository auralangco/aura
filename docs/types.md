# Type System

Aura has a type system that aims to be extensible, flexible and still being safe so invalid data cannot be represented.

## Primitive Data Types

Basic types provided by Aura and cannot be accesed in parts

### Numbers

Represent numeric data

- `I8`, `I16`, `Int`/`I32`, `I64`: Signed integers
- `U8`, `U16`, `UInt`/`U32`, `U64`: Unsigned integers
- `Float`/`F32`, `F64`: floating point numbers

### Chars

Represents characters

- `Char`: utf-8 char

### Atom

- `Atom` `:[a-z]([a-z0-9]|-)*`: it is its own value

### Bool

Booleanic data `:true` or `:false`

- `Bool`

### Void

Represents non-data

- `Void` `:null`

## Collection Types

### `Array(T)`

- A sequence of data of type T

### `String`

- A sequence of `Char`

### `List(T)`

- A linked list of type T

### `Object(T)`

- A sequence of data of type `T` indexed by `Atom`

### `Map(#hash, T)`

- A sequence of data of type `T` indexed by `#hash`

## Algebraic Data Types

### [Enum](./types/enum.md): Sum type

Each variant is named after an Atom and may have associated data

```txt
( :variant-1 T, :variant-2 U, :variant-3 V )
```

A `match` can be used to safely handle each variant

```txt
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

```txt
( field_1 T, field_2 U, field_3 V )
```

Each field can be accessed by the `.` operator

```txt
type Foo = (foo Int, bar Bool)

pure Foo.invert(foo Foo) -> Foo = (-foo.foo, !foo.bar)

```

Also `Foo.bar` is a function like `pure (foo Foo) -> Bool { foo.bar }`

### Compound: Product type

Similar to struct but the fields are named after natural number literals

```txt
(T, U, V, W)
```

## Monads: computations wrapped by types

### `Nullable(T)`

Brings null-safety to Aura

- `:some T`: a non-null value
- `:null`: represents an empty value

### `Failable(T)`

Makes failures recoverable

- `:succ T`: the value that represents success
- `:fail #failure`: the value that represents a failure (a `#failure`)

### `Async(T)`

Produces a handler to deal with potentially long running calls

```txt
handle Async(String) = Async::new(fn () -> udp.recv("0.0.0.0:4000"));
// Do your stuff
res Result(String) = Async::await(handle);
```

## Functional Types

### Closures

Closures are a way to write functions as values, non-pure functions may capture their environment

```txt
fn (args...) -> expr
pure (args...) -> expr
```

### Functions

Functional types receives an input and produces an output

```txt
fn (arg1 T, arg2 U) -> V
```

### Pure Functions

Functions that doesn't produce any side-effects and the only data it captures is static data. The function type consists of two parts (the input and output) divided by a `->`

```txt
pure (arg1 T, arg2 U) -> V
```

## Casting

Castings are convertions between different types. The important part of a cast is that it only exists between types in the same category, so the casting function never fails.

There are automatic castings allowed in Aura. But only from more general types to more specialized if, and only if, the types are structuraly identical.

It means `Int` auto casts to `I32`, also `T` auto casts to `(T)`, but not from `Float` to `Int`.
