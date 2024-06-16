# Calls

Calls execute a function, either to produce state or data. There are some different kinds of calls in Aura:

## Standard

The standard call is by giving the name of the function to be called and then a compound expression with the arguments

```txt
IO::println("Hello World"); // This produces state (prints text to the console)
List:replicate("Hi", 10); // This produces data (a new list)
```

## Partial

By providing less arguments then needed (by using the `_` keyword) is possible to create a partial call that will produce a function to complete the call

```txt
Int:power(2, 5); // 32
Int:power(2, _); // (exp Int) -> Int:power(2, exp)
Int:power(_, 5); // (base Int) -> Int:power(base, 5)
```

## Piping

The `|>` operator passes the values on the left side as the input of the function on the right side.

```txt
"Hello World" |> IO::println;
5 |> Int:power(2, _);
(2, 5) |> Int:power;
```

## Composing

By passing a functional value that returns `T` (such as a function `(A, B, ...) -> T`) as the argument of a call that needs `T` will compose the functions.

```txt
Int:gt(Int:add(2), 2); // (rhs Int) -> Int:gt(Int:add(2, rhs), 2)
Int:mult(Int:add, Int:add); // (lhs (lhs Int, rhs Int), rhs (lhs Int, rhs Int)) -> Int:gt(Int:add(lhs.lhs, lhs.rhs), Int:add(rhs.lhs, rhs.rhs))
```

## Labeled Bodies

If the last arguments of a call are functional values, they can be passed outside the parenthesis with their identfier.

```txt
// func _if(cond Bool, then () -> $T, else () -> $T) -> $T
_if(true) then { 10 } else { 0 }; 

// func _for(col #iterable($T), do ($T) -> Void) -> Void
_for(0..10) do (it Int) -> { IO::println("${it}"); };
```
