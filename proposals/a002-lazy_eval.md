# Proposal #A002: Lazy Evaluation

This proposal states for the need to add syntax to some kind of lazy evaluation of expressions

## Approach 1: Scoped Functions

A function declared that is bound to a scope by the scope tag. A bound scope cannot be used outside of the scope it's tied to. This kind of binding is useful for labeled control flow macros like: `return`, `break`, `next` etc

### Syntax

```rs

main -> {
    'outer:{
        f := 'outer:() -> { @return:'outer };
        
        // do stuff
        {
            f(); 
        }

        // unreachable
    } // returns from this scope
}

```