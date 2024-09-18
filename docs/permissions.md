# Permissions

Safety is a core part of Aura. The permission system has two steps:

1. Ask for the permission 
2. Given the permission

A function must specify the permissions it need in order to be executed. A function may pass its permissions to all functions it calls. The `main` entrypoint is the only place where a permission can be emitted.

A permission is a literal of form `/$[a-z][a-z.-]*[a-z]+/` (`kebab-case` prefixed with `$` that may contain some `.`). The `.` is the hierachy separator, the permission `$foo.bar` is conceeded once a function has `$foo`, but not the other way around.

A function lists its needed permissions in the function signature, before the `;` in the parameters, in the same place where generics are defined.

## Example

```rs
// Hello World

// We'll conceed all $io permissions
main ($io) {
    println("Hello World") // The println function signature is `fn println($io.write; msg String)`
} 
``` 

```rs
// cat clone
// We allow for all $io, but just $fs.read from $fs
main ($io, $fs.read; argc Int, argv List(String)) {
    File:open(argv[1])?! // The `File:open` function needs $fs.read in order to interact with the filesystem
    |> File:read_all() // `File:read_all` needs $io.read
    |> println; // Again `println` needs $io.write
}
```

```rs
// This function asks for $io permissions
fn ask($io; question String) -> String {
    print(question); // Needs $io.write
    readln()?! // Also needs $io.read
}

main ($io) {
    response = ask("What's your name? "); // This needs all $io
    println("Hello " + response); // This needs only $io.write
}
```