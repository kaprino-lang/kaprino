########################################################
#
# Compile grammer files.
#
########################################################

set(KAST_INCLUDE_DIR "${CMAKE_SOURCE_DIR}/src/kast/dist")
set(KAST_SOURCE_DIR "${CMAKE_SOURCE_DIR}/src/kast/dist")
set(KAST_GRAMMER_FILES
    "${KAST_SOURCE_DIR}/../src/KaprinoLexer.g4"
    "${KAST_SOURCE_DIR}/../src/KaprinoParser.g4"
)

set(KAST_SOURCE
    "${KAST_SOURCE_DIR}/KaprinoLexer.cpp"
    "${KAST_SOURCE_DIR}/KaprinoParser.cpp"
    "${KAST_SOURCE_DIR}/KaprinoParserBaseListener.cpp"
    "${KAST_SOURCE_DIR}/KaprinoParserBaseVisitor.cpp"
    "${KAST_SOURCE_DIR}/KaprinoParserListener.cpp"
    "${KAST_SOURCE_DIR}/KaprinoParserVisitor.cpp"
)

add_custom_command(
    OUTPUT ${KAST_SOURCE}
    PRE_BUILD
    WORKING_DIR ${CMAKE_SOURCE_DIR}/src/kast
    COMMAND java -jar ${CMAKE_SOURCE_DIR}/antlr-4.8-complete.jar ${KAST_GRAMMER_FILES} -Dlanguage=Cpp -visitor -o ${KAST_SOURCE_DIR}
)
