# Modules and Libraries

An Aura module is defined within a file. The file name should be `kebab-case` and this becomes the module name.

TODO: describe visibility

The submodules of a given module can be defined as files under a folder with the same name as the module.

When importing modules we use Unix standards for paths and we can alias/destructure imported modules:

```rs
import ./foo // child module foo as @foo
import ../baz // sibling module baz as @baz
import @foo2 = ../baz/foo // as the names conflict, we rename it to @foo2
import (kee, Daz) = ./bar // we import kee and Daz from bar as is

type T = (@foo:Foo, @baz:Baz, Daz)
```