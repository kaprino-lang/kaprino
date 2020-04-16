lexer grammar KaprinoLexer;

// --------------
//
// Lexer
//
// --------------

NEWLINE         : ('\r')? '\n' -> skip
                ;
COMMENT         : '//' .*? (NEWLINE | EOF) -> skip
                ;

//
// String
//
QUOTE           : '"' -> more, mode(TEXT_MODE)
                ;

//
// Reserved word
//
fragment COMMAND: '#'
                ;
LET             : COMMAND ('let' | 'Let' | 'LET')
                ;
PROCESS         : COMMAND ('process' | 'Process' | 'PROCESS')
                ;
IF              : COMMAND ('if' | 'If' | 'IF')
                ;
OTHERWISE       : COMMAND ('otherwise' | 'Otherwise' | 'OTHERWISE')
                ;
WHILE           : COMMAND ('while' | 'While' | 'WHILE')
                ;
LOOP            : COMMAND ('loop' | 'Loop' | 'LOOP')
                ;
BREAK           : COMMAND ('break' | 'Break' | 'BREAK')
                ;
CLASS           : COMMAND ('class' | 'Class' | 'CLASS')
                ;
SUBMIT          : COMMAND ('submit' | 'Submit' | 'SUBMIT')
                ;
PRINT           : COMMAND ('print' | 'Print' | 'PRINT')
                ;
READ            : COMMAND ('read' | 'Read' | 'READ')
                ;
AND             : 'and'
                ;
OR              : 'or'
                ;
NOT             : 'not'
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
UNDERBAR: '_'
        ;
CLOSER  : '|>'
        ;

//
// Boolean
//
B_TRUE    : ('true' | 'True')
        ;
B_FALSE   : ('false' | 'False')
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
UP_ARROW        : '^'
                ;
PARCENT         : '%'
                ;
CONTAINS        : '<-'
                ;
MAPPING_TO      : '->'
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