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

statement       : LET parameter=ID (EQUAL expr)? LEFT_BRACKET CONTAINS types=ID RIGHT_BRACKET finisher # LetStatement
                | LET parameter=ID LEFT_BRACKET function_type RIGHT_BRACKET finisher # LetFuncStatement
                | (call_func | ID) ASSIGN assigner=expr finisher # AssignStatement
                | call_func finisher # RunProcessStatement
                | funcname=ID LEFT_BRACKET (ID (COMMA ID)*)? RIGHT_BRACKET EQUAL expr LEFT_BRACKET function_type RIGHT_BRACKET finisher # DefineFunctionStatement
                | PROCESS ID LEFT_BRACKET (ID (COMMA ID)*)? RIGHT_BRACKET LEFT_BRACKET function_type RIGHT_BRACKET CLOSER statement* CLOSER # DefineProcessStatement
                | IF expr CLOSER ifst=statement* CLOSER (OTHERWISE CLOSER owst=statement* CLOSER)? # IfStatement
                | LOOP CLOSER statement* CLOSER # LoopStatement
                | CLASS name=ID CLOSER statement* CLOSER # ClassStatement
                | SUBMIT expr? finisher # SubmitStatement
                | BREAK finisher # BreakStatement
                | PRINT expr finisher # PrintStatement
                | READ (call_func | ID) finisher # ReadStatement
                ;
finisher        : DOT
                ;

call_func       : ID LEFT_BRACKET (expr (COMMA expr)*)? RIGHT_BRACKET # RunProcessSentence
                ;

expr            : LEFT_BRACKET expr RIGHT_BRACKET # BracketExpr
                | expr uparrow_op expr # UpArrowExpr
                | expr mul_op expr # MulExpr
                | expr add_op expr # AddExpr
                | expr compare_op expr # CompareExpr
                | expr boolean_op expr # BooleanOpExpr
                | not_op expr # NotExpr
                | number # NumberExpr
                | text # TextExpr
                | const_bool # BooleanExpr
                | call_func # FunctionExpr
                | ID # ParameterExpr
                ;

function_type   : ID (COMMA ID)* MAPPING_TO ID
                ;
number          : NUMBER PARCENT?
                | REAL_NUMBER PARCENT?
                ;
const_bool      : TRUE
                | FALSE
                ;
uparrow_op      : UP_ARROW
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
