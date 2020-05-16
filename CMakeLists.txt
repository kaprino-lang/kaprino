cmake_minimum_required(VERSION 3.15)

project(kaprino VERSION 1.0)

########################################################
#
# Dependencies pathes
#
########################################################

set(KAPRINO_MONOTONE_LOG OFF CACHE BOOL "Make stdout monotone")
set(KAPRINO_OPTIMIZER_ON OFF CACHE BOOL "Optimize output")
set(KAPRINO_EMIT_LLVM_IR_ONLY OFF CACHE BOOL "Generate LLVM IR and finish program")
set(ANTLR4_IncludePath "" CACHE STRING "ANTLR4 include files")
set(ANTLR4_LibPath "" CACHE STRING "ANTLR4 library files")
set(LLVM_IncludePath "" CACHE STRING "LLVM include files")
set(LLVM_LibPath "" CACHE STRING "LLVM library files")

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

if ("" STREQUAL "${LLVM_IncludePath}")
    message(FATAL_ERROR "[Error] Required \${LLVM_IncludePath} but not applied")
    return()
endif()

if ("" STREQUAL "${LLVM_LibPath}")
    message(FATAL_ERROR "[Error] Required \${LLVM_LibPath} but not applied")
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
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/ExitStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/IfStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/LetStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/LoopStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/PrintStatementVisitor.cpp"
    "${CMAKE_SOURCE_DIR}/src/interpreter/visitor/statements/ReadStatementVisitor.cpp"
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

add_executable(kaprino ${INTERPRETER_SOURCES} ${PARSER_SOURCES})

target_include_directories(kaprino PRIVATE ${ANTLR4_IncludePath})
target_include_directories(kaprino PRIVATE ${LLVM_IncludePath})

if(UNIX)
    target_compile_options(kaprino PRIVATE -frtti)
endif()

########################################################
#
# Link ANTLR4 libraries.
#
########################################################

target_link_directories(kaprino PUBLIC ${ANTLR4_LibPath})
target_link_libraries(kaprino antlr4-runtime)
if(MSVC)
    add_custom_command(
        TARGET kaprino POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${ANTLR4_LibPath}/antlr4-runtime.dll
        $<TARGET_FILE_DIR:kaprino>
    )
endif()

########################################################
#
# Link LLVM libraries.
#
########################################################

target_link_directories(kaprino PUBLIC ${LLVM_LibPath})
target_link_libraries(kaprino
    LLVMXRay
    LLVMWindowsManifest
    LLVMTextAPI
    LLVMTableGen
    LLVMSymbolize
    LLVMDebugInfoPDB
    LLVMOrcJIT
    LLVMJITLink
    LLVMObjectYAML
    LLVMMCA
    LLVMLTO
    LLVMPasses
    LLVMObjCARCOpts
    LLVMLineEditor
    LLVMLibDriver
    LLVMInterpreter
    LLVMFuzzMutate
    LLVMMCJIT
    LLVMExecutionEngine
    LLVMRuntimeDyld
    LLVMDlltoolDriver
    LLVMOption
    LLVMDebugInfoGSYM
    LLVMCoverage
    LLVMCoroutines
    LLVMXCoreDisassembler
    LLVMXCoreCodeGen
    LLVMXCoreDesc
    LLVMXCoreInfo
    LLVMX86Disassembler
    LLVMX86AsmParser
    LLVMX86CodeGen
    LLVMX86Desc
    LLVMX86Utils
    LLVMX86Info
    LLVMWebAssemblyDisassembler
    LLVMWebAssemblyCodeGen
    LLVMWebAssemblyDesc
    LLVMWebAssemblyAsmParser
    LLVMWebAssemblyInfo
    LLVMSystemZDisassembler
    LLVMSystemZCodeGen
    LLVMSystemZAsmParser
    LLVMSystemZDesc
    LLVMSystemZInfo
    LLVMSparcDisassembler
    LLVMSparcCodeGen
    LLVMSparcAsmParser
    LLVMSparcDesc
    LLVMSparcInfo
    LLVMRISCVDisassembler
    LLVMRISCVCodeGen
    LLVMRISCVAsmParser
    LLVMRISCVDesc
    LLVMRISCVUtils
    LLVMRISCVInfo
    LLVMPowerPCDisassembler
    LLVMPowerPCCodeGen
    LLVMPowerPCAsmParser
    LLVMPowerPCDesc
    LLVMPowerPCInfo
    LLVMNVPTXCodeGen
    LLVMNVPTXDesc
    LLVMNVPTXInfo
    LLVMMSP430Disassembler
    LLVMMSP430CodeGen
    LLVMMSP430AsmParser
    LLVMMSP430Desc
    LLVMMSP430Info
    LLVMMipsDisassembler
    LLVMMipsCodeGen
    LLVMMipsAsmParser
    LLVMMipsDesc
    LLVMMipsInfo
    LLVMLanaiDisassembler
    LLVMLanaiCodeGen
    LLVMLanaiAsmParser
    LLVMLanaiDesc
    LLVMLanaiInfo
    LLVMHexagonDisassembler
    LLVMHexagonCodeGen
    LLVMHexagonAsmParser
    LLVMHexagonDesc
    LLVMHexagonInfo
    LLVMBPFDisassembler
    LLVMBPFCodeGen
    LLVMBPFAsmParser
    LLVMBPFDesc
    LLVMBPFInfo
    LLVMARMDisassembler
    LLVMARMCodeGen
    LLVMARMAsmParser
    LLVMARMDesc
    LLVMARMUtils
    LLVMARMInfo
    LLVMAMDGPUDisassembler
    LLVMAMDGPUCodeGen
    LLVMMIRParser
    LLVMipo
    LLVMInstrumentation
    LLVMVectorize
    LLVMLinker
    LLVMIRReader
    LLVMAsmParser
    LLVMAMDGPUAsmParser
    LLVMAMDGPUDesc
    LLVMAMDGPUUtils
    LLVMAMDGPUInfo
    LLVMAArch64Disassembler
    LLVMMCDisassembler
    LLVMAArch64CodeGen
    LLVMGlobalISel
    LLVMSelectionDAG
    LLVMAsmPrinter
    LLVMDebugInfoDWARF
    LLVMCodeGen
    LLVMTarget
    LLVMScalarOpts
    LLVMInstCombine
    LLVMAggressiveInstCombine
    LLVMTransformUtils
    LLVMBitWriter
    LLVMAnalysis
    LLVMProfileData
    LLVMObject
    LLVMBitReader
    LLVMBitstreamReader
    LLVMCore
    LLVMRemarks
    LLVMAArch64AsmParser
    LLVMMCParser
    LLVMAArch64Desc
    LLVMMC
    LLVMDebugInfoCodeView
    LLVMDebugInfoMSF
    LLVMBinaryFormat
    LLVMAArch64Utils
    LLVMAArch64Info
    LLVMSupport
    LLVMDemangle
)

########################################################
#
# Make output monotone.
#
########################################################

if(${KAPRINO_MONOTONE_LOG})
    message(WARNING "[WARN] Make stdout monotone")
    target_compile_definitions(kaprino
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
    target_compile_definitions(kaprino
        PRIVATE KAPRINO_OPTIMIZER_ON
    )
endif()

#
# LLVM IR option
#
if(${KAPRINO_EMIT_LLVM_IR_ONLY})
    message(WARNING "[WARN] The tool will generate only llvm ir")
    target_compile_definitions(kaprino
        PRIVATE KAPRINO_EMIT_LLVM_IR_ONLY
    )
endif()

########################################################
#
# Create install rule
#
########################################################

install(TARGETS kaprino)
