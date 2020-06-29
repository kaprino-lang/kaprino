########################################################
#
# Source files
#
########################################################

set(INTERPRETER_SOURCES
    "${CMAKE_SOURCE_DIR}/src/kprc/ArgsManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/DependencySolver.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/ExecutableGenerator.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/FunctionManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/KaprinoParser.impl.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/TypeManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/VariableManager.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/internallib/PowInternal.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/internallib/PrintfInternal.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/internallib/ScanfInternal.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/CodeBlockStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/FunctionTypeVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/ProgramVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/assignee/AccessAssigneeVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/assignee/ParameterAssigneeVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/AccessExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/AddExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/BooleanExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/BooleanOpExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/BracketExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/CompareExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/FunctionExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/MulExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/NotExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/NumberExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/ParameterExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/RealNumberExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/TextExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/expr/UpArrowExprVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/cstatements/ClassStructureObject.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/cstatements/LetCStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/AssignStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/ClassStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/DefineFunctionStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/DefineProcessStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/ExitStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/ExprStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/IfStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/LetStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/LoopStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/PrintStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/ReadStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/RequireStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/kprc/visitor/statements/SubmitStatementVisitor.cpp"
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