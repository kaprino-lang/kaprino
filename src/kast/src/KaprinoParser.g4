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

statement       : LET name=ID (EQUAL expr)? LEFT_BRACKET CONTAINS types=ID RIGHT_BRACKET # LetStatement
                | assignee ASSIGN assigner=expr # AssignStatement
                | expr # ExprStatement
                | ID LEFT_BRACKET (ID (COMMA ID)*)? RIGHT_BRACKET EQUAL expr LEFT_BRACKET function_type RIGHT_BRACKET # DefineFunctionStatement
                | FUNC ID LEFT_BRACKET (ID (COMMA ID)*)? RIGHT_BRACKET LEFT_BRACKET function_type RIGHT_BRACKET (codeblock | (EXTERN)) # DefineProcessStatement
                | IF expr codeblock (OTHERWISE codeblock)? # IfStatement
                | LOOP codeblock # LoopStatement
                | CLASS name=ID CLOSER classstatement* CLOSER # ClassStatement
                | SUBMIT expr? # SubmitStatement
                | EXIT # ExitStatement
                | PRINT expr (COMMA expr)* # PrintStatement
                | READ assignee # ReadStatement
                | REQUIRE text # RequireStatement
                ;

classstatement  : LET name=ID LEFT_BRACKET CONTAINS types=ID RIGHT_BRACKET # LetCStatement
                | ID LEFT_BRACKET (ID (COMMA ID)*)? RIGHT_BRACKET EQUAL expr LEFT_BRACKET function_type RIGHT_BRACKET # FunctionCStatement
                | FUNC ID LEFT_BRACKET (ID (COMMA ID)*)? RIGHT_BRACKET LEFT_BRACKET function_type RIGHT_BRACKET (codeblock | (EXTERN)) # ProcessCStatement
                ;

codeblock       : CLOSER statement* CLOSER # CodeBlockStatement
                ;

expr            : LEFT_BRACKET expr RIGHT_BRACKET # BracketExpr
                | expr UNDERBAR ID # AccessExpr
                | expr uparrow_op expr # UpArrowExpr
                | expr mul_op expr # MulExpr
                | expr add_op expr # AddExpr
                | expr compare_op expr # CompareExpr
                | expr boolean_op expr # BooleanOpExpr
                | not_op expr # NotExpr
                | number # NumberExpr
                | real_number # RealNumberExpr
                | text # TextExpr
                | const_bool # BooleanExpr
                | name=ID # ParameterExpr
                | ID LEFT_BRACKET (expr (COMMA expr)*)? RIGHT_BRACKET # FunctionExpr
                ;

function_type   : ID (COMMA ID)* MAPPING_TO ID # FunctionType
                ;

assignee        : ID # ParameterAssignee
                | assignee UNDERBAR ID # AccessAssignee
                | assignee LEFT_BRACKET (expr (COMMA expr)*)? RIGHT_BRACKET # FunctionAssignee
                ;

//
// There are no plans to implement the visitors of them.
//
number          : NUMBER PARCENT?
                ;
real_number     : REAL_NUMBER PARCENT?
                ;
const_bool      : B_TRUE
                | B_FALSE
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
