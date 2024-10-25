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

A type whose value is the own identifier

- `Atom`: `$my:atom`

### Bool

Booleanic data `true` or `false`

- `Bool`: `@enum(true, false)`

### Void

Represents non-data

- `Void` `@enum(null)`

## Collection Types

### `List(T)`

- A sequence of data of type T

### `Array(n USize, T)`

- A fixed length sequence of data of type T

### `String`

- A sequence of `Char`

### `LinkedList(T)`

- A linked list of type T

### `Map(K #(hash, eq), V)`

- A sequence of data of type `V` indexed by `K` thats both hasheable and equatable

### `@const T`

A variant of a type `T` that only accepts compile time known values

### `Never`

A special value that can be virtually cast to any type but panics if it happens

```rs
type Never = @enum (unvalue)
```

## Algebraic Data Types

### [Enum](./types/enum.md): Sum type

Each variant is named and may have associated data

```rs
@enum ( variant_1 T, variant2 U, variant3 V )
```

A `match` can be used to safely handle each variant

```rs
import (println) = aura/io

type Foo = @enum (foo Int, bar Bool, foobar)

fn foo -> {
    f = Foo.bar(false);

    str = match (f) {
        Foo.foo f => f |> #printable:from,
        Foo.bar b => b |> #printable:from,
        Foo.foobar => "null" |> #printable:from;
    }
    
    println(str);
}
```

### Struct: Product type

Each field is named after an Atom and must have a type

```rs
( field_1 T, field_2 U, field_3 V )
```

Each field can be accessed by the `.` operator

```rs
type Foo = (foo Int, bar Bool)

fn Foo:invert(foo Foo) -> Foo = (-foo.foo, !foo.bar)
```

Also `Foo.bar` is a function like `(foo Foo) -> Bool { foo.bar }`

### Compound: Product type

Similar to struct but the fields are named after natural number literals

```rs
(T, U, V, W)
```

### Unions: Sum type

Similar to enum but the variants are anonymous and are matched using the type in a match

```rs
type Number = @union (Int, Float)

fn foo -> {
    n Number = 10;
    match (n) {
        _ Int => println("n is an Int"),
        _ Float => println("n is a Float"),
    }
}
```

## Monads: computations wrapped by types

### `Nullable(T)`

Brings null-safety to Aura

```rs
type Nullable(T) = @union (T, Void)
```

### `Failable(S, F)`

Makes failures recoverable

```rs
type Failable(T, F) = @enum (ok T, fail F)
```

- `Failable.ok(T)`: the value that represents success
- `Failable.fail(F)`: the value that represents a failure

### `Control(B, C)`

Describes control flow

```rs
type Control(B, C) = @enum (break B, continue C)
```

### `Async(T)`

Produces a handler to deal with potentially long running calls

```rs
handle Async(String) = Async:new do -> { udp:recv("0.0.0.0:4000") };
// Do your stuff
res Result(String) = handle:await();
```

## Functional Types

### Functions

Functional types receives an input and produces an output

```rs
(arg1 T, arg2 U, ...) -> V
Fn((arg1 T, arg2 U, ...), V) // Same as above
CapFn(n USize, caps Array(n, Atom), (arg1 T, arg2 U, ...), V) // Same as above but with capabilities 
```

## Casting

Castings are convertions between different types. The important part of a cast is that it only exists between types in the same category, so the casting function never fails.

There are automatic castings allowed in Aura. But only from more general types to more specialized if, and only if, the types are structuraly identical.

It means `Int` auto casts to `I32`, also `T` auto casts to `(T)`, but not from `Float` to `Int`.

## Type Parameters

By the identifier of the type, a set of `( )` can be used to pass parameters to the type. Those parameters can be either types or compile-time known values

### Generic Types

When declaring a type, any type identifier passed is a generic type

```rs
type List(T) = ... // T is a generic type
type Failure(T, F) = ... // Both T and F are generic types
```

Generics can be bound by tags so we limit what types can be passed

```rs
type Map(K #(hash, eq), V) = ... // V can be any type, but K can only be types tagged as #hash and #eq
```

### Values

When declaring a type, a value identifier followed by its type defines a compile-time known value parameter

```rs
type Array(n USize, T) = ... // Arrays have a fixed compile-time known size
```