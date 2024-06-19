# Flow

Flow is a type used by many structures that handles control/data flow. It's a enum

```rs
type Flow($N, $B) = enum (:next $N, :break $B)
```

## Branching

[See more](./branching.md)

## Match

The `:=` match operator works over branches (`$T => $U`) checking which pattern matches, and then executes the code. If no branch matches nothing happens. But if the resulting value is meant to be binded, then a error happens.

Also there is the `!:=` operator that checks if the branches **do not match** the pattern.

## Loop

The loop built-in expects a initial value `$T`, a function `($T) -> Flow($T, $U)` and returns `$U`. It will feed the function with the initial, if the function returns `:next(n)`, `n` is feed to the function and then it's run again. Otherwise, `:break(b)` breaks the loop, returning `b`.

## If / Cases

`if` is a function that receives a condition, a callback if the condition succeeds and another if it fails. If you need more cases, then use `cases`.

```rs
fn if(cond Bool, then () -> $T, else () -> $T) -> $T
fn cases(do Void => $T) -> $T
``` 
