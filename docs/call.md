# Calls

Calls execute a function, either to produce state or data. There are some different kinds of calls in Aura:

## Standard

The standard call syntax is by giving the identifier of the function to be called and then a struct expression with the arguments

```rs
io:println("Hello World"); // This produces state (prints text to the console)
List:replicate("Hi", 10); // This produces data (a new list)
List:map([1, 2, 3, 4], with = (e) -> { e + 1 }); // Produces a new list by mapping
```

### Partial

By providing less arguments then needed (by using the `_` keyword) is possible to create a partial call that will produce a function to complete the call

```rs
Int:power(2, 5); // 32
Int:power(2, _); // (exp Int) -> Int:power(2, exp)
Int:power(_, 5); // (base Int) -> Int:power(base, 5)
```

## Piping

The `|>` operator passes the values on the left side as the input of the function on the right side.

```rs
"Hello World" |> io:println;
5 |> Int:power(2, _);
(2, 5) |> Int:power;
```

## Composing

By passing a functional value that returns `T` (such as a function `(A, B, ...) -> T`) as the argument of a call that needs `T` will compose the functions.

```rs
Int:gt(Int:add(2), 2); // (rhs Int) -> Int:gt(Int:add(2, rhs), 2)
Int:mult(Int:add, Int:add); // (lhs (lhs Int, rhs Int), rhs (lhs Int, rhs Int)) -> Int:gt(Int:add(lhs.lhs, lhs.rhs), Int:add(rhs.lhs, rhs.rhs))
```

## Labeled Arguments

The later arguments of a call can be passed outside the parenthesis with their identifier. If the value is a List/Array/LinkedList it can be passed between `[ ]`, if it's a match expression can be passed with a match body `{ => }`, if it's a function it can be passed as a function `(...) -> { }` or `-> { }` (if no parameters are needed), otherwise the value can be passed between `{ }` 

```rs
// fn if(T; is Bool, then () -> T = -> { unvalue }, else () -> T = { unvalue }) -> T
if(true) then -> { 10 } else -> { 0 }; 

// fn foreach(T; of #iterable(T), do (T) -> Void) -> Void
foreach of[0, 1, 2, 3, 4] do (it Int) -> { 
    io:println(it); 
};
```

Or if the arguments are branching values

```rs
// fn loop(T; from T, do (T) -> Control(Void, T))
loop(10) do (it) -> {
    case of {
        ~ it > 1 && it:is_odd() => {
            io:println("Odd");
            continue(3*it + 1)
        },
        ~ it > 1 && it:is_even() => {
            io:println("Even");
            continue(it / 2)
        },
        _ => {
            io:println("Reached 1");
            break
        }
    }
}
```
