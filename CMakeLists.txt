cmake_minimum_required(VERSION 3.15)

project(kaprino VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)

########################################################
#
# Dependencies pathes
#
########################################################

find_package(LLVM 9.0.0 REQUIRED)

set(KAPRINO_MONOTONE_LOG OFF CACHE BOOL "Make stdout monotone")
set(KAPRINO_OPTIMIZER_ON OFF CACHE BOOL "Optimize output")
set(KAPRINO_EMIT_LLVM_IR_ONLY OFF CACHE BOOL "Generate LLVM IR and finish program")
set(ANTLR4_IncludePath "" CACHE STRING "ANTLR4 include files")
set(ANTLR4_LibPath "" CACHE STRING "ANTLR4 library files")

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
set(INTERPRETER_SOURCES
    "${CMAKE_SOURCE_DIR}/src/interpreter/ArgsManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/ExecutableGenerator.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/FunctionManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/KaprinoParser.impl.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/PackagePathManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/TypeManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/VariableManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/internallib/PowInternal.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/internallib/PrintfInternal.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/internallib/ScanfInternal.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/CodeBlockStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/FunctionTypeVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/ProgramVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/assignee/ParameterAssigneeVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/AddExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/BooleanExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/BooleanOpExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/BracketExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/CompareExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/FunctionExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/MulExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/NotExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/NumberExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/ParameterExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/RealNumberExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/TextExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/expr/UpArrowExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/AssignStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/DefineFunctionStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/DefineProcessStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/ExitStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/IfStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/LetStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/LoopStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/PrintStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/ReadStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/RequireStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/SubmitStatementVisitor.cpp"
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

########################################################
#
# Define kaprino, the application which we want to get.
#
########################################################

add_executable(kprc ${INTERPRETER_SOURCES} ${PARSER_SOURCES})

target_include_directories(kprc PRIVATE ${ANTLR4_IncludePath})
target_include_directories(kprc PRIVATE ${LLVM_INCLUDE_DIRS})

if(UNIX)
    target_compile_options(kprc PRIVATE -frtti)
endif()

target_compile_definitions(kprc
    PRIVATE KAPRINO_VERSION=\"${kaprino_VERSION}\"
)

########################################################
#
# Link ANTLR4 libraries.
#
########################################################

target_link_directories(kprc PUBLIC ${ANTLR4_LibPath})
target_link_libraries(kprc antlr4-runtime)
if(MSVC)
    add_custom_command(
        TARGET kprc POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${ANTLR4_LibPath}/antlr4-runtime.dll
        $<TARGET_FILE_DIR:kprc>
    )
endif()

########################################################
#
# Link LLVM libraries.
#
########################################################

llvm_map_components_to_libnames(LLVM_LIBS all)
target_link_directories(kprc PUBLIC ${LLVM_LIBRARY_DIRS})
target_link_libraries(kprc ${LLVM_LIBS})

########################################################
#
# Make output monotone.
#
########################################################

if(${KAPRINO_MONOTONE_LOG})
    message(WARNING "[WARN] Make stdout monotone")
    target_compile_definitions(kprc
        PRIVATE KAPRINO_MONOTONE_LOG
    )
endif()

########################################################
#
# Define kaprino, the application which we want to get.
#
########################################################

if(${KAPRINO_OPTIMIZER_ON})
    message(WARNING "[EXPERIMENTAL] Optimizer on")
    target_compile_definitions(kprc
        PRIVATE KAPRINO_OPTIMIZER_ON
    )
endif()

#
# LLVM IR option
#
if(${KAPRINO_EMIT_LLVM_IR_ONLY})
    message(WARNING "[WARN] The tool will generate only llvm ir")
    target_compile_definitions(kprc
        PRIVATE KAPRINO_EMIT_LLVM_IR_ONLY
    )
endif()

########################################################
#
# Create install rule
#
########################################################

install(TARGETS kprc)
