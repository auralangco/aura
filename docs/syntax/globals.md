# Global Scope Syntax

## Module

A module is a file and there are two kinds of modules: executable and library. The structure of a module is divided in sections and it's quite straight forward: imports, types and values, main, functions.

### Imports

This first section is where the `use` and `link` keywords are used both to import stuff from other modules or link to external functions (using a Foreign Function Interface)

#### `use`

`~` means: the current project root. If not used, then the first part of the path is the name of the library

```bnf
<use> ::=
    use (<moddestruct> =)? ~(/<ipath>)*/<imod>
    | use (<moddestruct> =)? <ipath>(/<ipath>)*/<imod>
```

#### `link`
