# Branching Structures

`Aura` provides a very simple branching structure that follows a simple syntax:

```ts

pattern => code, 
pattern => code,
...

```

Each branch is separated by a comma, and a branch has two parts: the pattern and the code.

The `$` (eval branch) operator evaluates the structure. It's a binary operator that receives a struture and an input value, then checks returns the `code` associated with the first matching `pattern` as a closure.

There's also the `$?` operator which returns `null` if the default branch matches.

A pattern can be: a value, an identifier, a compound pattern, a list pattern, an object pattern, an union field pattern. A pattern may or may not fails.

The last branch can be a default branch by using `_`, `else`, `otherwise`, `default` or a never failing pattern as its pattern, they always match, the four first options don't capture a value to the code.

Every `code` must return the same type. If the branching struture returns `Void` then a default pattern is auto inserted as `_ => ()`.

## Branching Type

`I => O` can be undestood as the type signature of a branching structure. Where `I` is the type of the input value and `O` is the type returned by the branches. The `$` operator signature then can be understood as `(I, I => O) -> (() -> O)`.

## Applications of the Branching Structure

### If

The `if` is a function of type `(Bool => T) -> T`, and it's implementation is:

```ts
decl static if = <T>(bs: Bool => T) -> T {
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
decl static when = <T, U>(bs: T => U, value: T) -> U {
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

The `while` function is quite different, the output value of the branching structure it receives must be `Control<Void>`.

The `Control<T>` is an union with variants `Next: Void` and `Break: T`. The key words `next` and `break x` are a shortcut for `retn Control.Next` and `retn Control.Break x`

As you may predict, using multiple branches means that each iteration may execute a different code. And the default branch only matches if it's the first iteration.

`while` is a statement, so no value can be returned.

```ts
decl static while = <T>(bs: Bool => Control<Void>) {
    decl var first_iter = true;

    loop {
        if first_iter => {
            first_iter = false; 
            (bs $ true)()
        }, 
        else => (bs $? true)
            :map(() |>)
            :unwrap_or(Control.Break())
    }
}

test "Simple While" {
    decl var iter = 0;

    while iter < 10 => {
        println(`It's the iteration ${iter}`);
        iter += 1;
        next
    }
}

test "Default Branched While" {
    decl var num = 11;
    decl var result = Vec:new();

    while num < 5 => {
        result += "Adding";
        next;
    }, else => {
        result += "Failed";
        break;
    };

    assert(result == ["Failed"]);

    num = 0;
    result = Vec:new();

    while num < 5 => {
        result += "Adding";
        next;
    }, else => {
        result += "Failed";
        break;
    };

    assert(result == ["Adding", "Adding", "Adding", "Adding", "Adding"]);
}

test "Branch Changing While" {
    decl State = union {A, B, C, D};
    decl var s = State.A;

    // prints ACB
    while s <> State.A => {
        print("A");
        s = State.C;
        next;
    }, s <> State.B => {
        println("B");
        s = State.D;
        next;
    }, s <> State.C => {
        print("C");
        s = State.B;
        next;
    };
}
```

In the non-default branches a `next` is auto inserted at the end. In the default branch a `break` is auto inserted.
