# Implementation

Implementation is how we define the methods for a given type. The implementation can only be defined in the `type` declaration. New methods can be added using tags.

```rs
type Car = (name String, value Int, color String) {
    fn self drive() = IO::println("Driving ${name}");

    fn self fix() -> Self = (self.name, self.value + 500, self.color);

    fn new(name String, value Int) -> Self = (name, value, "Black");
}
```

A simple `fn` is a function that is not bound to an instance of this object. But a `fn self` is bound to an instance. It means one can call `Car:new(...)` and get a car. But calling `Car:fix()` produces a function `(car Car) -> Car` rather than a `Car`. Notice that `Self` is an alias to the type being implemented.
