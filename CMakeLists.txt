cmake_minimum_required(VERSION 3.16)

project(kaprino VERSION 1.0)

#
# Dependencies pathes
#
set(ANTLR4_IncludePath "" CACHE STRING "ANTLR4 include files")
set(ANTLR4_LibPath "" CACHE STRING "ANTLR4 library files")
set(LLVM_IncludePath "" CACHE STRING "LLVM include files")
set(LLVM_LibPath "" CACHE STRING "LLVM library files")

#
# Check whther dependencies pathes are all setted.
#
if ("" STREQUAL "${ANTLR4_IncludePath}")
    message(FATAL_ERROR "[Error] Required \${ANTLR4_IncludePath} but not applied")
    return()
endif()

if ("" STREQUAL "${ANTLR4_LibPath}")
    message(FATAL_ERROR "[Error] Required \${ANTLR4_LibPath} but not applied")
    return()
endif()

if ("" STREQUAL "${LLVM_IncludePath}")
    message(FATAL_ERROR "[Error] Required \${LLVM_IncludePath} but not applied")
    return()
endif()

if ("" STREQUAL "${LLVM_LibPath}")
    message(FATAL_ERROR "[Error] Required \${LLVM_LibPath} but not applied")
    return()
endif()

#
# Source files
#
set(PARSER_SOURCES
    "src/parser/KaprinoLexer.cpp"
    "src/parser/KaprinoParser.cpp"
    "src/parser/KaprinoParserBaseListener.cpp"
    "src/parser/KaprinoParserBaseVisitor.cpp"
    "src/parser/KaprinoParserListener.cpp"
    "src/parser/KaprinoParserVisitor.cpp"
)
set(INTERPRETER_SOURCES
    "src/interpreter/KaprinoParser.impl.cpp"
    "src/interpreter/VariableManager.cpp"
    "src/interpreter/visitor/ProgramVisitor.cpp"
    "src/interpreter/visitor/expr/NumberExprVisitor.cpp"
    "src/interpreter/visitor/statements/PrintStatementVisitor.cpp"
)

#
# Define kaprino, the application which we want to get.
#
add_executable(kaprino ${INTERPRETER_SOURCES} ${PARSER_SOURCES})

target_include_directories(kaprino PUBLIC ${ANTLR4_IncludePath})
target_include_directories(kaprino PUBLIC ${LLVM_IncludePath})

#
# Link directory
#
target_link_libraries(kaprino ${ANTLR4_LibPath}/antlr4-runtime.lib)
target_link_directories(kaprino PUBLIC ${LLVM_LibPath})
target_link_libraries(kaprino
    LLVMCore.lib
    LLVMSupport.lib
    LLVMBitWriter.lib
    LLVMMC.lib
    LLVMBinaryFormat.lib
    LLVMObject.lib
    LLVMBitReader.lib
    LLVMMCParser.lib
    LLVMObjectYAML.lib
    LLVMRemarks.lib
)
