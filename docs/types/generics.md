# Generics

Types may have a hole to be filled later, so they can operate on a wider range of types at once. Generic types names begin with `$` and are `PascalCase` (more often only a single char) and the names are bound to a scope. I means `$T` meaning depends on the context. Generics are always bound to a tag (`#any` if no tag is specified). Here is how to specify a generic and its tag in different scenarios.

## Function

If the generics used in a function need to be bound to a tag, this must be done inside the arguments, before a `;`

```rs
fn foo($T #tag1, $U #tag2; bar $T) -> $U {
    ...
}
```

## Call

When calling a function with generics they are concretized using the types of the arguments. If the output type is generic and cannot bet concretized by the inputs, then a bind or the `$>` operator can be used to provide a concrete definition. If the types still can be bound by inference, define them before the `;` within the arguments.

## Types

In types, the generics in the definition are set using `( )` before the `=` sign.

``` rs
type Foo($T #tag1, $U #tag2) = ... { 
    fn bar() -> $U { ... }
    fn self baz() -> $T { ... }
}
```

When using methods from the type, the generics may not need to be concretized if the method uses `self` or if the types are used in the arguments of the method. But if needed, the types can be concretized usng the `( )`.

```rs
Foo(Int, Float):bar() // returns Float
foo Foo(String, Bool) = ...
foo:baz() // returns String
```
