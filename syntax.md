# Syntax

## main

```rust
decl main = {
    println("Hello world");
}
```

## variables

```rust
decl main = {
    name := "Aura";

    println("Hello, {name}"); //> Hello, Aura
}
```

Use `@mut` to add mutability

```rust
decl main = {
    name := @mut "Aura";
    name[0] = 'a';

    println("Hello, {name}"); //> Hello, aura 
}
```

## global declarations

```rust
decl year = 2025
decl name = "Aura"

decl fmt = (name String, year Int) -> String {
    "{name} at {year}"
}

decl main = {
    println(fmt(name, year)); //> Aura at 2025
}
```

## functions
```rust
// Sum 2 integers
decl sum = (a Int, b Int) -> Int { a + b }

// Sum any a T and b U if T can be added to U
decl sum_any = [U, T Add[U]](a T, b U) -> T:Output { a + b }
```

## types

```rust
@derive:(Debug)
@derive:(ToString)
decl Category = @enum(sedan, hatch, pickup, sport)

@derive:(Debug)
decl Car = (
    name String, 
    year Int, 
    brand String, 
    category Category
)

decl Car:to_string = (self) -> String { "[{self.year}] {self.brand} {self.name} ({self.categoty})" }

decl main = {
    car Car := (
        name = "Enzo", 
        year = 2013, 
        brand = "Ferrari", 
        category = Category.sport
    );

    println(car);
}
```

## conditionals

```rust
decl Car:is_old = (self) -> Bool {
    self.year < 2000
}

decl main = {
    car Car := // ...
    if (car:is_old()) then {
        println("Old car");
    } else {
        println("New car");
    }
}
```

TODO: if as expression

## loops

loop

while

for

## values

### primitives

- `1234`: `I32`
- `12.34`: `F32`
- `"some text"`: `String`
- `' '`: `Char`
- `true`, `false`: `Bool`

### collections

- `[0, 1, 2]`: `Array[I32]` fixed keys
- `@list [0, 1, 2]`: `List[I32]` flexible size
- `[ "key" = "value" ]`: `Map[String, String]` fixed keys
- `@dict [ "key" = "value" ]`: `Dict[String, String]` flexible keys
- `@set [ 0, 1, 2, 3, 3 ]`: `Set[I32]` unique elements

### functions
