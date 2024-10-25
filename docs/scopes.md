# Scopes

Scopes are pieces of code delimited by `{ }`.

```rs
main -> { // Beginning of function scope
    a = { // Begginging of local scope
        5 + 6
    }; // End of local scope

    println(a);
} // End of function scope
```

Scopes may produce a value, defined by the last expression in it. Expressions inside a scope are separated by `;`, the trailing `;` is optional.

Scopes have 3 properties:
- `type`: the type it produces
- `label`: the "name" of the scope (`@label:` macro)
- `context`: what kind of context the expressions inside this scope are (`@context:` macro)

```rs
main -> { // label: $main, context: $function
    a = @label:($outer) { // label: $outer, context: $local
        @label:($inner) { // label: $inner, context: $local
            @return:($outer) 11 // This returns from the outer scope binding `a` to 11
        }

        12 // This is unreachable
    }
}
```

## Labels

A way to give names to scopes, this is used by macros that are short-circuiting to early finish the execution of a scope is a given condition mets.

Macros will look upward trying to find where is the closest scope with the given label since labels can be shadowed.

```rs
1. @label:($a) {
2.     @label:($a) {
3.         @return:($a) null // This returns from scope at line 2 not at 1
4.     }
5.     // This line will be executed
6. }
```

## Context

A context is useful for macros because it specifies for what this scope is being used for. Labels are just names, contexts are meaningful.

Some built-in contexts are:
- $loop: this scope is used by a regular looping structure so its value is `Control(B, C)`
- $try: this scope returns a `Failable(T, F)`
- $maybe: this scope returns a `Nullable(T)`
- $async: this scope returns an `Async(T)`

