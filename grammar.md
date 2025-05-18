# Aura Grammar

## Keywords

- `type`
- `func`
- `decl`	

## Identifiers

- **value_identifier**: `[a-z][a-z0-9_]*`
- **type_identifier**: `[A-Z][a-zA-Z0-9]*`
- **macro_identifier**: `@[a-z][a-z0-9_]*`

## Literals

- **natural**: `(0|[1-9][0-9]*)`
- **integer**: `(-|+)?<natural>`
- **sized_integer**: `<integer>((I|U)(8|16|32|64))?`
- **float**: `<integer>?.<natural>?`
- **raw_char**: `([^'\]|\\(n|r|t|u[0-9]{4}))`
- **char**: `'<raw_char>'`
- **string** `"<raw_char>*"`

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
- **parametric**: `\[<type_expression>(,<type_expression>)*,?\]<type_expression>`
- **unit**: `\(\)`
- **tuple**: `\(<type_expression>(,<type_expression>)*\)`
- **struct**: `\((<value_identifier> <type_expression>)(,(<value_identifier> <type_expression>))*,?\)`
- **function**: `(<tuple>|<struct>) -> <type_expression>`
- **associated_type**: `<type_expression>:<type_identifier>`

## Value Expressions

- **variable**: `<value_identifier>`
- **literal**: `<integer>|<float>|<char>|<string>`
- **call**: `(<value_identifier>|<type_expression>|<associated_value>|<field>)(<value_expression>(,<value_expression>)*)`
- **index**: `<value_expression>\[<value_expression>\]`
- **slice**: `<value_expression>[<range>]`
- **field**: `(<value_expression>|<type_expression>).<value_identifier>`
- **associated_value**: `<value_expression>:<value_identifier>`
- **range**: `<natural>?..<natural>?`
- **compound**:
