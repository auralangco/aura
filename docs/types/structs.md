# Structs: Product Types

Structs are a specialization of compounds where each part is indexed by an identifier as `(v1 T1, v2 T2, ..., vn Tn)`.

## Default Values

A struct may define default values (literals or expressions) for a field if it's the last field or every field after it also has default values.

```
type Car = struct (name String, brand String = "Ferrari", year UInt = 2024)

c Car = ("LaFerrari", year = 2012)
```

## Casting

Since compounds are said more general than structs, the compound `(T1, T2, ..., Tn)` can be auto casted into a struct `(v1 T1, v2 T2, ..., vn Tn)`.

## Labeled Expressions

When building a compound value for a struct, you can pass the last fields using labels even if there are no default values. Also if the type for a field is: list, branch or functional the label can be outside of the parenthesis or even omited if only one argument is being passed outside.

```
type If(T) = struct (cond Bool, then () -> T, else () -> T = () -> { undefined })

main = {
    myif If = (5 == 6) then { 5 } else { 6 };
}
```