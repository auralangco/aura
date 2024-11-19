# Aura Basics

## Folder Structure

Aura is not meant to be a scripting language, so to start a project one will need to scafold a project. A really simple structure is what we want:

```
- bin               # Your binaries
- lib               # All libs your program uses
- src               # Your application code
    - main.aura     # The entrypoint
- .aura             # The project specification
```

We plan to have a project manager to help you with this

## Hello World

What would a hello world program in Aura looks like?

```rs
//! main.aura

main -> {
    println("Hello World"); //>> Hello World
}
```

Seems good right? Let's break it down for you:

- `main`: the keyword that defines the entry point of the application
- `->`: the arrow operator used to define functions, since main takes no args and returns nothing those can be ommited
- `{ }`: the definition of a scope, the function body
- `println`: the function that prints a line (with a `\n` at the end)
- `( )`: the function arguments goes there
- `"Hello World"`: a string literal

Nice but this could be shorter. First of all, the last line in a scope doesn't needs a `;`

```rs
//! main.aura

main -> {
    println("Hello World") //>> Hello World
}
```

Also, functions whose body is a single expression may drop the `{ }`

```rs
//! main.aura

main -> println("Hello World") //>> Hello World
```

## Variables

What about creating variables in Aura? How it looks like?

```rs
//! main.aura

main -> {
    message String := "Hello World";
    println(message) //>> Hello World
}
```

Ok, some new syntax here, let's again break it down

- `message`: the variable name, they must be `snake_case`, may contain numbers but must start with a letter
- `String`: the variable type, all types are `PascalCase`, again, may contain numbers but must start with a capital letter and no underscores
- `:=` the variable declaration operator indicates that `message` is a new variable (this will be clear later)
- `"Hello World"`: we already know this guy, right?

This one could also be shorter since Aura has type inference. A really nice feature that lets types be infered from the context rather than explicitly defined.

```rs
//! main.aura

main -> {
    message := "Hello World";
    println(message) //>> Hello World
}
```

## Mutating Variables

Well, Aura variables are not that variable, we'll discuss more about this when we talk about [mutations](./xxx-mutations.md) but as of now, just remember that variables are immutable by default and to make them mutable you need to add `$mut` to the variable type or prepend the value with `@mut`

```rs
//! main.aura

main -> {
    message String$mut := @mut "Hello World";
    message = "World Hello";
    println(message) //>> World Hello
}
```

Let's break it down for you once again

- `String$mut`: it says that this particular string may suffer from mutations, the consequences will be cleared later
- `@mut "Hello World`: this transforms this string into a mutable string
- `=`: the mutation operator that lets you change the value of a variable
- `"World Hello"`: notice that we don't need to use `@mut` since it's clear that this value must be mutable

Keep in mind that you don't need to use both `$mut` subtype and `@mut` macro so

```rs
//! main.aura

main -> {
    // Those are the same
    message String$mut := "Hello World";
    message := @mut "Hello World";
    
    // But not this one
    message String := @mut "Hello World";

    println(message) //>> Hello World
}
```

## Redeclaring Variables

The above code is not just an example, it's working Aura code, you might redefine your variables giving different types and values to it without mutations

```rs
//! main.aura

main -> {
    x := 100; // x is an Int
    println(x); //>> 100

    x := "A string"; // Now it's a String
    println(x); //>> A string
    
    x := false; // Yep a boolean
    println(x); //>> false
}
```

Now you don't need to invent several names for your temporary variables (and in a second you'll notice that you don't even need them). This really cool feature becomes more interesting when you create local scopes


```rs
//! main.aura

main -> {
    x := 100; // x is 100
    println(x); //>> 100

    { // Remember the scope markers?
        x := 5; // Inside this scope x becomes 5
        println(x); //>> 5
    }
    
    // Here it's 100 once again
    println(x); //>> 100
}
```

## Local Scopes

This one is pretty straightforward. Scopes are local regions of your code whose variables declared inside won't affect anything outside of the scope as we saw from the above example. But also scopes can be used to pack expressions together and return a single value.

```rs
//! main.aura

main -> {
    x := {
        a := 10;
        b := 12;

        a + b // The last expression in a scope is it's return value
    }
}
```

## Basic Types

Time to increase your Aura arsenal with some types you you can play along

- `Int`: integer numbers that are 32-bits long like `10`, `-2024`, `12345678`
- `Float`: floating point numbers that are 32-bits long like `3.1415`
- `Char`: characters like `'a'`, `'~'`, `'ó'` 
- `Bool`: `true` or `false` values
- `Atom`: a self-representing type whose value is it's name like `'some-atom`, `'outer`
- `String`: a sequence of chars like `"I love Aura"`
- `List(T)`: a sequence of values of the same type

Well we kinda introduced you to something completly new that are lists, we'll talk more about them later. We need to discuss more on numeric types in Aura. There isn't only `Int` and `Float`:

- `I8`, `I16`, `I32`, `I64`: integer numbers from 8 to 64 bits
- `U8`, `U16`, `U32`, `U64` and `UInt`: natural numbers from 8 to 64 bits (`UInt` is 32 bits long)
- `F32`, `F64`: 32 and 64 bits long floating point numbers

Also those types support some operators like: 
- `+`, `-`, `*`, `/`: 4 basic math operations (only operate on numbers from the same type) 
- `%`: modulo (only for integer types of same bit width)
- `^`: power operator (only operate on numbers from the same type)
- `==`, `!=` equals and not equals operators
- `<`, `>`, `<=`, `>=` comparison operators
- `&&`, `||` logical AND and OR operators
- `!` logical not operator
- `++` concatenation (for strings and lists)

We also talked about this `Atom` thing. Well, `Atom`s are pretty simple, they are self-representing values. So the value of the atom `'keyword` is `'keyword` and that's it. We'll show later some cool use cases for atoms. But keep in mind that an atom is only equals to itself. This concept of atom is highly inpired by Elixir/Erlang's atoms.

Enough of basics, lets make something a little more fun like [handling input and output from the user](./003-io.md).