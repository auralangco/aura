#`fn`

It's the function definition keyword used in static scopes. The declaration run as follows:

```rs
fn <value_identifier> <parameters_type> -> <type> (= <expr> | <block_expr> )
```

The parameters declaration (before the `->`) uses struct syntax (comma separated list of typed identifiers) where a list of generics can be put in the beggining and separated with `;`, the output type (after the `->`) can be any type. The parameters or the output may be omitted if the input is `()` or the output is `Void`. The fields are available as binded identifiers in the expression passed as the body. The body expression type must be a type that is implicitly castable into the output type of the function.

```rs
fn sum(T #sum; a T; b T) -> T = a + b 
```

## `matchfn`

Syntatical sugar for a `fn` that does a `match` with the input parameters.

```rs
matchfn foo_matchfn(a #number, b #number) -> #number {
    (Int, Int) => a + b,
    (Float, Float) => a + b,
    (Int, Float) => a:into(Float) + b,
    (Float, Int) => a + b:into(Float),
    _ => unimplemented()
}

fn foo_fn(a #number, b #number) -> #number = match((a, b)) {
    (Int, Int) => a + b,
    (Float, Float) => a + b,
    (Int, Float) => a:into(Float) + b,
    (Float, Int) => a + b:into(Float),
    _ => unimplemented()
}
```

## `loopfn`

Similar to `matchfn` but for the `loop` function. Produces a "recursive" function with tail call optimization by nature.

```rs
// Gets the nth fibbonacci number if fibc = 0 and fibn = 1
loopfn fib_loopfn(n UInt, fibc UInt, fibn UInt) -> UInt {
    if (n == 0) then { 
        break(fibc) 
    } else {
        continue((n - 1, fibn, fibc + fibn))
    }
}

fn fib_fn(n UInt, fibc UInt, fibn UInt) -> UInt = loop((n, fibc, fibn)) do ((n, fibc, fibn)) -> {
    if (n == 0) then { 
        break(fibc) 
    } else {
        continue((n - 1, fibn, fibc + fibn))
    }
}
```