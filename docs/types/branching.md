# Branching

The branching type `T => U` is built with patterns and expressions. By calling the match operator (`:=`) the branching structure will execute the first branch whose pattern matches the right hand side of the operator.

```txt
br Int => String = {
    5 => "It's five",
    n : n > 5 => "Greater than five",
    _ => "Less than five"
}

res String = br := 10; // "Greater than five"
```

## Patterns

There are several accepted patterns, but it depends on the nature of `T`. Important to remember that _ can be used as a ignore capture

### Primitive

- Literal: a literal of the type
- Bind: matches anything and captures the value

### String

Regexes with captures can be used with the ```x`` ``` notation

### Compound

Same as primitive, but for each field of the compound

### Structure

Same as compound but the name given to the bind matters

### Enumeration

Agregated atoms `@atom(value)`are used as the pattern. Where pattern matching standards also apply to the value

### Lists

`[ ]` are used to the define the pattern, each element passed will be matched with the initial part of the list, the last element matches the remainder of the list.

### Map / Object

TODO

### Tag

A `value Type` can be used to capture a value of a tag type downcasting it.

```rs
text = 10 #> #printable;
res = {
    str String => "It's a string",
    int Int => "It's a number",
    _ => "Unknown" 
} := text
```
