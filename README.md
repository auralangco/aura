# Aura

Aura is a functional parallelizable multitarget programming language. It means it's meant to be functional first, be easy to run in multicore and in multiple platforms by transpiling to other languages.

## Features

### Immutability

There are no variables in Aura, you may give aliases to values, and that's it.

### Functional

Aura implements several functional programming features such as:

- Functions as values
- Purity and side-effect management
- Pattern matching

### Type Safe

Aura uses a type system that ensures operations validity is checked during compile time

### Static vs Dynamic

Aura atempts to make several compile time operations available in runtime while enforcing proper compile time treatment

## First Steps

An Aura file can represent a library or an executable. The general syntax for a file is:

```rs
// `import` statements

// `type` statements

// `tag` statements

// `val` statements

// `main` statement if it's an executable
// `lib` statement if it's a library

// `fn` statements
```

## `import`

Here is where you gonna import stuff from other Aura files. This helps Aura in building the dependency tree and check what needs to be compiled and do the proper linking of symbols.

```rs
import ~/path/to/module // Import everything from a module
import (symbol1, altname = symbol2) = ~/path/to/module // Import just those symbols and even rebind them
```

## `type`

The `type` statement defines the named types that are going to be used in the program. There are some models of types:

### Aliased

Just an alias for another type. This is quite useful for semantics in your code.

```rs
type Integer = I64

val i Integer = 1234i64
```

There is an implicit cast available for I64 to Integer but not the other way around. But the Integer type gets tagged with `#into(I64)`

### Compounds

This is the same as aliased but aliasing a compound type

```rs
type Pair(T, U) = (T, U)

val p Pair(Int, String) = (123, "abc")
```

The fields in compounds are indexed by an integer literal. The casting logic is the same as aliased types.

### Structs

Just compounds with named fields.

```rs
type Car = struct (name String, brand String, year UInt = 2024)

val c1 Car = ("LaFerrari", "Ferrari", 2012)
val c2 Car = struct ("Huracan", "Lamborghini", year = 2015)
val c3 Car = Car("Zonda R", "Pagani")
```

The final fields may have default values. There is an implicit casting from (String, String, UInt) for Car, but not the other way around (again, the tag is implemented).

Note: if the type has generics, pass them before a `;` inside the parenthesis

```rs
type Foo(T, U) = struct (foo T, bar U)

val f Foo(Int, String) = Foo(Int, String; 123, "abc")
```

Also for the later fields, if they are: `List`, `->` or `=>` they can be passed with their identifier outside the parenthesis. Also if only the last non-defaultable field is being passed outside, the label can be omited.

```rs
type Map(T, U) = struct (value T, map T -> U)

val m Map(Int, String) = Map(Int, String; 10) map { it*20 }
```

### Unions

### Enums

## `val`

## `main`

## `lib`

## `fn`