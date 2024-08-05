# Branching

The branching type `T => U` is built with patterns and expressions. By calling the match operator (`~~`) the branching structure will execute the first branch whose pattern matches the right hand side of the operator.

```txt
br Int => String = {
    5 => "It's five",
    n : n > 5 => "Greater than five",
    _ => "Less than five"
}

res String = br ~~ 10; // "Greater than five"
```

## Patterns

There are several accepted patterns, but it depends on the nature of `A` (`A => B`). Important to remember that `_` can be used as a ignore capture (aka catch all)

### Primitive

- Literal: a literal of the type
- Bind: matches anything and captures the value

```
match (12) {
    3 => ...,
    x ~ x > 6 => ...,
    _ => ...
}
``` 

To be exaustive must have a catch all 

### String

String literals

```
match ("Hello World") {
    "Bye World" => ...,
    "Hello World" => ...,
    str ~ str:length() > 5 => ...
}
``` 

To be exaustive must have a catch all 

### Union

Each branch covers one variant of the union. The pattern should be a bind `name Type`, just the type or a pattern of the given type.

```
match (15 $$ Int | Float) {
    x Int ~ x > 60 => ...,
    Float => ...,
    x Int => ...,
}
``` 

To be exaustive, must exaust every variant

### Lists

`[ ]` are used to the define the pattern, each element passed will be matched with the initial part of the list, the last element matches the remainder of the list.

```
match ([1, 2, 3, 4, 5, 6]) {
    [6, 5, x] => ..., // x is the remainder of the list
    [1, 2, 3, []] => ..., // Wont match since the reimainder actually is [4, 5, 6]
    [1, x] ~ x:length() > 0 => ..., // Matches 
}
```

> Note that `x` and `[x]` are the same thing

> To be exaustive, must have a catch all


### Enum

Similarly to union patterns, covers variants of a enum, but the capture is done within `( )` after the variant name. The variant can be identified as `Type.variant` or just `.variant` since we know the type.

```
match (.succ(15) $$ Result(Int; Float)) {
    .succ(o) => ...,
    .fail(0.5) => ...,
    .fail(_) => ...,
}
```

To be exaustive must exaust every variant 

### Compound

Each field is pattern matched on it's own

### Structure

Same as compound but the name given to the bind matters

### Tag

Behaves the almost the same as union pattern, but must have a catch all to be exaustive