########################################################
#
# Source files
#
########################################################

set(KGEN_INCLUDE_DIR "${CMAKE_SOURCE_DIR}/src/kgen/include")
set(KGEN_SOURCE_DIR "${CMAKE_SOURCE_DIR}/src/kgen/src")

set(KGEN_SOURCE
    "${KGEN_SOURCE_DIR}/internallib/PowInternal.cpp"
    "${KGEN_SOURCE_DIR}/internallib/PrintfInternal.cpp"
    "${KGEN_SOURCE_DIR}/internallib/ScanfInternal.cpp"

    "${KGEN_SOURCE_DIR}/notifications/NotificationManager.cpp"

    "${KGEN_SOURCE_DIR}/solvers/DependencySolver.cpp"
    "${KGEN_SOURCE_DIR}/solvers/FunctionManager.cpp"
    "${KGEN_SOURCE_DIR}/solvers/TypeManager.cpp"
    "${KGEN_SOURCE_DIR}/solvers/VariableManager.cpp"

    "${KGEN_SOURCE_DIR}/visitors/CodeBlockStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/FunctionTypeVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/ProgramVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/assignee/AccessAssigneeVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/assignee/ParameterAssigneeVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/AccessExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/AddExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/BooleanExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/BooleanOpExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/BracketExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/CompareExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/FunctionExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/MulExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/NotExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/NumberExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/ParameterExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/RealNumberExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/TextExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/expr/UpArrowExprVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/cstatements/ClassStructureObject.cpp"
    "${KGEN_SOURCE_DIR}/visitors/cstatements/LetCStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/AssignStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/ClassStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/DefineFunctionStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/DefineProcessStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/ExitStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/ExprStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/IfStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/LetStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/LoopStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/PrintStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/ReadStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/RequireStatementVisitor.cpp"
    "${KGEN_SOURCE_DIR}/visitors/statements/SubmitStatementVisitor.cpp"
)

########################################################
#
# Define kaprino, the application which we want to get.
#
########################################################

add_library(kgen ${KGEN_SOURCE} ${ANTLR_KaprinoLexer_CXX_OUTPUTS} ${ANTLR_KaprinoParser_CXX_OUTPUTS})

target_include_directories(kgen PRIVATE ${KGEN_INCLUDE_DIR})

if(UNIX)
    target_link_libraries(kgen stdc++fs)
endif()

target_compile_definitions(kgen
    PRIVATE KAPRINO_VERSION=\"${kaprino_VERSION}\"
)

########################################################
#
# Link ANTLR4 libraries.
#
########################################################

target_include_directories(kgen PRIVATE ${ANTLR4_INCLUDE_DIRS})
target_include_directories(kgen PRIVATE ${ANTLR_KaprinoLexer_OUTPUT_DIR})
target_include_directories(kgen PRIVATE ${ANTLR_KaprinoParser_OUTPUT_DIR})

target_link_libraries(kgen antlr4_static)

########################################################
#
# Link LLVM libraries.
#
########################################################

target_include_directories(kgen PRIVATE ${LLVM_INCLUDE_DIRS})

llvm_map_components_to_libnames(LLVM_LIBS all)
target_link_directories(kgen PUBLIC ${LLVM_LIBRARY_DIRS})
target_link_libraries(kgen ${LLVM_LIBS})

########################################################
#
# Make output monotone.
#
########################################################

if(${KAPRINO_MONOTONE_LOG})
    message(WARNING "[WARN] Make stdout monotone")
    target_compile_definitions(kgen
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
    target_compile_definitions(kgen
        PRIVATE KAPRINO_OPTIMIZER_ON
    )
endif()

########################################################
#
# Create install rule
#
########################################################

install(TARGETS kgen)