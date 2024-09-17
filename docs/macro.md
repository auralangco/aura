# Macro

Macros are function-like definitions but they produce code rather than produce values. They are useful mainly for creating syntax that changes the way the code execution is flowing.

## Metatypes

While functions use regular types, macros use metatypes that describe the type of the code it receives, rather than the type of the value produced by the given piece of code

- `@expr`: any kind of evaluatable expression
- `@pat`: a pattern for a match expression
- `@ty`: a type
- `@binop`: a binary operator
- `@preop`: a prefix unary operator
- `@postop`: a postfix unary operator
- `@vident`: a value identifier
- `@tyident`: a type identifier
- `@tagident`: a tag identifier
- `@never`: the meta version of the never type

## Built-in Macros

### `return!(e $expr = ()) ~> $never`

Early exits a function body returning a value

### `typeof!(e $expr) ~> $ty`

Get's the type of an expression

### `if!(then $expr, else $expr = NaV) -> $expr

## Calling Macros

Same syntax as for calling functions