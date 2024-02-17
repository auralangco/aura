# Declaration

Declaring means atribute data to names. In Aura, almost anything can be assigned to a name by a declaration.

The declaration statement follows the following structure:

```ts
decl (var|final|static)? <ident>(: <type>)? (= | := | => ) <expr>;
```

- `decl`: the declaration keyword
- `var`: the declaration is variable, it means, it's data can be mutated
- `final`: the declaration can't be redeclared
- `static`: the declaration data is known during compile time
- `<ident>`: the name that's being declared. The name must follow a pattern to indicate what is being declared.

    - *value*: `snake_case` is used, unless it's `final`, then `SCREAMING_SNAKE_CASE` should be used.
    - *type*: `PascalCase` is used
    - *tag*: `#kebab-case` is used

- `<type>`: Specifies the type of the value being declared. Only used for values declarations, and can be ommited
- `=`: copy assign operator copies the expression assigning it to the declared name;
- `:=`: deep copy assign operator deep copies the expression, so any internal references are also copied. Only used for values declarations
- `=>`: pointing operator is used to declare a reference to some value
- `<expr>`: the data that's being assigned to the declared name.

## Values

```ts
// A constant of Int type and value 19
decl age: Int = 19;
// A compile time known constant type String and value "John Doe"
decl static name: String = "John Doe";

// A compile time known function that executes an action: printing to the console
decl static greet: Act<(String), Void> = act (name: String) println(`Hello ${name}!`); 
```

Values are the most common data: text, numbers, logic values, objects, functions, etc.

## Types

```ts
// Creating an alias to the String type
decl MyString = String;
// Creating a new type that is an object with writable Float x and y fields
decl Point = object {write x: Float, write y: Float};
// Creates an alias to the compound (Float, Float) type
decl Point2 = (Float, Float);
```

Types are descritors to values, they are used to group them by their common properties and operations.

## Tags

They are like types for types, but less restrictive. Are used to mark that types can do some operations
