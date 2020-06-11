########################################################
#
# Source files
#
########################################################

set(INTERPRETER_SOURCES
    "${CMAKE_SOURCE_DIR}/src/interpreter/ArgsManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/DependencySolver.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/ExecutableGenerator.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/FunctionManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/KaprinoParser.impl.cpp"
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
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/ExprStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/IfStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/LetStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/LoopStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/PrintStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/ReadStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/RequireStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/SubmitStatementVisitor.cpp"
)

########################################################
#
# Define kaprino, the application which we want to get.
#
########################################################

add_executable(kprc ${INTERPRETER_SOURCES} ${ANTLR_KaprinoLexer_CXX_OUTPUTS} ${ANTLR_KaprinoParser_CXX_OUTPUTS})

target_include_directories(kprc PRIVATE ${LLVM_INCLUDE_DIRS})

target_include_directories(kprc PRIVATE ${ANTLR4_INCLUDE_DIRS})
target_include_directories(kprc PRIVATE ${ANTLR_KaprinoLexer_OUTPUT_DIR})
target_include_directories(kprc PRIVATE ${ANTLR_KaprinoParser_OUTPUT_DIR})

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

target_link_libraries(kprc antlr4_static)

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

########################################################
#
# Create install rule
#
########################################################

install(TARGETS kprc)