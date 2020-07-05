########################################################
#
# Compile grammer files.
#
########################################################

antlr_target(KaprinoLexer
    "${CMAKE_SOURCE_DIR}/src/kast/src/KaprinoLexer.g4"
    LEXER
)
antlr_target(KaprinoParser
    "${CMAKE_SOURCE_DIR}/src/kast/src/KaprinoParser.g4"
    PARSER
    VISITOR
    DEPENDS_ANTLR KaprinoLexer
    COMPILE_FLAGS -lib ${ANTLR_KaprinoLexer_OUTPUT_DIR}
)
