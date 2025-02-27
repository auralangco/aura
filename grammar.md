# Aura Grammar

## Keywords

- `xtrn`
- `decl`
- `final`

## Identifiers

- **value_identifier**: `[a-z][a-z0-9_]*`
- **type_identifier**: `[A-Z][a-zA-Z0-9]*`
- **macro_identifier**: `@[a-z][a-z0-9_]*`

## Literals

- **integer**: `(-|+)?(0|[1-9][0-9]*)`
- **sized_integer**: `(<integer>)((I|U)(8|16|32|64))?`
- **float**: `(<integer>).(<integer>)`
- **raw_char**: `([^'\]|\\(n|r|t|u[0-9]{4}))`
- **char**: `'(<raw_char>)'`
- **string** `"(<raw_char>)*"`

## Operators

- `=` mutate
- `:=` declare
- `+` add
- `-` minus
- `*` star
- `/` divide
- `%` modulus
- `**` power
- `+=` bind-add
- `-=` bind-minus
- `*=` bind-star
- `/=` bind-divide
- `%=` bind-modules
- `**=` bind-power
- `&&` and
- `&` bitwise and
- `||` or
- `|` bitwise or
- `!` not
- `` ` `` bitwise not
- `==` equals
- `!=` not equals
- `>` greater than
- `<` less than
- `>=` greater or equals to
- `<=` less or equals to
- `++` concatenate
- `++=` bind concatenate
- `_` curry/don't care
- `::` join
- `::=` bind join
- `\\` split
- `\\=` bind split
- `.` property
- `.=` bind property
- `:` associated
- `...` spread
- `->` function arrow
- `~>` macro arrow
- `=>` branch arrow
- `~` guard
- `|>` pipe
- `??` unwrap
- `!?` hard unwrap
- `=?` unwrap or
- `.?` safe property
- `:?` safe associated

## Brackets

- `(` `)` compound, parameters
- `{` `}` bodies
- `[` `]` lists

## Type Expressions

- **simple**: `<type_identifier>`
- **parametric**: `<type_identifier>((<type_expression>)(,(<type_expression>))*,?)`
- **unit**: `()`
- **compound**: `((<type_expression>)(,(<type_expression>))*,?)`
- **struct**: `struct ((<value_identifier> <type_expression>)(,(<value_identifier> <type_expression>))*,?)`
- **union**: `union ((<type_expression>)(,(<type_expression>))*,?)`
- **enum**: `enum ((<value_identifier> <type_expression>?)(,(<value_identifier> <type_expression>?))*,?)`
- **function**: `((<value_identifier> <type_expression>)(,(<value_identifier> <type_expression>))*,?) -> <type_expression>`
- **branch**: `<type_expression> => <type_expression>`

## Value Expressions

