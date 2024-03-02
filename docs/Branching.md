# Branching Structures

`Aura` provides a very simple branching structure that follows a simple syntax:

```ts

pattern => code, 
pattern => code,
...

```

Each branch is separated by a comma, and a branch has two parts: the pattern and the code.

The `$` (eval branch) operator evaluates the structure. It's a binary operator that receives a struture and an input value, then checks returns the `code` associated with the first matching `pattern` as a closure.

A pattern can be: a value, an identifier, a compound pattern, a list pattern, an object pattern, an union field pattern.

Usually, the last branch's pattern is `_`, `else`, `otherwise` or `default`, they always match, but never captures a value to the code.

Every `code` must return the same type. If the branching struture returns `Void` then no `else` pattern is needed, otherwise, it's mandatory.

## Branching Type

`I => O` can be undestood as the type signature of a branching structure. Where `I` is the type of the input value and `O` is the type returned by the branches. The `$` operator signature then can be understood as `(I, I => O) -> (() -> O)`.

## Applications of the Branching Structure

### If

The `if` is a function of type `(Bool => T) -> T`, and it's implementation is:

```ts
decl if = <T>(bs: Bool => T) -> T {
    (bs $ true)()
}

test "If Testing" {
    decl age = 59;

    decl category = if  
        age < 18 => "Underage",
        age < 60 => "Adult",
        else => "Senior";

    assert(category == "Adult")!;
}
```

### When

The `when` function is a more general `if` that lets you match using things other than `true`

```ts
decl when = <T, U>(bs: T => U, value: T) -> U {
    (bs $ value)()
}

test "When Testing" {
    decl smth = (5.5, false, "City");

    decl result = when (smth) 
        (_, true, _) => "It's truth",
        (num, false, _) => `Number ${num}`,
        (_, _, "City") => "We got City";
    
    assert(result == "Number 5.5")!;
}
```

### While

The `while` function is quite different, the output value of the branching structure it receives must be `Control<T>`.

The `Control<T>` is an union with variants `Next: Void` and `Break: T`. The key words `next` and `break x` are a shortcut for `retn Control.Next` and `retn Control.Break x`

As you may predict, using multiple branches means that each iteration may execute a different code.

`while` can be used as an expression, which value will be the value wrapped with the `Control.Break: T`

```ts
decl while = <T>(bs: Bool => Control<T>) -> T {
    loop (bs $ true)() // Loops until a Control.Break is returned by the struture
}

test "Simple While" {
    decl var iter = 0;

    while iter < 10 => {
        println(`It's the iteration ${iter}`);
        iter += 1;
    }
}
```

In `while` it's not recommended to have an `else` branch. If you do have make sure it has a break.

In the non-default branches a `next` is auto inserted at the end. In the default branch a `break` is auto inserted only if `T` in `Control<T>` has the `#default` tag. Also, the default branch executes as the last iteration.
