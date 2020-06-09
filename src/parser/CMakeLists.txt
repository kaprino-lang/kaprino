########################################################
#
# Check whether dependencies pathes are all setted.
#
########################################################

if ("" STREQUAL "${ANTLR4_IncludePath}")
    message(FATAL_ERROR "[Error] Required \${ANTLR4_IncludePath} but not applied")
    return()
endif()

if ("" STREQUAL "${ANTLR4_LibPath}")
    message(FATAL_ERROR "[Error] Required \${ANTLR4_LibPath} but not applied")
    return()
endif()

########################################################
#
# Source files
#
########################################################

set(PARSER_SOURCES
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoLexer.cpp"
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParser.cpp"
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParserBaseListener.cpp"
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParserBaseVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParserListener.cpp"
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParserVisitor.cpp"
)
set(GRAMMER_FILES
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoLexer.g4"
    "${CMAKE_SOURCE_DIR}/src/parser/KaprinoParser.g4"
)

########################################################
#
# Compile grammer files.
#
########################################################

add_custom_command(
    OUTPUT ${PARSER_SOURCES}
    COMMAND antlr4 ${GRAMMER_FILES} -Dlanguage=Cpp -visitor -o src/parser
    COMMENT "Generate lexer and parser from Grammer files"
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    VERBATIM
)
