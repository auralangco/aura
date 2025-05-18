# Calling Functions

In Aura we have some different ways to call functions and we'll go through all of them meanwhile we increment again out Aura arsenal

## Functions

To this point we have used two functions `println` and `readln`, they're pretty simple and they are declared as so:

```rs
fn println(message Printable) -> { ... }

fn readln() -> Result(String, IOError) { ... }
```

So `println` takes a type that implements the `Printable` interface and returns nothing. While `readln` takes nothing and return a `Result(String, IOError)`. Don't you bother with much other syntax, we'll cover them next. To call those functions just use the name and then a set of parenthesis with the arguments in a comma separated list within it.

```rs
fn main -> {
    // Echoes back whatever is typed in by the user
    println(readln()?!);
}
```

## Methods

Methods are just functions that are tied to a type and we have lot's of them. To call a method use `:` after some value, then the method name and a set of parenthesis with the arguments for the method being called.

```rs
fn main -> {
    x Int := 100;

    x:is_odd(); // -> Bool
    x:is_even(); // -> Bool
    x:is_positive(); // -> Bool
    x:is_negative(); // -> Bool
    x:to_string(); // -> String
    x:to(1000); // (-> IntRange) same as 100..1000

    y Float := 3.1415; 
    y:round(); // (-> Float) Rounds to the closest whole number
    y:floor(); // (-> Float) Rounds to the lower whole number
    y:ceil(); // (-> Float) Rounds to the upper whole number
    y:sqrt(); // (-> Float) Calculates the square root
    y:safe_sqrt(); // (-> Result(Float, SqrtError)) Calculates the square root, is err if it's negative

    s String := "100";
    s:length(); // -> USize the string length
    s:uppercase(); // -> String
    s:lowercase(); // -> String
    s:chars(); // -> List(Char)
    s:parse(Int;); // (-> Result(Int, ParseError)) tries to parse the string to an Int
    s:replace("0"); // (-> String) returns a new string replacing the first ocurrence of "0"
}
```

A method can also be called on the type rather than on the value, and later passing the value as an argument

```rs
fn main -> {
    x Int := 100;

    Int:is_odd(x); // -> Bool
    Int:is_even(x); // -> Bool
    Int:is_positive(x); // -> Bool
    Int:is_negative(x); // -> Bool
    Int:to_string(x); // -> String
    Int:to(x, 1000); // (-> IntRange) same as 100..1000

    y Float := 3.1415; 
    Float:round(y); // (-> Float) Rounds to the closest whole number
    Float:floor(y); // (-> Float) Rounds to the lower whole number
    Float:ceil(y); // (-> Float) Rounds to the upper whole number
    Float:sqrt(y); // (-> Float) Calculates the square root
    Float:safe_sqrt(y); // (-> Result(Float, SqrtError)) Calculates the square root, is err if it's negative

    s String := "100";
    String:length(s); // -> USize the string length
    String:uppercase(s); // -> String
    String:lowercase(s); // -> String
    String:chars(s); // -> List(Char)
    String:replace(s, "0"); // (-> String) returns a new string replacing the first ocurrence of "0"
    String:parse(Int; s); // (-> Result(Int, ParseError)) tries to parse the string to an Int
}
```

We need to clarify something about the calling syntax for `String:parse`. Functions might also receive static parameters, ie, parameters that are known at compile time (literals, types, etc). The function `parse` uses this feature to find out to what we want to parse the string.

```rs
// It's not declared exactly like this, but this it's enough for us right now
fn String:parse(T; self) -> Result(T, ParseError) { ... }
```

The static arguments (for methods and functions) are comma-separeted-listed before regular arguments with a `;` to indicate where the regular arguments start

### Surprise

Some of the control flow stuff we showed earlier are methods.

- `foreach` is a method for any iterable type
- `loop` is a method for any type

So we can do stuff like:

```rs
fn main -> {
    [1, 2, 3]:foreach do (i) -> {
        println(i);
    }
}
```

## Infix Functions

Any function that takes only two arguments might be called infix. Lets take the `to` function for an example

```rs
fn to(L, R; left L, right R) -> (L, R) = (left, right)
```

Again the declaration syntax will be cleared next, but the important is, it takes two arguments (the static arguments doesn't matter for this case) and returns a tuple with the two given arguments.

```rs
fn main -> {
    element := to("height", 1.8);
    element := "height" `to 1.8; // Use the backtick notation to indicate this is an infix call to a function
    element := "height" `to(String, Float) 1.8; // You might pass the static args this way

}
```

## Operators

Almost all operators we showed you all are tied to a function/method so they can be overloaded. Most of them are methods bound to an interface so to implement it to your type you'll need to implement the appropriate interface. Like the `-` operator uses the `minus` method of the `Minus` interface. We'll show this better when talking about interfaces.

## Labelled Calls

This is one of Aura's features we are most proud of, please pay attention to the beauty of labelled calls. The function's arguments can be passed outside of the parenthesis if it's: a function, a matching expression, a list, a tuple or a struct. And you've already used this feature in the previous session.

If can be understood as the function

```rs
fn if(T; cond Truthy, then 'fn:() -> T = () -> { NaV.nav }, else 'fn:() -> T = () -> { NaV.nav } ) -> T
```

It means: `if` is a function that has a static type parameter `T`, a condition that is `Truthy`, two functions that takes no arguments, returns `T` and by default returns `NaV` and are bound to the scope of the calling function called `then` and `else`.
