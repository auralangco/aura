# Declaring Functions

We use the `decl` keyword to declare a function. A simple function declaration is as follows:

```rs
func my_function(arg1 Int, arg2 Float, arg3 String) -> Bool {
    // Body
}
```

But that's not all, just like the `main` we can remove the `{}` if its body is just one line long like:

```rs
// `:=` operator needed since we have a type other than Void
func sum(a Int, b Int) -> Int := a + b
```

If the function has no return we might omit it

```rs
func print_sum(a Int, b Int) -> {
    println(a + b)
}
```

Notice that no semicolon is needed at the end of a function declaration since it's a top level declaration.

To call a function just pass the name and the arguments in a comma separated list

```rs
func main -> {
    print_sum(1, 7); //>> 8
}
```

The signature of the funciton describes its type. In the above example the type of `sum` is `(a Int, b Int) -> Int` 
while the type of `print_sum` is `(a Int, b Int) -> Void`. And yes, the parameter name is part of the type, but they 
can be coverted to/from function types where the arguments aren't named. So `(a Int, b Int) -> Int` can be converted to
`(Int, Int) -> Int`.

## Static Parameters

Before the list of parameters, a function might ask for static parameters, ie, arguments whose value are known 
at compile-time. Any literal value can be used, values subtyped as `$static` as well and also types. 

```rs
func build_array(T, len USize$static; elem T) -> Array(T, len) := [len; elem]
```

Again, the static parameters also are part of the function type, so the type of `build_array` is `(T, len USize$static; elem T) -> Array(T, len)`
that can be converted to `(T, len USize$static; T) -> Array(T, len)`

## Closures

Since functions can be used as a value, closures are a way of instantiate functions on the fly.

```rs
//! main.aura

func main -> {
    name := "Aura";

    closure () -> Void := () -> { println(name) };

    closure(); //>> Aura
}
```

New syntax, lets take a look:

- `closure () -> Void`: we will assign a function that takes no arguments and returns nothing
- `() -> { println(name) }`: here we create a function that will print the value of `name` we it's called

Pay attention to an important fact: the closure captures the environment. It means that it will hold some sort 
of reference to the variable `name` declared in the `main` body. What happens it we redeclare `name`?

```rs
//! main.aura

func main -> {
    name := "Aura";

    closure () -> Void := () -> { println(name) };

    name := "aruA";

    closure(); //>> Aura
}
```

Nothing changes. This is because `name` is a immutable string. What it it's mutable?

```rs
//! main.aura

main -> {
    name := @mut "Aura";

    closure () -> Void := () -> { println(name) };

    name = "aruA";

    closure(); //>> aruA
}
```

Interesting. Since `name` is now mutable the value changes. If you don't what it to change create a immutable 
variable with the `@imm` macro

```rs
//! main.aura

main -> {
    name := @mut "Aura";

    i_name := @imm name;
    closure () -> Void := () -> { println(i_name) };

    name := "aruA";

    closure(); //>> Aura
}
```

Since functions are values we can return them from other functions

```rs
//!main.aura

func myfunc() -> () -> String {
    name := "MyFunc"

    () -> { name }
}

func main -> {
    cb := myfunc()
    println(cb()); //>> MyFunc 
}
```

If you want to call a function that is an expression (say the return value from another function) you might add an extra set of parenthesis

```rs
func main -> {
    (myfunc())();
}
```

Or use the pipe operator

## Piping

Remember we said that temporary variables might not be even needed? Well we were talking about `|>`, the pipe operator. It's similar to Elixir/F# pipe operator. On the left side goes the arguments on the right side, the function to be called.

```rs
func main -> {
    x Int := ("100", "0") |> String:replace |> String:parse |> Result:unwrap;
}
```

The above takes the tuple `("100", "0")` calls replace on `"100"`, returning `"10"` then parses it and unwraps the result (extracts the `succ` or crashes otherwise)

This is a core feature for functional programming so you can actually see your program as a transformation of values instead of a sequence of steps

```rs
func main -> {
    // You might also write it this way
    x Int := ("100", "0") 
        |> String:replace 
        |> String:parse 
        |> Result:unwrap;
}
```

We could event print it without ever binding it to a variable;

```rs
func main -> {
    ("100", "0") 
        |> String:replace 
        |> String:parse(Int;) // Type annotations needed 
        |> Result:unwrap
        |> println;
}
```

## Labeling Parameters

When declaring a function, one might add an extra identifier to a parameter it, this is useful to create an external name for a parameter.

```rs
func map(n1 Int, n2 Int, with f (Int, Int) -> Int) -> Int := f(n1, n2)
```

If no extra identifier is provided, you're still able to refer to the parameter by the name. The extra `with` identifier is 
how callers to this function will refer to the `f` parameter.

So when calling `map` we might use:

```rs
func main -> {
    // Regular call
    map(1, 2, sum);
    // Calling with the labels name
    map(1, n2 = 2, with = sum);
    // Calling passing the arguments outside of the parenthesis
    map(1, 2) with (x, y) -> { x * y }; 
}
```

### Using labels outside the parenthesis

If the arguments for your function are: functions, lists or matching expressions you might pass outside of the parenthesis identifying them by their label

```rs
func each(T; of collection List(T), do f (T) -> Void) -> ...

func main -> {
    // We already know this syntax, right?
    each ([1, 2, 3]) do (i) -> {
        println(i)
    }

    each of [1, 2, 3] do (i) -> {
        println(i)
    }
}
```

If the function used as an argument does not take any input the `->` might be ommited

```rs
func main -> {
    if(true) then -> { println("True") } else -> { println("What???") }
    if(true) then { println("True") } else { println("What???") }
}
```

## Default Parameters

The later parameters in a function may receive a default value that must be a static value

```rs
func increment(n Int, by value Int = 1) -> Int := n + value
```

Only the later parameters may be defaulted, so when calling, the non-default parameters come first and the later
 might be specified by name

 ```rs
func some_function(a Int, b Int = 8, arg c Float = 2.5, last d String = "default") ...

func main -> {
    some_function(1); // Alright all the remainders have default values
    some_function(4, 0, last = "another value"); // We might define the values for defaulted params in order and then by label
}
 ```

## Inlined Closures

A closure might be bound to the scope it were created. This is useful for macros that change the control flow of the code. Lets first understand what are those macros

- `@return:'scope value`: finishes the execution of the scope `'scope` and sets its return value to `value` 
- `@return value`: returns from the current function scope (same as `@return:'fn value`)
- `@break:'scope value`: finishes the execution of the scope `'scope` and sets its return value to `break(value)` 
- `@break value`: returns from the current function scope (same as `@break:'loop value`)
- `@continue:'scope value`: finishes the execution of the scope `'scope` and sets its return value to `continue(value)` 
- `@continue value`: returns from the current function scope (same as `@continue:'loop value`)

To give a name `'scope` to a scope (`{}`) put `'scope:` before the `{}` or use the `@scope:` macro

Bind a closure to a scope means that the closure cannot exists outside of the given scope, to do so, prepend the function parameters with `'scope` 
so `'scope:() -> { ... }` creates a closure tied to the `'scope` scope

We have 4 special scope names said `'fn`, `'loop`, `'local` and `'static`. The body of a `fn` function is named `'fn` and the body of the `loop` function is named `'loop`, 
all scopes are implicitly named `'local` unless other specified and `'static` refeers to the global scope.

```rs
func main -> {
    // This is a scoped closure
}
```

To describe the type of a scoped closure