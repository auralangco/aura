# Control Flow and Decisions

What would a program be if we couldn't make decisions and run different pieces of code in different scenarios?

## `if`

Aura's `if` is just a bit different from what you might be used to

```rs
main -> {
    input Result(_, _) := readln();

    if (input) then {
        println("The input is " ++ input?!);
    } else {
        println("No input at all");
    }
}
```

First notice the `Result(_, _)` syntax, the `_` operator is saying: "Come on type inference, you got this, right?". It's a place holder that can be used when you don't want to write down the full type. But we have this pretty cool `if` syntax:

- `if`: the if **function**
- `(input)`: the if condition. A value of a type that is `#truthy`. For a `Result` it's `true` once it's `succ`
- `then`: the argument to be evaluated once the condition is `true`
- `else`: the argument to be evaluated once the condition is `false`

Notice that `if` is a function just like `println` or `readln` or whatever. And `if` can be used as an expression, it will return the value from the `then` or `else` argument.

```rs
main -> {
    x := if (5 > 6) then { "Craziness" } else { "Alright" };

    println(x); //>> Alright
}
```

Also, `then` and `else` have default values so you my use `if` with only one branch (or even neither). In that case `if` cannot be used as an expression since it will return a `NaV` (not a value) that can't be use as a value.

```rs
main -> {
    if (1 == 1) then { println("This is working") };
    if (1 == 2) else { println("This is also working") };
}
```

If you need more branches then you might want to use:

## `case`

Case is like `if` but for more branches

```rs
main -> {
    input := readln()?!;

    case of {
        ~ input:contains("Aura") => println("Yay! Aura mentioned!"),
        ~ input:length() < 2 => println("Too short"),
        => println("The input is " ++ input)
    }
}
```

Hmmmm ok, this syntax is something really different, but don't you worry, I got you:

- `case`: this calls the `case` function which receives only one argument: a `Match`
- `of`: the argument that receives the `Match` is labelled as `of` so we can pass it outside of the regular `()` for function arguments
- `~ input:contains("Aura") => println("Yay! Aura mentioned!"),`: this is a branch from a `Match` structure, that prints this text if `input` contains the string `"Aura"`

Let us describe this `Match` thing a bit better.

In Aura, pattern matching structures can be described as a value. A `Match` is built with branches, each branch has 3 parts `pattern ~ guard => expr`:

- pattern: the base of a pattern matching, it describes a pattern of how a value is structured (we'll talk about it soon)
- guard: a `#truthy` expression. The branch only matches if both the pattern and the guard matches
- expr: what will be evaluated once the branch matches

Only one branch matches per time

## `match`

## `loop`