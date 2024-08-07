# Enumerations: Sum types

Enumerations are tagged unions where each variant has a name

```rs
type IOError = enum(
    not_found String, 
    permission_denied, 
    unknown U8)
```

Each variant has a name and an optional payload (defined by the type). If no type is provided, the default `Void` type is used.

When instantiating values of a enum use: `Type.variant`, `Type.variant(payload)`, `.variant`, `.variant(payload)`. `Type` can be ommited if it can be infered by the context. The payload can be ommited if the payload type is `Void`.

```rs
main {
    no_perm IOError = .permission_denied;
    open_file = IOError.not_found("/etc/foo/bar.txt");
}
```