# Grammar (WIP)
## Guidelines
> Ambiguities should be resolved within the BNF declarations rather than other informal clarifications.

## Lexical Syntax

### Abstract Terminals
`<terminal> ::= <simple> | <name> `

#### Simple Primitives
``<simple> ::= <boolean> | <char> | <integer> | <float> | <string> ``
```ignore
<integer> ::= <_non-zero> <_dec-digit>* | "0" r/[bB]/ <_bin-digit>+ | "0" "0" <_oct-digit>+  | "0" r/[xX]/ <_hex-digit>+ ;
<float>   ::= <_dec-digit>+ "." <_dec-digit>+ ( r/[eE][-+]/ <_dec-digit>+ )?;
<boolean> ::= "true" | "false" ;
<char>    ::= r/'/ <_char> r/'/ ;
<string>  ::= r/"/ <_char>* r/"/ ;
```
```ignore
<pattern>      ::= <variant> ;
<regex>        ::= "r/" ( r/[^/]/ "\/" )* "/" ;
<pattern-item> ::= <regex> | <char> | <string> ;
<variant>      ::= <concat> ( "|" <variant> )?
<concat>       ::= <pattern-item> <concat>? ;
```

#### Identifiers
```ignore
<name-lower> ::= <_lower> <_alnum>* ;
<name-upper> ::= <_upper> <_alnum>* ;
<name> ::= <name-lower> | <name-upper>
```

### Expr
``<expr> ::= <or-ops> ;``

#### Control
```ignore
<let>
<if-t-e>
```
#### Logical
```ignore
<or-ops>  ::= "||" ;
<or>      ::= <or> <or-ops> <and> ;
<and-ops> ::= "&&" ;
<and>     ::= <and> <and-ops> <eq> ;
```

#### Comparative
```ignore
<eq-ops>   ::= "==" | "!=" ;
<eq>       ::= <eq> <eq-ops> <comp> ;
<comp-ops> ::= "<" | "<=" | ">" | ">=" ;
<comp>     ::= <comp> <comp-ops> <term> ;
```

#### Arithmetic
```ignore
<term-op> ::= "+" | "-" ;
<term>    ::= <term> <term-op> <factor> ;
<fact-op> ::= "*" | "/" | "%" ;
<factor>  ::= <factor> <term-op> <cons> ;
```

#### Other Infix Operations
```ignore
<cons> ::= <unary> "::" <cons> | <unary> ;
```

#### Unary
```ignore
<unary-ops>   ::= "-" | "!" ;
<unary>       ::= <unary-ops> <unary> | <apply> ;
```

#### Application/Membership
```ignore
<apply>  ::= <access> ( " " <atom> )*;
<access> ::= <access> "." <name> | <atom> ;
```

#### Atomic
```ignore
<tuple> ::= "(" ( <expr> "," )* <expr>? ")" ;
<list>  ::= "[" ( <expr> "," )* <expr>? "]" ;
<atom> ::= <terminal> | <tuple> | <list> | "(" <expr> ")" ;
```

### Declaration

### Util Definitions

#### Numeric
```ignore
<_non-zero>  ::= r/[1-9]/ ;
<_dec-digit> ::= r/[0-9]/ ; 
<_bin-digit> ::= r/[01]/ ; 
<_oct-digit> ::= r/[0-7]/ ; 
<_hex-digit> ::= r/[0..9a-fA-F]/ ; 
```

#### Character
```ignore
<_lower>  ::= r/[_a-z]/ ;
<_upper>  ::= r/[_A-Z]/ ;
<_alpha>  ::= <_lower> | <_upper> ;
<_alnum>  ::= <_alpha> | <_dec-digit> ;
<_escape> ::= r/[abefnrtv\'"?]/ | <_oct-digit>{3} | "x" <_hex-digit>{2} | "u" <_hex-digit>{4} | "U" <_hex-digit>{8} ;
<_char>   ::= r/[^\"]/ | "\" <_escape> ;
```

## Ignored

### Whitespaces
``<_whitespace> ::= " " | "\t" | "\r" | "\n" ;``

### Comments
``<_comment> ::= "\\" r/.*/ "\n";``

## Syntax Legend
Basic declarations follow the [Backus-Naur Form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form#Overview) with whitespaces ignored wherever they are not part of any token and the following extended syntax,

| Form                             | Definition                                                                                                          |
| -------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| "`<_SYMBOL>`"                    | Helper Symbol (spaces between which are _not_ ignored when concatenated with terminal strings or other `_` symbols) |
| `(<bnf expr>)`                   | Grouping                                                                                                            |
| `r/<regex>/`                     | Regular Expression (_raw_ string pattern)                                                                           |
| `<bnf expr>{<number>, <number>}` | Repetition (range _inclusive_, open if one `<number>` omitted)                                                      |
| `<bnf expr>{<number>}`           | Repetition (exact _# of times_)                                                                                     |
| `<bnf expr>*`                    | `<bnf expr>{0,}`                                                                                                    |
| `<bnf expr>+`                    | `<bnf expr>{1,}`                                                                                                    |
| `<bnf expr>?`                    | `<bnf expr>{0,1}`                                                                                                   |