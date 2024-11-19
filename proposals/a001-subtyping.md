# Proposal #A001: Subtyping

This proposal is meant to add syntax for deriving subtypes from a type that are structurally identical but different semantics

## Example

Measure units subtypes could be derived for numeric types to add semantics to what a number means

```rs
main -> {
    height Float$m := 1.8; // Non subtyped value can be subtyped
    weight Float$kg := 71.0; 

    weight = height; // ERROR: Float$m cannot be assigned to Float$kg
}
```

```rs
tag #from(Float$kg) = Float$g {
    fn from(value Float$kg) -> Float$g = value $$ Float * 1000
}
```

## Syntax

To derive a subtype just append: `$kebab-case` to it with no explicit declaration needed