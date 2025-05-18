# Control Flow and Decisions

What would a program be if we couldn't make decisions and run different pieces of code in different scenarios?

## `if`

Aura's `if` is just a bit different from what you might be used to

```rs
func main -> {
    input Result(_, _) := readln();

    if (input) then {
        println("The input is " ++ input?!);
    } else {
        println("No input at all");
    }
}
```

First notice the `Result(_, _)` syntax, the `_` operator is saying: "Come on type inference, you got this, right?". It's a place holder that can be used when you don't want to write down the full type. But we have this pretty cool `if` syntax:

- `(input)`: the if condition. A value of a type that is `Truthy`. For a `Result` it's `true` once it's `succ`
- `then`: the expression to be evaluated once the condition is `true`
- `else`: the expression to be evaluated once the condition is `false`

Notice that `if` can be used as an expression, it will return the value from the `then` or `else` argument depending on the condition.

```rs
func main -> {
    x := if (5 > 6) then { "Craziness" } else { "Alright" };

    println(x); //>> Alright
}
```

Also, `then` and `else` have default values so you might use `if` with only one branch (or even neither). In that case `if` cannot be used as an expression since it will return a `NaV` (not a value) that can't be use as a value.

```rs
func main -> {
    if (1 == 1) then { println("This is working") };
    if (1 == 2) else { println("This is also working") };
}
```

Notice that for `then` and `else` be recognized as named arguments, you need them to be in the same line as previous the enclosing brace (`)`, `]` or `}`).


```rs
func main -> {
    // This won't work
    if (1 == 1) 
    then { println("This is working") }
    else { println("This is also working") }
}
```

If you need more branches then you might want to use:

## `case`

It is like `if` but for more branches

```rs
func main -> {
    input := readln()?!;

    case of {
        ~ input:contains("Aura") => println("Yay! Aura mentioned!"),
        ~ input:length() < 2 => println("Too short"),
        => println("The input is: " ++ input)
    }
}
```

Hmmmm ok, this syntax is something really different, but don't you worry, I got you:

- `of`: the body of `case` that is a matching expression
- `~ input:contains("Aura") => println("Yay! Aura mentioned!"),`: this is a branch from a matching expression, that prints this text if `input` contains the string `"Aura"`

Let us describe this matching expression a bit better.

In Aura, pattern matching structures can be described as a value (in fact, this is just an anonymous function). A matching expression is built with branches, each branch has 3 parts `pattern ~ guard => expr`:

- pattern: the base of a pattern matching, it describes a pattern of how a value is structured (we'll talk about it soon)
- guard: a `Truthy` expression. The branch only matches if both the pattern and the guard matches. The guard is optional.
- expr: what will be evaluated once the branch matches

Only one branch matches per time and in `case` the pattern is always empty

## `match`

This function unlocks the full potential of Aura pattern matching system. You already know the matching function expressions: a comma separated list of branches `pattern ~ guard => expr` surrounded by `{ }`. Let's break them down.

- `expr`: you might use literally any expression here, it will be evalutated once the branch matches. Keep in mind the expression of every branch must have the same type otherwise the matching function will return a `NaV`
- `guard`: is an expression that evaluates to a `Truthy` value, it will only be evaluated when the pattern matches and it will determine if the branch matches or not. The guard is optional.
- `pattern`: the heart of the branch, it verifies the form of the values being matched to

```rs
//! main.aura

func main -> {
    x := 8
    match (x) do {
        1 => println("It's 1"),
        2..=7 => println("It's between 2 and 6"),
        8 => println("Yay! It's 8"),
        y ~ y:is_even() => println("It's " ++ y ++ ", an even number"),
        => println("It's an odd number") 
    }
}
```

Let's break this down for you

- `match (x)`: the match is called with the `x` value being passed to it
- `do`: the body of a match 
- `1 => println("It's 1")`: a branch that matches once the input value is `1`
- `2..=7 => println("It's between 2 and 6")`: a branch that matches for any value between 2 and 7 (inclusive)
- `8 => println("Yay! It's 8")`: a branch that matches the value `8`
- `y ~ y:is_even() => println("It's " ++ y ++ ", an even number")`: a branch which pattern matches any value, but the guard will check if it's an even number. Notice that the variable `y` can be used in the expression, this is called a capture.
- `=> println("It's an odd number")`: a branch that always matches

In fact, `match` is just syntactic sugar for a function call to a function that takes a value and a function that takes a value and returns a value.

```rs
func main -> {
    x := 8

    // This is the same as the match above
    {
        1 => println("It's 1"),
        2..=7 => println("It's between 2 and 6"),
        8 => println("Yay! It's 8"),
        y ~ y:is_even() => println("It's " ++ y ++ ", an even number"),
        => println("It's an odd number") 
    }(x)
}
```

### Patterns

As we go on with this you will learn how to pattern match against more values, but let start with the basics:

- `_`: matches all values but won't capture it
- variable: also matches all, but captures the value to the specified name
- *empty*: an empty pattern works similar to `_`, but `_` can be used in more spots
- literals: can be used for: integers, floats, bool, chars, strings, lists and atoms
- ranges: can be used for: integers, floats and chars

#### List Patterns

We have some special syntax for matching against lists

```rs
decl main -> {
    x := [1, 2, 3, 4, 5]
    
    match (x) do {
        [] => println("An empty list"),
        [1, 2] => println("The [1, 2] list"),
        [a, 2] => println("A list [" ++ a ++ ", 2]"),
        [..., 4] => println("A list ending in 4"),
        [...init, 5] => println("A list with init " ++ init ++ " ending with 5"),
        [1, 2, ...tail] => println("A list with tail " ++ tail ++ " starting with 1, 2"),
        [1, 2, ...body, 4, 5] => println("A list starting with 1, 2, ending with 4, 5 with body " ++ body)
        [1, 2, 4, ...]&list ~ list:length() > 3 => println("A list longer than 3 starting with 1, 2, 4: " ++ list)
    }
}
```

Whoa, lot's of patterns

- `[]`: matches only empty lists
- `[1, 2]`: matches only the `[1, 2]` list
- `[a, 2]`: matches a list with 2 elements where the second one is 2 and captures the first element to `a`
- `[..., 4]`: matches any list that ends with 4 and ignores all the other elements
- `[...init, 5]`: matches any list that ends with 5 and captures the other elements to a list `init`
- `[1, 2, ...tail]`: matches lists starting with 1 and 2 and captures the remainders to `tail`
- `[1, 2, ...body, 4, 5]`: matches list starting with 1, 2 and ending in 4, 5 capturing the middle elements to the list `body`
- `[1, 2, 4, ...]&list`: matches lists starting with 1, 2, 4 and captures the **whole** list to `list`

Notice that we are pattern matching on individual elements of the list so we could use any integer pattern not only integer literals. Also if you're confused, the above program would print only `A list with init [1, 2, 3, 4] ending with 5` since it's the first pattern that matches.

The `...` operator is called a *spread* operator it's used to insert values from a list into another. But in this case it's being used to capture sublists of a list. Notice both `init`, `tail`, `body` would be of type `List(Int)` even if they match only a single element.

#### Destructuring Patterns

The pattern used by lists is called a destructuring pattern and have similars for other types that are used in many other places. 

## `foreach`

This looping structure iterates over values and executes some action for each (lol) of them

```rs
func main -> {
    list := [0, 1, 2, 4, 5];

    foreach (list) do (i) -> {
        println(i*2)
    }
}
```

Breaking down the loop:

- `(list)`: the first argument is a collection where to get elements from
- `do`: here goes the function to what will be executed for each element
- `(i) ->`: this captures the current element being iterated on by `foreach`
- `{ println(i*2) }`: here we can use the captured value and do whatever we want with it

No type annotations are needed because of type inference

## `loop`

Differently from most languages, Aura has no `while` loop since it's based on mutations and we try to avoid them. Instead we use the beautiful and immutable `loop`

```rs
func main -> {
    initial := 9;

    // Collatz conjecture
    loop (initial) do (i) -> {
        case of {
            ~ i == 1 => @break,
            ~ i:is_even() => @continue i/2,
            => @continue (i + 3) * 4
        }
    }
}
```

This thing is doing iterations to test the Collatz conjecture, lets break it down:

- `loop (initial)`: defines the initial value to start our loop
- `do (i) ->`: similar to `foreach`s one, captures the current iteration value
- `@break`: tells the loop to stop
- `@continue`: tells the loop to go for the next iteration and sets the next value of `i`

This works but let's clear out some stuff: the body of a loop expects a value of type `Control`. The `@break` and `@continue` behave as regular `break` and `continue` from other languages by early finishing the loop. But you don't need to use them, you could use `break` and `continue` as values of the type `Control`

Similarly to `Result(T, E)`, we have `Control(B, C)` that also has two variants `break(B)` and `continue(C)`. They are used by loops to know when they must stop iterations (also by defining a return value) or to define the value for the next iteration. In out above example, our loop returns nothing once stopped but the iterations use an input value of type `Int` so we have a `Control(Void, Int)`

We could also count (immutably) how many iterations we have done before finishing the loop

```rs
func main -> {
    initial := 9;

    // Collatz conjecture
    iters := loop ((initial, 0)) do ((value, count)) -> {
        case of {
            ~ value == 1 => @break count,
            ~ value % 2 == 0 => @continue (value/2, count + 1),
            => @continue ((value + 3) * 4, count + 1)
        }
    }
}
```

Okay lets see what changed

- `iters :=`: yeah we are getting the break value of the loop and saving it to `iters`
- `loop ((initial, 0))`: we are creating a tuple with two elements: the initial value and the initial count (0, of course)
- `do ((value, i)) ->`: a nice destructuring pattern so we can catch the current value for the iteration and the current count
- `@break count`: when breaking from the loop we will return the current iteration count
- `@continue (value/2, count + 1)`: for the next iteration we now must specify both the value and the next counter (just incrementing the current one)

That's it! We have loops in a way we don't need to use any dirty mutations. Actually 'til this point we didn't need to use `@mut` for anything and that's beautiful.

Next up, time to [create functions](./005-functions.md)