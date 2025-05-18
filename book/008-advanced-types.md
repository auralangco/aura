# Advanced Types

## Type Aliases

Type aliases are a way to give a name to an existing type. They are declared using the `type` keyword.

```rs
type MyType Int
```

## Tuples

Tuples are a way to group together a fixed number of values of different types. They are simply an alias for a list of types.

```rs
type MyTuple (Int, String)

decl my_tuple MyTuple := (1, "Hello")
```

## Structs

Structs are similar to tuples, but each field is named.

```rs
type Person (name String, age Int)

decl var1 Person := Person("John", 20)
decl var2 Person := (age = 20, name = "John")
```

## Unions

Unions are a way to combine multiple types into a single type. They are declared using the `@union` macro on a tuple.

```rs
type Number @union (Int, Float)

decl var1 Number := 1
decl var2 Number := 1.0
```

## Enums

Enums are a way to define a type with different variants. Think of them as a kind of union, but with named variants. You can create enums with the `@enum` macro on a struct.

```rs
type NumberOrString @enum (number Int, text String)

decl var1 NumberOrString := NumberOrString.number(1)
decl var2 NumberOrString := NumberOrString.text("Hello")
```

## Methods

The declaration of a method is similar to a function, but it is bound to a type. A method can be declared everywhere, but if declared in the same module as the type it is bound to, it will be imported automatically.

```rs
type Car := (brand String, model String, year Int)

func Car:drive(self) -> {
    println("Driving " ++ self.brand ++ " " ++ self.model)
}
```

### Mutable Methods

Methods can be declared as mutable by using the `@mut` macro before `self`.

```rs
func Car:get_older(@mut self) -> {
    self.year += 1
}
```

### Static Methods

Static methods are methods that are bound to the type itself, rather than an instance of the type. They are declared by not using `self` as an argument.

```rs
func Car:new(brand String, model String, year Int) -> Car {
    Car(brand, model, year)
}
```

## Inheritance

There's no inheritance in Aura, but you can achieve similar behavior using composition and interfaces.

## Interfaces

Interfaces are a way to define a contract for a type. They are declared using the `@interface` macro on a struct.

```rs
type CarLike @interface (
    drive (self) -> Void,
    get_older (self) -> Void
)
```

The implementation of an interface is done by simply declaring the methods in the type that implements the interface. There's no explicit declaration of the implementation

```rs
type Car (brand String, model String, year Int) {
    func drive(self) -> {
        println("Driving " ++ self.brand ++ " " ++ self.model)
    }

    func get_older(@mut self) -> {
        self.year += 1
    }
}

decl cl CarLike := Car("Toyota", "Corolla", 2020)
```

