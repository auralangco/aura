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

## Everything

Almost everything can be treated as data in Aura: literals, functions, types, etc. We categorize data as:

### Values

The lowest order data and the data you're used to handle in general languages. Numbers, text, booleans, functions, etc.

### Types

Types categorize values by ensuring they atend to certain properties. Values belongiing to a same type are structurally identical

### Tags

Tags categorize types by their behaviour. Types belonging to the same tag must implement a certain group of methods

### Modules

Modules group data that should have the same meaning

## Binding

Data can be bound to a name in lots of situations

### Global Scopes

`link`, `use`, `val`, `func`, `type`, `tag`, `main` bind some data to a name in the global scope as static data (known at compile time) that can't be redefined.

### Local Scopes

The `=` operator binds data to a name in local scopes (function bodies). This kind of binding can be redefined (only if the name doesn't conflict with a globally bound one).
