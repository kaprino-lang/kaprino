lexer grammar KaprinoLexer;

// --------------
//
// Lexer
//
// --------------

NEWLINE         : ('\r')? '\n' -> skip
                ;
COMMENT         : '#' .*? NEWLINE -> skip
                ;

//
// String
//
QUOTE           : '"' -> more, mode(TEXT_MODE)
                ;

//
// Reserved word
//
CONSIDER    : ('C' | 'c') 'onsider'
            ;
VARIABLE    : 'variable'
            ;
IF          : ('I' | 'i') 'f'
            ;
THEN        : 'then'
            ;
OTHERWISE   : ('O' | 'o') 'therwise'
            ;
WHILE       : ('W' | 'w') 'hile'
            ;
LOOP        : 'loop'
            ;
CLASS       : ('C' | 'c') 'lass'
            ;
DEFINE      : ('D' | 'd') 'efine'
            ;
RETURN      : ('R' | 'r') 'eturn'
            ;
AND         : 'and'
            ;
OR          : 'or'
            ;
NOT         : 'not'
            ;

//
// Number
//
fragment DIGIT  : [0-9]
                ;
NUMBER          : DIGIT+
                ;
REAL_NUMBER     : DIGIT+ '.' DIGIT+
                ;
//
// Reserved symbol
//
DOT     : '.'
        ;
CLOSER  : '|>'
        ;

//
// Boolean
//
TRUE    : 'True'
        ;
FALSE   : 'False'
        ;

//
// ID
//
fragment LETTER : [a-zA-Z]
                ;
ID              : LETTER (LETTER | DIGIT)*
                ;

//
// Operators
//
COMMA           : ','
                ;
INDEX_OP        : '_'
                ;
GRATEROREQUAL   : '>=' | '=>'
                ;
LESSOREQUAL     : '<=' | '=<'
                ;
GRATERTHAN      : '>'
                ;
LESSTHAN        : '<'
                ;
EQUAL           : '='
                ;
ASSIGN          : ':='
                ;
LEFT_BRACKET    : '('
                ;
RIGHT_BRACKET   : ')'
                ;
ADD_OP          : '+'
                ;
MINUS_OP        : '-'
                ;
MUL_OP          : '*'
                ;
DIV_OP          : '/'
                ;
PARCENT         : '%'
                ;
CONTAINS        : '<-'
                ;

WS  : [ \t]+ -> skip
    ;

mode TEXT_MODE;

QUOTE_CHAR          : '\\"' -> more
                    ;
BACKSLASH_CHAR      : '\\\\' -> more
                    ;
STRING              : '"' -> mode(DEFAULT_MODE)
                    ;
fragment CHARACTER  : .
                    ;
TEXT                : CHARACTER -> more
                    ;