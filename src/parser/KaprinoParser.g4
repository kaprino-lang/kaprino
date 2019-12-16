parser grammar KaprinoParser;

// --------------
//
// Parser
//
// --------------

options{
    tokenVocab = KaprinoLexer;
}

program         : statement*
                ;

statement       : CONSIDER parameter_define # ConsiderStatement
                | parameter ASSIGN expr # AssignStatement
                | run_macro # RunProcessStatement
                | function_define # DefineFunctionStatement
                | process_define # DefineProcessStatement
                | IF expr THEN CLOSER statement* CLOSER (OTHERWISE CLOSER statement* CLOSER)? # IfStatement
                | WHILE expr LOOP CLOSER statement* CLOSER # WhileStatement
                | CLASS parameter CLOSER (function_define | parameter_define | process_define)* CLOSER # ClassStatement
                | RETURN expr? # ReturnStatement
                ;
finisher        : DOT
                ;

function_define : function LEFT_BRACKET arguments RIGHT_BRACKET contains EQUAL expr # DefineFunctionSentence
                ;
process_define  : DEFINE function LEFT_BRACKET arguments RIGHT_BRACKET contains? CLOSER statement* CLOSER # DefineProcessSentence
                ;
parameter_define: parameter contains (COMMA parameter contains)? VARIABLE? # DefineParameterSentence
                ;
contains        : CONTAINS ID # ContainsSentence
                ;
arguments       : (parameter contains (COMMA parameter contains)*)?
                ;

run_macro       : function LEFT_BRACKET expr RIGHT_BRACKET # RunProcessSentence
                ;

expr            : LEFT_BRACKET expr RIGHT_BRACKET # BracketExpr
                | expr mul_op expr # MulExpr
                | expr add_op expr # AddExpr
                | expr compare_op expr # CompareExpr
                | expr boolean_op expr # BooleanOpExpr
                | not_op expr # NotExpr
                | number # NumberExpr
                | text # TextExpr
                | bool # BooleanExpr
                | function LEFT_BRACKET (expr (COMMA expr)*)? RIGHT_BRACKET # FunctionExpr
                | parameter # ParameterExpr
                ;

function        : ID
                ;
parameter       : ID
                | ID INDEX_OP expr
                ;
number          : NUMBER PARCENT?
                | REAL_NUMBER PARCENT?
                ;
bool            : TRUE
                | FALSE
                ;
mul_op          : MUL_OP
                | DIV_OP
                ;
add_op          : ADD_OP
                | MINUS_OP
                ;
not_op          : NOT
                ;
boolean_op      : AND
                | OR
                ;
compare_op      : GRATERTHAN
                | GRATEROREQUAL
                | LESSTHAN
                | LESSOREQUAL
                | EQUAL
                ;
text            : STRING
                ;
