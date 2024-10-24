# Macro

Macros are function-like definitions but they produce code rather than produce values. They are useful mainly for creating syntax that changes the way the code execution is flowing.

## Metatypes

While functions use regular types, macros use metatypes that describe the type of the code it receives, rather than the type of the value produced by the given piece of code

- `Expr`: any kind of evaluatable expression
- `Pat`: a pattern for a match expression
- `Ty`: a type
- `Binop`: a binary operator
- `Preop`: a prefix unary operator
- `Postop`: a postfix unary operator
- `VIdent`: a value identifier
- `TyIdent`: a type identifier
- `TagIdent`: a tag identifier

## Built-in Macros

- `@union ty Ty -> Ty`
- `@enum ty Ty -> Ty`
- `@return expr Expr -> Expr`: Early exits a function body returning a value
- `@crash expr Expr -> Expr`: Crashes the program emiting the expression as the message
- `@fail expr Expr -> Expr`: Same as `@return fail(...)`
- `@ok expr Expr -> Expr`: Same as `@return ok(...)`
- `@break expr Expr -> Expr`: Short finishes the current looping scope returning `break(...)`
- `@continue expr Expr -> Expr`: Short finishes the current looping scope returning `continue(...)`
- `@typeof expr Expr -> Ty`: Get's the type of an expression
- `@loop f FnDecl -> FnDecl`: Involves the body of the function in a loop
- `@match f FnDecl -> FnDecl`: Creates a match function
- `@async f FnDecl -> FnDecl`: Creates an async function
- `@io f FnDecl -> FnDecl`: Creates a function with io capabilities
- `@fs f FnDecl -> FnDecl`: Creates a function with filesystem capabilities
- `@net f FnDecl -> FnDecl`: Creates a function with networking capabilities
- `@requires(c: Capability) f FnDecl -> FnDecl`: Creates a function that needs a given capability
- `@revoke(c: Capability) expr Expr -> Expr`: revokes a capability when evaluating an expression

## Declaring Macros

```rs
macro @name (param1 ty1, param2 ty2) input MetaType -> MetaType
```

- `@name`: macro identifier (`@[a-z]+([:a-z])*`)
- Value parameters
- Input: construct that will be transformed
- Output

## Calling Macros

The macro operates on the next thing in the code (identifier, declaration, expression, etc)

```rs
fn foo() -> I32 {
    @return 5;
}

@loop fn mid(initial I32, end I32) -> I32 {
    if (initial >= end) then {
        @break initial
    } else {
        @continue (initial + 1, end - 1)
    }
}

@io fn println(p: #printable) {
    ...
}
```
