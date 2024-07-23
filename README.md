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
type Map(T, U) = (value T, map T -> U)

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
    val max_num Number = 2_147_483_647
}
```

It can be called as `Type:assoc_val` outside the definition scope. As with functions and types, it can be defined outside the definition scope

```rs
type Number = Int

val max_num Number = 2_147_483_647
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

### Using Tags

Tags works as union types of all types that are tagged (compound tags works as set of types in the intersection of the tags being composed). When calling a tag associated function for a known type use the `Type#tag:function` notation. When using the tag as a union type just call `#tag:function` and give enough information so the compiler may in infer the type.

```rs
Int#from:from(WeekDay.sunday) // We know which implementation of #from:from to use
#from:from(WeekDay.sunday) $$ Int // Same thing here since we assert the output type is Int
#printable:format(12345) // Here we know it's Int#printable:format
```

```rs
fn sum(a #into(Int), b #into(Int)) -> Int = #into:into(Int; a) + #into:into(Int; b)

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
- Output: `-> Void` (can be ommited), `Result(Void, #failure)`

```rs
main {} // () -> Void
main (argc Int, argv List(String)) {} // (Int, String) -> Void
main -> Result((), #failure) {succ(null)} // () -> Result((), #failure)
main (argc Int, argv List(String)) -> Result((), #failure) { succ(()) } // (argc Int, argv List(String)) -> Result((), #failure)
```

Also if the main body is just an expression the `=` can be used just like in regular functions

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

### Output

The resulting type of the function, if it's `-> Void` both the arrow and the type can be ommited (`fn println(value #printable)` or `fn println(value #printable) -> Void`).

### Body

The body is the code that is evalutated once the function is ran if it's only an expression just use a `= expression` as the body. Other wise if a block of code is needed use `{ statement; statement; statement }` (the last `;` and the initial `=` can be ommited) and the statement value will be used as the value of the function.

### Closures

Functions can be used as values, there are three ways of creating functions: anonymous functions, composition and currying. And both support capture of environment (it means, they are closures).

#### Anonymous functions

Using `(args, ...) -> expression` a function literal is created, inside the expression environment values can be captured, it means they are closures.

```rs
List(Int):filter([25, 0, -10, 45, 10], (elem) -> elem > 10)
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

In a function call, arguments assigned with `_` are curried out, so instead of calling the function, a new closure is created with the needed values.

```rs
increment = sum(1, _); // (b Int) -> sum(1, b)
alt_sum = sum(_, _); // (a, b) -> sum(a, b)
```

Currying is also supported with compounds and structs if the type is specified (both within the parenthesis or by the context)

```rs
(_ String, 123); // (a) -> (a, 123)
(_, _) $$ Bool, Bool // (a, b) -> (a, b)
```

### Calling

There are two forms of calling functions: `( )` and `|>`. The former can be used only if the function is bound to an identifier.

#### `( )`

Since the arguments use the same struct notation to define the input type, we use a similar notation for the one used to build structs. You can pass the arguments in order as expected. The last arguments can be labeled with `label =` so they can be specified out-of-order or if the arguments are `List`, `=>` or `->` they can be labeled outside the parenthesis (the label can be ommited if only one argument is being passed the label can be ommited). For `->` being passed outside if the input `(arg, arg2, arg3...) ->` can be ommited and just `{ }` be used refering to the input arguments as `it` or `it.0`, `it.1`, etc.

```rs
main {
    // fn if(T; cond Bool, then () -> T, else () -> T = () -> undefined)
    if (true, () -> println("Hello World"), () -> println("Good bye")) // >> Hello World
    
    if (5 == 6) {
        println("This shouldn't be printed")
    } // This can't be used as a value since the type is Undefined as said by the default `else` callback

    res = if (-1 > 1) then { "Fizz" } else { "Buzz" }; // if (-1 > 1) else { "Buzz" } then { "Fizz" } is also valid but not semantic in this context

    List:filter([1, 2, 3, 4, 5]) { it % 2 == 0 }; // [2, 4]

    each () values ["Hello", "World", "Aura"] do (str) -> { 
        String:upper(str) 
    }; // ["HELLO", "WORLD", "AURA"]

    each(["Hello", "World", "Aura"], String:upper); // ["HELLO", "WORLD", "AURA"]
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
|> List:filter(_) { it % 2 == 1 } // [ 1, 3, 5, 7 ]
|> #iter:map(_) { it * 2 } // [ 2, 6, 10, 14 ]
|> #iter:reduce(_, 0) (acc, elem) -> { acc + elem } // 32 
```

## `final`

The `final` keyword can prepend `val`, `fn`, and members of `import` to specify that those names cannot be shadowed. It means the name cannot be binded again in function bodies/parameters or other `val` and `fn` statements. But it can still be used in other contexts like fields and variants.

```rs
import (final println) = aura/io

final fn foo() {}

main {
    foo Int = 90; // ERROR: `foo` cannot be shadowed
    println String = "Lorem ipsum dolor"; // ERROR: `println` cannot be shadowed
}
```

By default `type`, `alias` and `external` declarations are `final`

## `external`

Since Aura is multitarget, some code might be implemented natively in the target language. `external` is used then to tell the compiler that the symbol is defined outside Aura code.

To use `external` prepend a definition (`val`, `type` or `fn`) with `external "Lang" identifier` where `"Lang"` is the string that identifies the target language.

The general syntax is: `external "Lang" identifier (val | type | fn) external-identifier [type]`. The `external` keyword, followed by the string representing the target language, the name to bind this external symbol in Aura code, what kind of symbol it is (value, type or function) the name in the target language and finally the type expression that represents this symbol.

```rs
external "C" String type String // Like a typedef const char* String;
external "JS" String type string

external "C" Void type void
external "JS" Void type void

external "C" println fn println(String) -> Void // expects that a `void println(String)` exists in C code
external "JS" println fn log(String) -> Void // expects that a `function log(string): void` exists in JS code
```

Since the target language naming rules might not be as restrictive as Aura's one can rename the symbol in Aura code, the `external-identifier` can be anything that follows the regex `[a-zA-Z_][a-zA-Z0-9_]*`

## Type System

### Primitive Types

- `I8`, `I16`, `Int`, `I32`, `I64`
- `U8`, `U16`, `UInt`, `U32`, `U64`
- `Float`, `Double`
- `Bool`
- `Char`

All those types have no fields, their information can only be accessed as a whole. They can be pattern matched using their literals

```rs
match (6 $$ Int) {
    1 => println("One"),
    2 => println("Two"),
    3 => println("Three"),
    4 => println("Four"),
    5 => println("Five"),
    6 => println("Six"),
    x => println(x:to_string()),
}
```

### Derivate Types

- `List(T)`
- `String`

Types that are derived from primitives and can be accessed in parts. `List` and `String` are both `#indexable` and `#iterable` so can be transformed and accessed in many different ways. The literal for `List` is a comma separated list of expressions with `[ ]` while for `String` is a double quoted text.

```rs
l List(Float) = [8.1, 12.4, 9.6, 4.02];
l:get(3); // Nullable.some(4.02) $$ Nullable(Float)
s String = "Hello World":map() { it:upper() }:reverse(); // "DLROW OLLEH"
```

They can be pattern matched using literals and `++` operator

### Compound Types

- `(T, U, ...)`: a comma separated list of types within `( )`

A compound type is a type which has parts and each part can be of a different type (AKA a _product type_ in ADT language). Its parts can be accessed with `.n` operator where `n` is an integer literal (non-negative).

Pattern matching for compounds is pattern matching againts each component

```rs
match (("Hello", false, 8) $$ (String, Bool, Int)) {
    (text, false, 6) => ..., // Won't match
    ("He" ++ ending, _, 8) => ... // Matches
    value => // Catch all
}
```

Components that aren't matchable must use _catch all_ patterns or ignored

### Union Types

- `T | U | ...`: a pipe separated list of types

The dual of a compound type, its value if of one of its variants which can only be accessed by pattern matching (AKA a _sum type_).

Pattern matching for unions is checking every possible variant:

```rs
match (7.5 $$ Int | Float | Bool) {
    i $$ Int => ..., // Won't match
    7.2 => ..., // Won't match
    f $$ Float ~ f < 10.0 => ...,// Matches
    false => ...,// Won't match
    _ => ...,// Catch all
}
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

### Enum Types

- `t T | u U | ...`:  a union with name variants

Enums behave similar to unions but their variants are named (this allows different variants to wrap the same type). If the variant type isn't Void the value can be pattern matched using `( )`

```rs
type Number = i Int | f Float | nan

match (Number.i(6)) {
    Number.i(i) ~ i > 6 => ,// Won't match
    Number.nan => , // NaN
    Number.f(f) => ,// Won't match
    Number.i(i) => //Matches
}
```

[WIP]: Syntax for anonymous enums

### Functional Types

### Tag Types

## Naming Rules

> Those are not conventions nor recommendations

- `snake_case` (`[a-z][a-z0-9_]*`): values (`val`, binds, function parameters and pattern match captures), functions, fields (in structs), variants (in enums)
- `PascalCase` (`[A-Z][a-zA-Z]*`): types
- `#kebab-case` (`#[a-z][a-z-]*`): tags

## Keywords

- `external`: The current definition is made in an external language
- `enum`*: [WIP] declares a enum type
- `final`: The current definition identifier cannot be shadowed
- `import`: Imports a module
- `lib`: Defines the current module as a library and the exposed interface (what simbols are exported)
- `main`: Defines the current module as an executable and defines the entrypoint code
- `fn`: Defines a function
- `matchfn`: Defines a function using pattern matching in it's declaration
- `loopfn`: Defines a function with tail call optimization in it's root
- `mod`: Declares a submodule as exportable in a `lib` statement
- `struct`*: [WIP] declares a struct type
- `tag`: Both defines a new tag or tags an existing type
- `type`: Defines a type
- `union`*: [WIP] declares a union type
- `val`: Defines a compiletime known constant value

## Operators

- `=` bind operator
- `+ - * / % **` arithmetic operators
- `=+ =- =* =/ =% =** ` bind arithmetic operators
- `&& || ! == != > < >= <=` logic operators
- `=!` bind not operator
- `++` concatenation operator
- `=++` bind concat operator
- `~:` composition operator
- `_` currying operator
- `[ ]` list operator
- `{ }` block operator
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
- `??` hard-unwrap operator. Gets the value wrapped or crashes otherwise
- `=??` bind unwrap operator
- `?=` unwrap-or-default operator if the lhs value can't be unwrapped returns rhs
- `?.` safe field access operator
- `?>` safe piping operator