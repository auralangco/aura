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
( T :variant-1 | :variant-2 | U :variant-2 )
```

### Struct: Product type

Each field is named after an Atom and must have a type

```elixir
( T :field-1, U :field-2, V :field-3 )
```

### Compound: Product type

```elixir
(T, U, V, W)
```

## Monads: computations wrapped by types

### `Nullable(T)`

Brings null-safety to Aura

- `:some(T)`: a non-null value
- `:null`: represents an empty value

### `Failable(T)`

Makes failures recoverable

- `:succ(T)`: the value that represents success
- `:fail(#failure)`: the value that represents a failure (a `#failure`)

### `Action(T)`

Wraps the result of `act` to encapsulate side effects

### `Async(T)`

Wraps the result of `async` to encapsulate assyncronous execution