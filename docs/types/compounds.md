# Compounds: Product Types

Compounds are a form of compose types in a way that data of a compound type must have a value for each of its composing types.

The composition of `T1`, `T2`, ..., `Tn` is denoted by `(T1, T2, ..., Tn)`. The order matters, so each part is indexed by an integer literal.

## Singles

Compounds of a single type.

There is equivalence between `T` and `(T)`, they are auto castable.
