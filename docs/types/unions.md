# Unions

A union describes a type whose value can be of any of its variants.

```rs
type Number = union(Int, I64, Float, F64)

val age Number = 65 // Here it's an Int

val weight Number = 6.5 // Here it's a Float
```

There is implicit cast available for any type `T` into a union where `T` is a variant, but not the other way around because it's not safe. To cast a union into one of its variants use a match.

When matching against a union, the patterns available are:
- Literals (the literal must be a literal of one of the union's variants)
- `Type`
- A bind `name Type`
- A catch all (`name` or `_`)

```rs
main {
    n Number = 56i64;

    match (n) {
        f F64 => ...,
        Float => ...,
        i I64 => ..., // Matches
        x => ..., // Notice that the typeof x is Number
    }
}
```

Notice that repeated variants are absorbed (`union(T, T)` is the same as `T`) and the order of the variants doesn't matter (`union(T, U)` is the same as `union(U, T)`)