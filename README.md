<h1 align="center">
    Aura Lang
</h1>

<div  align="center">
    <img src="https://img.shields.io/github/stars/auralangco/aura" alt="GitHub stars" href="https://github.com/auralangco/aura/stargazers" />
    <img src="https://img.shields.io/github/release/auralangco/aura" alt="GitHub release" href="https://github.com/auralangco/aura/releases" />
    <img src="https://img.shields.io/github/license/auralangco/aura" alt="License" href="https://raw.githubusercontent.com/auralangco/aura/master/LICENSE" />
</div>

Aura is a functional multitarget programming language. It means it's meant to be functional first, and easy to run in multiple platforms by transpiling to other languages.

> This is still an alpha specification with no working implementation
> My implementation of a compiler is [Aurac](https://github.com/auralangco/aurac)

## Table of Contents

- [Core Features](#core-features)
- [First Steps](#first-steps)
- [`import`](#import)
- [`alias`](#alias)
- [`type`](#type)
- [`tag`](#tag)
- [`val`](#val)
- [`main`](#main)
- [`fn`](#fn)
- [Operators](#operators)


## Core Features

### Immutability

There are no variables in Aura, you can bind values to names and rebind them later, but no mutability is allowed.

### Functional

Aura implements several functional programming features such as: higher order functions, functions as values, closures, etc

### Type Safe

Aura uses a type system that ensures operations validity is checked during compile time while add features so castings aren't too verbose

### Consistent Syntax

Aura tries to provide a consistent syntax so user created constructs looks like built in constructs.

## First Steps

An Aura file can represent a library or an executable

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
alias succ Failure.succ
alias fail Failure.fail
alias null Void.null
alias some Nullable.some
alias none Nullable.none
alias next Control.next
alias break Control.break
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
type Car = struct (name String, brand String, year UInt = 2024)

val c1 Car = ("LaFerrari", "Ferrari", 2012)
val c2 Car = ("Huracan", year = 2015, brand = "Lamborghini")
val c3 Car = Car("Zonda R", "Pagani")
```

The final fields may have default values. There is an implicit casting from (String, String, UInt) for Car, but not the other way around (again, the tag is implemented).

Note: if the type has generics, pass them before a `;` inside the parenthesis (if needed, Aura has type inference)

```rs
type Foo(T, U) = struct (foo T, bar U)

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
type Number = union (Int, Float)

val n Number = 8
```

There is an implicit cast from a type T to a union type U if T is in the union of types that form U. But there is no way to cast U into T. Use a `match` for this.

```rs
match(n) do {
    Int => println("It's an integer ${n}"),
    Float => println("It's a float ${n}")
}
```

### Enums

A enum is a union whose variants are named.

```rs
type Number = enum (i Int, f Float, v)
```

If no type is provided for a variant it's `Void`

To build a value of a enum just specify the variant and the value.

```rs
val n Number = Number.f(8.8)
```

To work with enums use a `match`

```rs
match (n) do {
    Number.i(i) => println("It's an integer ${i}"),
    Number.f(f) => println("It's a float ${f}")
}
```

### Associated Functions

A type can have functions bound to it (called as `Type:function_name`). Those are defined with `fn` inside `{ }` after the type definition

```rs
type Number = enum (i Int, f Float) {
    fn as_int(self) -> Int = match(self) do {
        Self.i(i) => i,
        Self.f(f) => Int:from(f),
    }

    fn as_float(self) -> Float = match(self) do {
        Self.i(i) => Float:from(i),
        Self.f(f) => f,
    }
}
```

Notice the `self` and `Self` keywords. `Self` means the current type (with the same generic parameters). While `self` is short for `self Self` it means the first parameter is of the `Self` type (`self` can only be used as the first parameter).

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

It can be called as `Type:AssocType` outside the definition scope.

As with associated functions, it can also be defined outside the definition scope

```rs
type Integer = Int

type Integer:Larger = I64
```

### Associated Values

Also values can be associated to types using similar syntax

```rs
type Number = Int {
    val max_num Self = 2_147_483_647
}
```

It can be called as `Type:assoc_val` outside the definition scope. As with functions and types, it can be defined outside the definition scope

```rs
type Number = Int

val Number:max_num Number = 2_147_483_647
```

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

If more than one tag is required use a compound tag `#(tag1, tag2, tag3)`

### Tagging a Type

The `tag` statement is also used to tag a type. Just specify the type being tagged, the tag and then fulfill the associated members definitions.

```rs
type WeekDay = enum (sunday, monday, tuesday, wednesday, thursday, friday, saturday)

tag #from(WeekDay) = Int {
    fn from(value WeekDay) -> Self = match(value) do {
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

### Using Tags

Tags works as union types of all types that are tagged (compound tags works as set of types in the intersection of the tags being composed). When calling a tag associated function for a known type use the `Type#tag:function` notation. When using the tag as a union type just call `#tag:function` and give enough information so the compiler may in infer the type.

```rs
Int#from:from(WeekDay.sunday) // We know which implementation of #from:from to use
#from:from(WeekDay.sunday) $$ Int // Same thing here since we assert the output type is Int
#printable:format(12345) // Here we know it's Int#printable:format
```

```rs
fn sum(a #into(Int), b #into(Int)) -> Int = a:into() + b:into()

println(sum(Bool.true, 89.9)); // 1 + 89 = 90
```

Tags associated members can be defined outside the definition scope, but they must provide a default value.

## `val`

Declaring global values can be done using `val`. Just keep in mind that a `val` is a global constant but it's name can't be shadowed. The type must be explicitly declared.

```rs
val hostname String = "localhost"
val port U16 = 8080
```

Only literals can be passed to `val` (it means, no function calls) and they are evaluated in order.

## `main`

The entrypoint for a executable program, is just a short-hand for `fn main`. There are some different possible signatures for the entrypoint type:

- Input: `()` (can be ommited), `(Int, List(String))`
- Output: `-> Void` (can be ommited), `Failure(Void, #failure)`

```rs
main -> {} // () -> Void
main (argc Int, argv List(String)) -> {} // (Int, List(String)) -> Void
main -> Failure((), #failure) {ok(null)} // () -> Failure(Void, #failure)
main (argc Int, argv List(String)) -> Failure((), #failure) { ok(null) } // (argc Int, argv List(String)) -> Failure((), #failure)
```

Also if the main body is just an expression the `=` can be used just like in regular functions

```rs
main = println("Hello World")
```

## `lib`

## `fn`

The function definition follows the pattern:

```rs
fn identifier Input -> Output = body
```

### The identifier

Functions use `snake_case` names and can be prefixed with `Type:` if they're associated to `Type`

### Input

The input uses the same struct notation: a comma separated list of identifiers with a explicit type and the last fields may have a default value after a `=` sign.

Any generics the function may use are defined before a `;` inside the parenthesis

If the input is `()` it can be ommited

### Output

The resulting type of the function, if it's `-> Void` both the type can be ommited (`fn println(value #printable) ->` or `fn println(value #printable) -> Void`).

### Body

The body is the code that is evalutated once the function is ran if it's only an expression just use a `= expression` as the body. Other wise if a block of code is needed use `{ statement; statement; statement }` (the last `;` and the initial `=` can be ommited) and the statement value will be used as the value of the function.

### Closures

Functions can be used as values, there are three ways of creating functions: anonymous functions, composition and currying. And both support capture of environment (it means, they are closures).

#### Anonymous functions

Using `(args, ...) -> expression` a function literal is created, inside the expression environment values can be captured, it means they are closures.

```rs
List(Int):filter([25, 0, -10, 45, 10], (elem) -> { elem > 10 })
[25, 0, -10, 45, 10]:filter by (elem) -> { 
    elem > 10 
}
```

#### Composition

If a function needs a value of type `A` as an argument and a function that returns `A` is provided, they are composed.

```rs
fn sum(a Int, b Int) -> Int

sum(10, sum); // (a, b) -> sum(10, sum(a, b))
```

If more arguments are also composed the input types are put into an anonymous struct

```rs
sum(sum, sum); // (a (a Int, b Int), b (a Int, b Int)) -> sum(a |> sum, b |> sum)
```

#### Currying

In a function call, arguments assigned with `_ T` are curried out, so instead of calling the function, a new closure is created with the needed values. The type `T` must be specified

```rs
increment = sum(1, _ Int); // (b Int) -> sum(1, b)
alt_sum = sum(_ Int, _ Int); // (a, b) -> sum(a, b)
```

Currying is also supported with compounds and structs if the type is specified (both within the parenthesis or by the context)

```rs
(_ String, 123); // (a) -> (a, 123)
(_ Bool, _ Bool); // (a, b) -> (a, b)
```

### Calling

There are two forms of calling functions: `( )` and `|>`. The former can be used only if the function is bound to an identifier.

#### `( )`

Since the arguments use the same struct notation to define the input type, we use a similar notation for the one used to build structs. You can pass the arguments in order as expected. The last arguments can be labeled with `label =` so they can be specified out-of-order or if the arguments are `List`, `=>` or `->` they can be labeled outside the parenthesis. For `->` being passed outside if the input is `()` it can be ommited.

```rs
main {
    // fn if(T; cond Bool, then () -> T, else () -> T = () -> { undefined })
    if (true, () -> { println("Hello World") }, () -> { println("Good bye") }) // >> Hello World
    
    if (5 == 6) then -> {
        println("This shouldn't be printed")
    } // This can't be used as a value since the type is Undefined as said by the default `else` callback

    res = if (-1 > 1) then -> { "Fizz" } else -> { "Buzz" }; // if (-1 > 1) else { "Buzz" } then { "Fizz" } is also valid but not semantic in this context

    List:filter([1, 2, 3, 4, 5]) by (it) -> { it % 2 == 0 }; // [2, 4]

    foreach of ["Hello", "World", "Aura"] do (str) -> { 
        String:upper(str) 
    }; // ["HELLO", "WORLD", "AURA"]

    foreach(["Hello", "World", "Aura"], String:upper); // ["HELLO", "WORLD", "AURA"]
}
```

#### `|>`

The forward piping operator calls the function passed as the right operand with the values passed as the left operand.

```rs
"Hello World" |> println;
```

The cool part about this is that functions that aren't bound to a identifier can be called. Also it gives the visual effect of data transformation that is a key concept of functional programming languages.

```rs
[1, 2, 3, 4, 5, 6, 7, 8]
|> List:filter(_) by (it) -> { it % 2 == 1 } // [ 1, 3, 5, 7 ]
|> List:map(_) with (it) -> { it * 2 } // [ 2, 6, 10, 14 ]
|> List:reduce(_, 0) with (acc, elem) -> { acc + elem } // 32 
```

## Type System

### Primitive Types

- `I8`, `I16`, `Int`, `I32`, `I64`
- `U8`, `U16`, `UInt`, `U32`, `U64`
- `Float`, `Double`
- `Bool`
- `Char`

All those types have no fields, their information can only be accessed as a whole. They can be pattern matched using their literals

```rs
match (6 $$ Int) do {
    1 => println("One"),
    2 => println("Two"),
    3 => println("Three"),
    4 => println("Four"),
    5 => println("Five"),
    6 => println("Six"),
    x => println(x),
}
```

### Derivate Types

- `List(T)`
- `String`

Types that are derived from primitives and can be accessed in parts. `List` and `String` are both `#indexable` and `#iterable` so can be transformed and accessed in many different ways. The literal for `List` is a comma separated list of expressions with `[ ]` while for `String` is a double quoted text.

```rs
l List(Float) = [8.1, 12.4, 9.6, 4.02];
l:get(3); // Nullable.some(4.02) $$ Nullable(Float)
s String = "Hello World":map(Char:upper):reverse(); // "DLROW OLLEH"
```

They can be pattern matched using literals and `++` operator

### Compound Types

- `(T, U, ...)`: a comma separated list of types within `( )`

A compound type is a type which has parts and each part can be of a different type (AKA a _product type_ in ADT language). Its parts can be accessed with `.n` operator where `n` is an integer literal (non-negative).

Pattern matching for compounds is pattern matching againts each component

```rs
match (("Hello", false, 8) $$ (String, Bool, Int)) {
    (text, false, 6) => ..., // Won't match
    ("Hello", _, 8) => ... // Matches
    value => // Catch all
}
```

Components that aren't matchable must use _catch all_ patterns or ignored

#### Compound Values

To create a new compound values create a comma-separated list of values delimited by parenthesis.

```rs
val origin (Int, Int) = (0, 0) // A compound value
```

### Union Types

- `@union (T, U, ...)`: a pipe separated list of types

The dual of a compound type, its value if of one of its variants which can only be accessed by pattern matching (AKA a _sum type_).

Pattern matching for unions is checking every possible variant:

```rs
match (7.5 $$ @union (Int, Float, Bool)) {
    i $$ Int => ..., // Won't match
    7.2 => ..., // Won't match
    f $$ Float ~ f < 10.0 => ...,// Matches
    false => ...,// Won't match
    _ => ...,// Catch all
}
```

#### Union Values

A value of type `T` can automatically casted into a union that contains `T`

```rs
val number @union (Int, Float) = 6.28 // This is a Float, but is autocasted into a union (Int, Float)
```

### Struct Types

- `(t T, u U, ...)`: a compound with named components

Structs are a less generic version of compounds where each component is identified with a name. Pattern match for structs is similar to pattern match for compounds except that:

- The later fields can be ignored using `...`
- Before `...` fields can be matched out of order using `field_name =` before the pattern
- The first fields can be matched in-order

```rs
match ((name = "John Doe", age = 42) $$ (name String, age Int)) {
    (age = 43, ...) => ,// Won't match
    (n, a) ~ a < 30 => ,// Same
    ("John Doe", age) ~ age >= 30 => , // Matches
    _ => // Catch all 
}
```

Structs support the `.` and `=.` operations:

- `expr.field` gets the value of the field `field`
- `Type.ident` gets a function that receives `Type` and return the value of the field `field`
- `expr.field(new_value)` produces a copy of `expr` but replacing the value of `field` to `new_value`
- `ident=.field` gets the value of the field `field` and binds it to `ident`
- `ident=.field(new_value)` produces a copy of `ident` but replacing the value of `field` to `new_value` and binds it to `ident`

#### Struct Values

First of all, a compound value that is structurally identical to a struct can be autocasted. Remember, the order of the fields matter.

```rs
type Person = (name String, age Int)

val doe Person = ("John Doe", 37) // This works 
val ipsum Person = (42, "Lorem Ipsum") // This doesnt, the order matters
```

A value can be created for an annonymous struct type if all the fields are labelled

```rs
main {
    // A value of an annonymous struct type struct (name String, color String, value Float) 
    grape = (name = "Grape", color = "Purple", value = 1.5);
}
```

When creating a type for a non-anon struct type, we support positional fields in the begginging and labelled fields at the end (they can't be mixed)

```rs
type Car = (brand String, model String, year Int, color String)

val car Car = ("Ferrari", "Italia", color = "Red", year = 2016)
```

When using labelled fields we support a special syntax for `List(T)`, `T => U` and `T -> U` where they can be labelled outside the parenthesis

```rs
type Craziness = (values List(Int), match Int => Bool, do Bool -> String)

val foo Craziness = () values [1, 2, 3, 4] match {
    x ~ x % 2 == 0 => false,
    _ => true
} do (b) -> {
    b:format()
}
```

### Enum Types

- `@enum (t T, u U, ...)`:  a union with name variants

Enums behave similar to unions but their variants are named (this allows different variants to wrap the same type). If the variant type isn't Void the value can be pattern matched using `( )`

```rs
type Number = @enum (i Int, f Float, nan)

match (Number.i(6)) {
    Number.i(i) ~ i > 6 => ,// Won't match
    Number.nan => , // NaN
    Number.f(f) => ,// Won't match
    Number.i(i) => //Matches
}
```

Enums support the `.` and `=.` operations:

- `Type.variant`: produces a new value with the given `variant`
- `expr.variant`: produces a new value with the given `variant`
- `Type.variant(...)`: produces a new value with the given `variant` where it carries some data (can be used to capture data in pattern matching)
- `expr.variant(...)`: produces a new value with the given `variant` where it carries some data (can be used to capture data in pattern matching)
- `ident=.variant`: if the enum is of the given `variant`, we bind the carried data to `ident`

[WIP]: Syntax for anonymous enums

#### Enum Values

To produce a new value of a given enum either use `Type.variant(...)` or `expr.variant(...)` to produce a value of said variant for the given type (the `( )` are only needed if the variant has any carry data)

```rs
type MaybeNaN = @enum (number Float, nan)

val nan = MaybeNaN.nan
val number = MaybeNaN.number(3.14) // or even nan.number(3.14) if you're too lazy to write `MaybeNaN` again
```

### Functional Types

A function type can be expressed in two ways:

- `(T1, T2, ...) -> U` for closures
- `(a1 T1, a2 T2, ...) -> U` for `fn` functions

Basicly either a compound type or a struct type followed by an arrow and the output type. The output type can be ommited if it's void, but not the arrow.

### Branching Types

### Tag Types

Tags can be used as types

```rs
val a #add(Int) = 5 // A value that can be added to Int 
```

Moreover, compound tags can be used to specify an even higher amount of tags a type must have to be accepted, the syntax is similar to a compound, but prefixed with `#`, the tags within it don't need to have the `#`

```rs
// In fact, Int can be added to, multiplied by, subtracted by and divided by an Int 
val a #(add(Int), mul(Int), sub(Int), div(Int)) = 42 
```

### Associated Members

Every type (and tag) can have its associated members (`val`, `fn` or `type`), we use `:` to get access to associated members. This operator can be used both with the type or with an expression. If used with the type in a associated function, a new first parameter araises if the function uses `self`, otherwise `self` is bound to the expression being used.

- `Type:TypeIdent` or `expr:TypeIdent`: gets the associated type
- `Type:val_ident` or `expr:val_ident`: gets the associated value
- `Type:fn_ident`: gets the associated function as a function expression with the extra `self` argument
- `expr:fn_ident`: gets the associated function as a function expression without the extra `self` argument (binds it to `expr`)
- `Type:fn_ident(...)` or `expr:fn_ident(...)` : calls the respective associated function

The special `=:` operator can be used with both associated values and functions in the following scenarios:

- `ident=:val_ident`: same as `ident = ident:val_ident`
- `ident=:fn_ident`: same as `ident = ident:fn_ident`
- `ident=:fn_ident(...)`: same as `ident = ident:fn_ident(...)`

### Type Parameters

In the type definition, some generic type parameters can be added within `( )` before the `=` in a `type` statement.

## Naming Rules

> Those are not conventions nor recommendations, they are mandatory

- `snake_case` (`[a-z][a-z0-9_]*`): values (`val`, binds, function parameters and pattern match captures), functions, fields (in structs), variants (in enums)
- `PascalCase` (`[A-Z][a-zA-Z]*`): types
- `#kebab-case` (`#[a-z]+(-[a-z]+)*`): tags
- `@train:case` (`@[a-z]+(:[a-z]+)*:?`): macros
- `$train:case` (`$[a-z]+(:[a-z]+)*:?`): atoms

## Keywords
- `import`: Imports a module
- `alias`: Creates an alias
- `main`: Defines the current module as an executable and defines the entrypoint code
- `fn`: Defines a function
- `tag`: Both defines a new tag or tags an existing type
- `type`: Defines a type
- `val`: Defines a compiletime known constant value

## Operators

- `=` bind operator
- `+ - * / % **` arithmetic operators
- `=+ =- =* =/ =% =** ` bind arithmetic operators
- `&& || ! == != > < >= <=` logic operators
- `++` concatenation operator
- `=++` bind concat operator
- `~:` composition operator
- `_` currying operator
- `[ ]` list operator
- `{ }` scope operator
- `::` compound join operator `(A1, A2, ..., An) :: (B1, B2, ..., Bm) = (A1, A2, ..., An, B1, B2, ..., Bm)`
- `=::` bind join operator
- `\\` compound split operator `(A1, A2, ..., An, B1, ..., Bm) \\ n = ((A1, A2, ..., An), (B1, B2, ..., Bm))`
- `=\\` bind split operator
- `.` property access (access a field or variant)
- `=.` bind property operator
- `:` associated access (access a associated member)
- `=:` bind associated operator
- `...` spread operator
- `->` function operator. Used in the function type notation and closure creation
- `=>` branch operator. Used in the branch type notation and branch maps creation.
- `~` guard operator. Used to separate the pattern capture and the guard in a pattern
- `|>` pipe-forward applies the lhs value as argument to the rhs function
- `$>` type cast operator
- `$$` type assertion operator
- `?!` hard-unwrap operator. Gets the value wrapped or crashes otherwise
- `??` unwrap propagate operator
- `=?!` bind unwrap operator
- `?=` unwrap-or-default operator if the lhs value can't be unwrapped returns rhs
- `?.` safe field access operator
- `?:` safe associated access operator
- `?>` safe piping operator

## Scopes

Scopes are defined by `{ }`

### Function Scope (`->`)

A body that produces a function

```rs
// A Function with no arguments that prints "Hello World"
-> { println("Hello World"); }
// A function that sums `a` with `b` (types are infered from the context)
(a, b) -> { a + b }
```

### Branch Body (`~ =>`)

A body that produces a branching expression

```rs
{ 
    "hello" => println("Hello"),
    "bye" => println("Not hello"),
    _ => println("Idk")
}
```

### Match Function Body

A combination of _function body_ and _branch body_

```rs
(a Int) -> {
    0 => "zero",
    a ~ a < 0 => "negative"
    a ~ a % 2 == 0 => "even",
    _ => "odd"
}
```

### Local Scope

Creates a body for local variable definitions, may return a value, ain't a function because it is still in the scope of the calling function.

```rs
fn foo -> Int { // Function body A
    a = -> { // Function body B
        @return 10 // Return affects this function body B  
    };
    a = { // Still body A 
        @return 20 // Return affects the function body A
    }
}
```

### Contextual Scopes

Scopes may be appended a context using `@context:(...)` macro. Contexts are read by macros

```rs
main -> {
    not_a_promise Int = @context:($async) { // This appends the $async context to this scope
        @await async_fn() // `@await` can only be called withing an async scope
    };
    promise Async(Int) = @async { // This appends the scope and encapsulates the output in an Async value
        @await async_fn()
    }
}
```

### Labeled Scopes

Scopes may also be labeled for early returning macros

```rs
main -> {
    a = @label:($outer) {
        @label:($inner) {
            @return:($outer) 1
        };
        5 // Unreachable
    }
}

```

# Brainstorm
- Subtyping: `F64$m`
- Atoms: `'atom` new syntax
- Mutable types: `@mut` macro or `$mut` subtype
- Extension members: `Int:(Int) -> Int`
- Global objects: `obj`
- Infix call: ``5 `add 6 == add(5, 6)``
