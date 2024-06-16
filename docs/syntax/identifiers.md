# Identifiers

Identifiers are the name given to stuff in a program. Aura uses different identifiers for different purposes

## `snake_case`

This one is used for local binded values, functions in general, structure fields

```bnf
<snake_case> ::= [a-z][a-z0-9_]+
```

## `SCREAMING_SNAKE_CASE`

This is used with the `val` declarations for constant values

```bnf
<screaming_snake_case> ::= [A-Z][A-Z0-9_]+
```

## `PascalCase`

This is used for types

```bnf
<pascal_case> ::= ([A-Z][a-z0-9]*)+
```

## `kebab-case`

This is used for atoms and tags, do not allow numbers

```bnf
<kebab_case> ::= [a-z]+(-[a-z]+)*
```

## C Case

The casing used by C identfiers

```
<c_case> ::= [a-zA-Z_][a-zA-Z0-9_]*
```

## Applied Casings

```
<ibind>     ::= <snake_case>
<ifn>       ::= <snake_case>
<itype>     ::= <pascal_case>
<ival>      ::= <screaming_snake_case>
<itag>      ::= <kebab_case>
<iatom>     ::= @<kebab_case>
<ipath>     ::= <kebab_case>
<imod>      ::= <pascal_case>
<ilink>     ::= <c_case>
```