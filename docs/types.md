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

### Identifier

Identifier as a literal. Useful for macros

- `Ident` ``` `type```

### Bool

Booleanic data `true` or `false`

- `Bool`

### Void

Represents non-data

- `Void` `null` `Void.Null`

## Collection Types

### `Array(T)`

- A sequence of data of type T

### `String`

- A sequence of `Char`

### `List(T)`

- A linked list of type T

### `Map(K #(hash, eq); K, V)`

- A sequence of data of type `V` indexed by `K` thats both hasheable and equatable

## Algebraic Data Types

### [Enum](./types/enum.md): Sum type

Each variant is named after an Atom and may have associated data

```txt
enum ( Variant1 T, Variant2 U, Variant3 V )
```

A `match` can be used to safely handle each variant

```txt
link (println) = aura/io

type Foo = enum (Foo Int, Bar Bool, Foobar)

fn foo {
    f = Foo:bar(false);

    str = match (f) {
        Foo.Foo f => f |> #printable:from,
        Foo.Bar b => b |> #printable:from,
        Foo.Foobar => "null" |> #printable:from;
    }
    
    println(str);
}
```

### Struct: Product type

Each field is named after an Atom and must have a type

```txt
struct ( field_1 T, field_2 U, field_3 V )
```

Each field can be accessed by the `.` operator

```txt
type Foo = struct (foo Int, bar Bool)

fn Foo:invert(foo Foo) -> Foo = (-foo.foo, !foo.bar)
```

Also `Foo.bar` is a function like `(foo Foo) -> Bool { foo.bar }`

### Compound: Product type

Similar to struct but the fields are named after natural number literals

```txt
(T, U, V, W)
```

### Unions: Sum type

Similar to enum but the variants are anonymous and are matched using the type in a match

```txt
type Number = union (Int, Float)

fn foo {
    n Number = 10;
    match (n) {
        Int => println("n is an Int"),
        Float => println("n is a Float"),
    }
}
```

## Monads: computations wrapped by types

### `Nullable(T)`

Brings null-safety to Aura

- `Nullable.Some(T)`: a non-null value
- `Nullable.Null`: represents an empty value

### `Failable(S, F)`

Makes failures recoverable

- `Failable.Succ(S)`: the value that represents success
- `Failable.Fail(F)`: the value that represents a failure

### `Async(T)`

Produces a handler to deal with potentially long running calls

```txt
handle Async(String) = Async:new(() -> udp::recv("0.0.0.0:4000"));
// Do your stuff
res Result(String) = Async:await(handle);
```

## Functional Types

### Closures

Closures are a way to write functions as values, non-pure functions may capture their environment

```txt
(args...) -> expr
```

### Functions

Functional types receives an input and produces an output

```txt
(arg1 T, arg2 U) -> V
```

## Casting

Castings are convertions between different types. The important part of a cast is that it only exists between types in the same category, so the casting function never fails.

There are automatic castings allowed in Aura. But only from more general types to more specialized if, and only if, the types are structuraly identical.

It means `Int` auto casts to `I32`, also `T` auto casts to `(T)`, but not from `Float` to `Int`.
