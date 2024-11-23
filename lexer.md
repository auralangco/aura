# Aura Lexer Reference

This document is the formal lexer reference for the Aura programming language

## Keywords

Aura is meant to have the least keywords possible

- `KW_FN := fn`
- `KW_TAG := tag`
- `KW_VAL := val`
- `KW_MAIN := main`
- `KW_TYPE := type`
- `KW_MACRO := macro`
- `KW_IMPORT := import`
- `KW_OBJECT := object`

Those are used as top level declarations in a file and only there

## Identifiers

Aura has really restrictive naming rules

- value identifiers are `snake_case`
- type identifiers are `PascalCase`
- tag identifiers are `#kebab-case`
- macro identifiers are `@kebab:case`
- subtypes identifiers are prefixed with `$` and may contain only letters and numbers

Formally:

- `IDENT_VAL := "[a-z][a-z0-9_]+"`
- `IDENT_TY := "[A-Z][a-zA-Z0-9]+"`
- `IDENT_TAG := "#[a-z][a-z0-9]*(-[a-z0-9])*"`
- `IDENT_MACRO := "@[a-z][a-z0-9:]*"`
- `IDENT_STY := "\$[a-zA-Z][a-zA-Z0-9]+"`

## Operators

- `OP_DECL := ":="`
- `OP_EQ := "="`
- `OP_PLUS := "+"`
- `OP_MINUS := "-"`
- `OP_STAR := "*"`
- `OP_SLASH := "/"`
- `OP_CARET := "\^"`
- `OP_USCORE := "_"`
- `OP_PERCENT := "%"`
- `OP_AND := "&"`
- `OP_ANDAND := "&&"`
- `OP_OR := "|"`
- `OP_OROR := "||"`
- `OP_NOT := "!"`
- `OP_NOTEQ := "!="`
- `OP_EQEQ := "=="`
- `OP_GT := ">"`
- `OP_GTEQ := ">="`
- `OP_LT := "<"`
- `OP_LTEQ := "<="`
- `OP_LSHF := "<<"`
- `OP_RSHF := ">>"`
- `OP_RARW := "->"`
- `OP_FAT_RARW := "=>"`
- `OP_TILDE := "~"`
- `OP_JOIN := "::"`
- `OP_BSLASH := "\\"`
- `OP_RANGE := "\.\."`
- `OP_CRANGE := "\.\.="`
- `OP_SPREAD := "\.\.\."`

## Delimiters

There are only 4 delimiters is Aura syntax:

- `( )` parameters delimiter
- `[ ]` listing delimiter
- `{ }` scope delimiter

Namely:

- `DELIM_OPAREN := "("`
- `DELIM_CPAREN := ")"`
- `DELIM_OBRACK := "["`
- `DELIM_CBRACK := "]"`
- `DELIM_OBRACE := "{"`
- `DELIM_CBRACE := "}"`

It's likely that several tokens are between each delimiter but they always have to match at some point

## Literals

- Integer
    - `LIT_INT_TY_SUFFIX := "(U|I)(8|16|32|64)"`
    - `LIT_INT_DEC := "(0|[1-9][0-9_]*)<LIT_INT_TY_SUFFIX>?"`
    - `LIT_INT_BIN := "0b[01][01_]*<LIT_INT_TY_SUFFIX>?"`
    - `LIT_INT_OCT := "0o[0-7]0-7_]*<LIT_INT_TY_SUFFIX>?"`
    - `LIT_INT_HEX := "0x[0-9A-Fa-f][0-9A-Fa-f_]*<LIT_INT_TY_SUFFIX>?"`
- Float `LIT_FLT := "(0|[1-9][0-9_]*)\.(0|[1-9][0-9_]*)(F(32|64))?"`
- Char ``
- String
- Atom `LIT_ATOM := "'[a-z][a-z0-9]*(-[a-z0-9]+)*"`

## Punctuation

- `PT_DOT := "."`
- `PT_COMMA := ","`
- `PT_COLON := ":"`
- `PT_SCOLON := ";"`