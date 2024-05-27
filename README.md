# Aura

Aura is a functional parallelizable multitarget programming language. It means it's meant to be functional first, be easy to run in multicore and in multiple platforms by transpiling to other languages.

## Features

### Immutability

There are no variables in Aura, you may give aliases to values, and that's it.

### Lazyness

Computations are not done until it's absolutely required. This may increase memory usage, but gives Aura superpowers.

### Functional

Aura implements several functional programming features such as:

- Functions as values
- Purity and side-effect management
- Pattern matching

### Type Safe

Aura uses a dynamic type system that ensures operations validity is checked during compile time

### Static vs Dynamic*

Aura atempts to make several compile time operations available in runtime while enforcing proper compile time treatment