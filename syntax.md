# Kaprino syntax

Written in extended [Backus Naur Form (BNF)](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).

## Syntax for the functions

```
<function>
    ::= <params> <args> "=" <expr> <functype>
    | "#func" <params> <args> <functype> "|>" (<statement>)* "|<"
<args>
    ::= "(" <params> ("," <params>)* ")"
<functype>
    ::= "(" <params> ("," <params>)* "->" <params> ")"
```

## Syntax for the statements

```
<statement>
    ::= "#let" <params> (":=" <expr>)? "(" "<-" <params> ")"
    | <params> ":=" <expr>
    | "#call" <expr>
    | "#if" <expr> "|>" (<statement>)* "|<"
    | "#loop" "|>" (<statement>)* "|<"
    | "#break"
    | "#ret" <expr>
```

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
