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

// `alias`

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

## `alias`

The alias statement is wuite useful to shorten symbols (identifiers for values, types, variants, functions, etc) that are being used. Those are some default alias in every Aura program:

```rs
alias true Bool.true
alias false Bool.false
alias succ Result.succ
alias fail Result.fail
alias null Void.null
alias some Nullable.some
alias none Nullable.none
alias map #functor:map
alias each #iter:each
alias truthy #truthy:truthy
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
type Car = (name String, brand String, year UInt = 2024)

val c1 Car = ("LaFerrari", "Ferrari", 2012)
val c2 Car = ("Huracan", "Lamborghini", year = 2015)
val c3 Car = Car("Zonda R", "Pagani")
```

The final fields may have default values. There is an implicit casting from (String, String, UInt) for Car, but not the other way around (again, the tag is implemented).

Note: if the type has generics, pass them before a `;` inside the parenthesis (if needed, Aura has type inference)

```rs
type Foo(T, U) = (foo T, bar U)

val f Foo(Int, String) = Foo(Int, String; 123, "abc")
```

Also for the later fields, if they are: `List`, `->` or `=>` they can be passed with their identifier outside the parenthesis. Also if only the last non-defaultable field is being passed outside, the label can be omited.

```rs
type Map(T, U) = struct (value T, map T -> U)

val m Map(Int, String) = Map(10) map (value) -> { value*20 } // map (value) -> value*20 // map { it*20 }
```

### Unions

A union type is a type that can assume values of a set of different types

```rs
type Number = Int | Float

val n Number = 8
```

There is an implicit cast from a type T to a union type U if T is in the union of types that form U. But there is no way to cast U into T. Use a `match` for this.

```rs
match(n) {
    Int => println("It's an integer ${n}"),
    Float => println("It's a float ${n}")
}
```

### Enums

A enum is a union whose variants are named.

```rs
type Number = i Int | f Float | v
```

If no type is provided for a variant it's `Void`

To build a value of a enum just specify the variant and the value.

```rs
val n Number = Number.Float(8.8)
```

To work with enums use a `match`

```rs
match (n) {
    Number.i(i) => println("It's an integer ${i}"),
    Number.f(f) => println("It's a float ${f}")
}
```

### Associated Functions

A type can have functions bound to it (called as `Type:function_name`). Those are defined with `fn` inside `{ }` after the type definition

```rs
type Number = i Int | f Float {
    fn as_int(self) -> Int = match(self) {
        Self.i(i) => i,
        Self.f(f) => f |> Float:to_int,
    }

    fn as_float(self) -> Float = match(self) {
        Self.i(i) => i |> Int:to_float,
        Self.f(f) => f,
    }
}
```

Notice the `self` and `Self` keywords. `Self` means the current type (with the same generic parameters). While `self` is short for `self Self` it means the first parameter is of the `Self` type.

Associated functions can be defined outside the type definition by prefixing the function name with the type it's associated to.

```rs
type Foo = Int

fn Foo:bar(self) -> Self = self + 1
```

### Associated Types

A type can have other types bound to it (this gets really useful for tags).

```rs
type Number = Int {
    type Collection = List(Int)

    fn prime_factors(self) -> Collection { ... }
}
```

It can be called as `Type:AssocType` outside the definition scope

### Associated Values

Also values can be associated to types using similar syntax

```rs
type Number = Int {
    val max_num Number = 2_147_483_647
}
```

It can be called as `Type:assoc_val` outside the definition scope

## `tag`

Statements where one can define a new tag or tag a defined type.

### Definining a New Tag

Using the `tag` statement create a tag name (`#kebab-case`) and add a set of `{ }` at the end where the associated members are defined.

```rs
tag #foo {
    fn bar(i Int) -> String
}
```

Inside the definition scope one may create associated functions, values and types using nearly the same syntax as in type definition scopes. But here, the associated members can be defined without a default value so it must be provided when tagging a type. If a default value is given it cannot be overwritten.

```rs
tag #from(T) {
    type Output = Self // The output type is set, cannot be overwritten

    fn from(value T) -> Output // The function body isn't set, must be overwritten
}
```

Another `#` expression can be added after the tag name to specify tags that a type must have before being tagged.

```rs
tag #text #printable { // Only #printable types can be tagged with #text
    ...
} 
```

If more than one tag is required use `#(tag1, tag2, tag3)`

### Tagging a Type

The `tag` statement is also used to tag a type. Just specify the type being tagged, the tag and then fulfill the associated members definitions.

```rs
type WeekDay = sunday | monday | tuesday | wednesday | thursday | friday | saturday

tag Int #from(WeekDay) {
    fn from(value WeekDay) -> Self = match(value) {
        WeekDay.sunday => 1,
        WeekDay.monday => 2,
        WeekDay.tuesday => 3,
        WeekDay.wednesday => 4,
        WeekDay.thursday => 5,
        WeekDay.friday => 6,
        WeekDay.saturday => 7,
    }
}
```

## `val`

Declaring global values can be done using `val`. Just keep in mind that a `val` is a global constant but it's name can't be shadowed. The type must be explicitly declared.

```rs
val hostname String = "localhost"
val port U16 = 8080
```

Only literals can be passed to `val` (it means, no function calls) and they are evaluated in order.

## `main`

## `lib`

## `fn`

## Type System


