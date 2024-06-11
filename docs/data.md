# Aura: Data

The way Aura treats data and memory depends on the target, but doesn't affect the result of the computation.

## Targets

### C

When targeting C there are three possibilities:

- Data is used only once, so it's moved
- Data is shared by reference when passed as arguments to calls
- Data is copied when passed as argument to data constructors

### JS

WIP

### HVM

WIP

## Immutability

The important thing to notice is: data is always immutable. So everytime you pass data to somewhere else, the original data is preserved.