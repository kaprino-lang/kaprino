cmake_minimum_required(VERSION 3.16)

project(kaprino VERSION 1.0)

#
# Dependencies pathes
#
set(KAPRINO_MONOTONE_LOG OFF CACHE BOOL "Make stdout monotone")
set(KAPRINO_OPTIMIZER_ON OFF CACHE BOOL "Optimize output")
set(KAPRINO_EMIT_LLVM_IR_ONLY OFF CACHE BOOL "Generate LLVM IR and finish program")
set(ANTLR4_IncludePath "" CACHE STRING "ANTLR4 include files")
set(ANTLR4_LibPath "" CACHE STRING "ANTLR4 library files")
set(LLVM_IncludePath "" CACHE STRING "LLVM include files")
set(LLVM_LibPath "" CACHE STRING "LLVM library files")

#
# Check whether dependencies pathes are all setted.
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
    "src/interpreter/ExecutableGenerator.cpp"
    "src/interpreter/FunctionManager.cpp"
    "src/interpreter/KaprinoParser.impl.cpp"
    "src/interpreter/TypeManager.cpp"
    "src/interpreter/VariableManager.cpp"
    "src/interpreter/internallib/PowInternal.cpp"
    "src/interpreter/internallib/PrintfInternal.cpp"
    "src/interpreter/internallib/ScanfInternal.cpp"
    "src/interpreter/visitor/CodeBlockStatementVisitor.cpp"
    "src/interpreter/visitor/FunctionTypeVisitor.cpp"
    "src/interpreter/visitor/ProgramVisitor.cpp"
    "src/interpreter/visitor/assignee/ParameterAssigneeVisitor.cpp"
    "src/interpreter/visitor/expr/AddExprVisitor.cpp"
    "src/interpreter/visitor/expr/BooleanExprVisitor.cpp"
    "src/interpreter/visitor/expr/BracketExprVisitor.cpp"
    "src/interpreter/visitor/expr/FunctionExprVisitor.cpp"
    "src/interpreter/visitor/expr/MulExprVisitor.cpp"
    "src/interpreter/visitor/expr/NumberExprVisitor.cpp"
    "src/interpreter/visitor/expr/ParameterExprVisitor.cpp"
    "src/interpreter/visitor/expr/TextExprVisitor.cpp"
    "src/interpreter/visitor/expr/UpArrowExprVisitor.cpp"
    "src/interpreter/visitor/statements/AssignStatementVisitor.cpp"
    "src/interpreter/visitor/statements/DefineFunctionStatementVisitor.cpp"
    "src/interpreter/visitor/statements/IfStatementVisitor.cpp"
    "src/interpreter/visitor/statements/LetStatementVisitor.cpp"
    "src/interpreter/visitor/statements/PrintStatementVisitor.cpp"
    "src/interpreter/visitor/statements/ReadStatementVisitor.cpp"
)

#
# Define kaprino, the application which we want to get.
#
add_executable(kaprino ${INTERPRETER_SOURCES} ${PARSER_SOURCES})

target_include_directories(kaprino PRIVATE ${ANTLR4_IncludePath})
target_include_directories(kaprino PRIVATE ${LLVM_IncludePath})

#
# Link ANTLR4
#
target_link_directories(kaprino PUBLIC ${ANTLR4_LibPath})
target_link_libraries(kaprino antlr4-runtime)
if(MSVC)
    add_custom_command(
        TARGET kaprino POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${ANTLR4_LibPath}/antlr4-runtime.dll
        $<TARGET_FILE_DIR:kaprino>
    )
else()
    add_custom_command(
        TARGET kaprino POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${ANTLR4_LibPath}/libantlr4-runtime.so
        $<TARGET_FILE_DIR:kaprino>
    )
endif()

#
# Link LLVM
#
target_link_directories(kaprino PUBLIC ${LLVM_LibPath})
target_link_libraries(kaprino
    LLVM-C
    LLVMAArch64AsmParser
    LLVMAArch64CodeGen
    LLVMAArch64Desc
    LLVMAArch64Disassembler
    LLVMAArch64Info
    LLVMAArch64Utils
    LLVMAggressiveInstCombine
    LLVMAMDGPUAsmParser
    LLVMAMDGPUCodeGen
    LLVMAMDGPUDesc
    LLVMAMDGPUDisassembler
    LLVMAMDGPUInfo
    LLVMAMDGPUUtils
    LLVMAnalysis
    LLVMARMAsmParser
    LLVMARMCodeGen
    LLVMARMDesc
    LLVMARMDisassembler
    LLVMARMInfo
    LLVMARMUtils
    LLVMAsmParser
    LLVMAsmPrinter
    LLVMBinaryFormat
    LLVMBitReader
    LLVMBitstreamReader
    LLVMBitWriter
    LLVMBPFAsmParser
    LLVMBPFCodeGen
    LLVMBPFDesc
    LLVMBPFDisassembler
    LLVMBPFInfo
    LLVMCodeGen
    LLVMCore
    LLVMCoroutines
    LLVMCoverage
    LLVMDebugInfoCodeView
    LLVMDebugInfoDWARF
    LLVMDebugInfoGSYM
    LLVMDebugInfoMSF
    LLVMDebugInfoPDB
    LLVMDemangle
    LLVMDlltoolDriver
    LLVMExecutionEngine
    LLVMFuzzMutate
    LLVMGlobalISel
    LLVMHexagonAsmParser
    LLVMHexagonCodeGen
    LLVMHexagonDesc
    LLVMHexagonDisassembler
    LLVMHexagonInfo
    LLVMInstCombine
    LLVMInstrumentation
    LLVMInterpreter
    LLVMipo
    LLVMIRReader
    LLVMJITLink
    LLVMLanaiAsmParser
    LLVMLanaiCodeGen
    LLVMLanaiDesc
    LLVMLanaiDisassembler
    LLVMLanaiInfo
    LLVMLibDriver
    LLVMLineEditor
    LLVMLinker
    LLVMLTO
    LLVMMC
    LLVMMCA
    LLVMMCDisassembler
    LLVMMCJIT
    LLVMMCParser
    LLVMMipsAsmParser
    LLVMMipsCodeGen
    LLVMMipsDesc
    LLVMMipsDisassembler
    LLVMMipsInfo
    LLVMMIRParser
    LLVMMSP430AsmParser
    LLVMMSP430CodeGen
    LLVMMSP430Desc
    LLVMMSP430Disassembler
    LLVMMSP430Info
    LLVMNVPTXCodeGen
    LLVMNVPTXDesc
    LLVMNVPTXInfo
    LLVMObjCARCOpts
    LLVMObject
    LLVMObjectYAML
    LLVMOption
    LLVMOrcJIT
    LLVMPasses
    LLVMPowerPCAsmParser
    LLVMPowerPCCodeGen
    LLVMPowerPCDesc
    LLVMPowerPCDisassembler
    LLVMPowerPCInfo
    LLVMProfileData
    LLVMRemarks
    LLVMRISCVAsmParser
    LLVMRISCVCodeGen
    LLVMRISCVDesc
    LLVMRISCVDisassembler
    LLVMRISCVInfo
    LLVMRISCVUtils
    LLVMRuntimeDyld
    LLVMScalarOpts
    LLVMSelectionDAG
    LLVMSparcAsmParser
    LLVMSparcCodeGen
    LLVMSparcDesc
    LLVMSparcDisassembler
    LLVMSparcInfo
    LLVMSupport
    LLVMSymbolize
    LLVMSystemZAsmParser
    LLVMSystemZCodeGen
    LLVMSystemZDesc
    LLVMSystemZDisassembler
    LLVMSystemZInfo
    LLVMTableGen
    LLVMTarget
    LLVMTextAPI
    LLVMTransformUtils
    LLVMVectorize
    LLVMWebAssemblyAsmParser
    LLVMWebAssemblyCodeGen
    LLVMWebAssemblyDesc
    LLVMWebAssemblyDisassembler
    LLVMWebAssemblyInfo
    LLVMWindowsManifest
    LLVMX86AsmParser
    LLVMX86CodeGen
    LLVMX86Desc
    LLVMX86Disassembler
    LLVMX86Info
    LLVMX86Utils
    LLVMXCoreCodeGen
    LLVMXCoreDesc
    LLVMXCoreDisassembler
    LLVMXCoreInfo
    LLVMXRay
)

if(${KAPRINO_MONOTONE_LOG})
    message(WARNING "[WARN] Make stdout monotone")
    target_compile_definitions(kaprino
        PRIVATE KAPRINO_MONOTONE_LOG
    )
endif()

#
# Optimizer option
#
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