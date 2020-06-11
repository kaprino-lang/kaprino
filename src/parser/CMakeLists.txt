########################################################
#
# Compile grammer files.
#
########################################################

antlr_target(KaprinoLexer
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoLexer.g4"
    LEXER
    VISITOR
)
antlr_target(KaprinoParser
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParser.g4"
    PARSER
    VISITOR
    DEPENDS_ANTLR KaprinoLexer
    COMPILE_FLAGS -lib ${ANTLR_KaprinoLexer_OUTPUT_DIR}
)
