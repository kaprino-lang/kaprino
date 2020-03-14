cmake_minimum_required(VERSION 3.16)

project(Kaprino VERSION 1.0)

set(ANTLR4_IncludePath "" CACHE STRING "ANTLR4 include files")
set(ANTLR4_LibPath "" CACHE STRING "ANTLR4 library files")

if ("" STREQUAL "${ANTLR4_IncludePath}")
    message(FATAL_ERROR "[Error] Required \${ANTLR4_IncludePath} but not applied")
    return()
endif()

if ("" STREQUAL "${ANTLR4_LibPath}")
    message(FATAL_ERROR "[Error] Required \${ANTLR4_LibPath} but not applied")
    return()
endif()

include_directories(${ANTLR4_IncludePath})
link_directories(${ANTLR4_LibPath})

file(GLOB PARSER_SOURCES "src/parser/*.cpp")

file(GLOB INTERPRETER_SOURCES "src/interpreter/*.cpp")

add_executable(ikaprino ${INTERPRETER_SOURCES} ${PARSER_SOURCES})
