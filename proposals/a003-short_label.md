# Proposal #A003: Short Hand Syntax For Labelling

Labels are a special syntax that uses atoms to give names for parts of the code that are useful for control flow macros

The current syntax is heavy by using the `@label:('some-atom)` macro (or `@label:'some-atom` considering `#A004`). A shorter syntax should be provided to label scopes and functions

## Syntax

```rs
'outer:{ // Labels this scope as 'outer
    // Binds the function to 'outer 
    f := 'outer:() -> 'inner:{ // Labels this scope as 'inner
        @return:('outer)
    };

    f();
}
```

