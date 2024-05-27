# Functions

In `Aura` (as in any other programming language) is a way of encapsulate code to reproduce some behaviour and/or operation.

In `Aura` there are two kinds of functions: `Func` and `Act`. An `Act` can do whatever a `Func` can and: modify data outside of the function and execute IO actions (files, sockets, etc).

```ts
// A regular Func
// Func<(Float, Float, Float), ?(Float, Float)> and (Float, Float, Float) -> ?(Float, Float) are the same
decl static baskhara: Func<(Float, Float, Float), ?(Float, Float)> = (a: Float, b: Float, c: Float) -> ?(Float, Float) {
    decl delta = b^2 - 4*a*c;
    if (delta < 0) => null,
        else => {
            decl x1 = (-b + delta^0.5) / 2*a;
            decl x2 = (-b - delta^0.5) / 2*a;
            (x1, x2)
        }
}

// An Act
// Act<(), ()> is the same as act () -> ()
decl static run: Act<(), ()> = act () -> () {
    print("Inform the coefficients (a, b, c): ");
    decl a = scan():parse<Float>()!;
    decl b = scan():parse<Float>()!;
    decl c = scan():parse<Float>()!;
    decl res: ?(Float, Float) = baskhara(a, b, c);

    when (res)
        null => println("No real solution"),
        (x1, x2) where x1 == x2 => println(`The solution is ${x1}`),
        (x1, x2) => println(`The solutions are ${x1} and ${x2}`);
}
```

## Closures

Closures are functions that can capture variables in the scope where they've been created (so they aren't always pure) and they use a much shorter notation. If a closure is pure, you may prepend it with the `pure` keyword so its type will be `I -> O` rather than the default `act I -> O`.

They are most times used as arguments to functions, so most part of the notation can be infered.

```ts
args >-> expr
```

Where args is a named compound (types can be ommited) and the return type is infered from the expr

```ts
my_array:filter x >-> (x > 9 && x % 2 == 0)
```
