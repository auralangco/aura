# Macro

Macros are function-like definitions but they produce code rather than produce values. They are useful mainly for creating syntax that changes the way the code execution is flowing.

## Metatypes

While functions use regular types, macros use metatypes that describe the type of the code it receives, rather than the type of the value produced by the given piece of code. Macros can operate on:

- expressions
- types
- declarations
- scopes

## Built-in Macros

- `@union ty Ty -> Ty` creates an union type from a compound
- `@enum ty Ty -> Ty` creates an enum type from a structure
- `@return expr Expr -> Expr`: Early exits a function body returning a value
- `@return:(a Atom) expr Expr -> Expr`: Early exits a scope by its label returning a value
- `@crash expr Expr -> Expr`: Crashes the program emiting the expression as the message
- `@fail expr Expr -> Expr`: Same as `@return fail(...)`
- `@ok expr Expr -> Expr`: Same as `@return ok(...)`
- `@fail:(a Atom) expr Expr -> Expr`: Same as `@return:(a) fail(...)`
- `@ok:(a Atom) expr Expr -> Expr`: Same as `@return:(a) ok(...)`
- `@break expr Expr -> Expr`: Short finishes the current looping scope returning `break(...)`
- `@continue expr Expr -> Expr`: Short finishes the current looping scope returning `continue(...)`
- `@typeof expr Expr -> Ty`: Get's the type of an expression
- `@loop f FnDecl -> FnDecl`: Involves the body of the function in a loop
- `@loop s Scope -> Scope`: Marks the scope as a looping scope for `@break` and `@continue`
- `@match f FnDecl -> FnDecl`: Creates a match function
- `@async f FnDecl -> FnDecl`: Creates an async function
- `@async s Scope -> Scope`: Marks the scope as a async scope
- `@sync f FnDecl -> FnDecl`: Creates a sync function
- `@sync s Scope -> Scope`: Marks the scope as a sync scope
- `@io f FnDecl -> FnDecl`: Creates a function that needs io capabilities
- `@fs f FnDecl -> FnDecl`: Creates a function that needs filesystem capabilities
- `@net f FnDecl -> FnDecl`: Creates a function that needs networking capabilities
- `@requires:(a Atom) f FnDecl -> FnDecl`: Creates a function that needs a given capability
- `@revoke:(a Atom) expr Expr -> Expr`: revokes a capability when evaluating an expression
- `@label:(a Atom) s Scope -> Scope`: Marks the scope with the given label
- `@context:(a Atom) s Scope -> Scope`: Marks the scope with the given context

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
type Color = @enum (
    red,
    green,
    blue
)

fn foo() -> I32 {
    @return 5;
}

/// A tail recursive function
@loop fn mid(initial I32, end I32) -> I32 {
    if (initial >= end) then {
        @break initial
    } else {
        @continue (initial + 1, end - 1)
    }
}

/// A function that needs the $io capability
@io fn println(p: #printable) {
    // ...
}

/// Example of a macro for a backend framework
@route:get:("/") fn root() -> Void {
    // ...
} 
```
