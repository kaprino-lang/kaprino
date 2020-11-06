# Kaprino syntax

Written in extended [Backus Naur Form (BNF)](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).

## Syntax for the expressions

```
<expr>
    ::= <term> (<addop> <term>)*
<addop>
    ::= ("+" | "-" | "and" | "or" | "xor")
<term>
    ::= <exponents> (<mulop> <exponents>)*
<mulop>
    ::= ("*" | "/" | "%")
<exponents>
    ::= <factor> [<expop> <exponents>]
<expop>
    ::= "^"
<factor>
    ::= <numbers>
    | <params>
    | "(" <expr> ")"
    | "not" <factor>
```
