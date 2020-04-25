cmake_minimum_required(VERSION 3.16)

project(kaprino VERSION 1.0)

#
# Dependencies pathes
#
set(KAPRINO_WIN32 OFF CACHE BOOL "Activate Windows virtual terminal processor")
set(KAPRINO_MONOTONE_LOG OFF CACHE BOOL "Make stdout monotone")
set(KAPRINO_OPTIMIZER_ON OFF CACHE BOOL "Optimize output")
set(KAPRINO_EMIT_OBJECT_CODE OFF CACHE BOOL "Generate object codes after running")
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
    "src/interpreter/KaprinoParser.impl.cpp"
    "src/interpreter/VariableManager.cpp"
    "src/interpreter/internallib/PrintfInternal.cpp"
    "src/interpreter/internallib/ScanfInternal.cpp"
    "src/interpreter/visitor/ProgramVisitor.cpp"
    "src/interpreter/visitor/assignee/ParameterAssigneeVisitor.cpp"
    "src/interpreter/visitor/expr/NumberExprVisitor.cpp"
    "src/interpreter/visitor/expr/ParameterExprVisitor.cpp"
    "src/interpreter/visitor/expr/TextExprVisitor.cpp"
    "src/interpreter/visitor/statements/LetStatementVisitor.cpp"
    "src/interpreter/visitor/statements/PrintStatementVisitor.cpp"
    "src/interpreter/visitor/statements/ReadStatementVisitor.cpp"
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
add_custom_command(
    TARGET kaprino POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
    ${ANTLR4_LibPath}/antlr4-runtime.dll
    $<TARGET_FILE_DIR:kaprino>
)
target_link_directories(kaprino PUBLIC ${LLVM_LibPath})
target_link_libraries(kaprino
    LLVM-C.lib
    LLVMAArch64AsmParser.lib
    LLVMAArch64CodeGen.lib
    LLVMAArch64Desc.lib
    LLVMAArch64Disassembler.lib
    LLVMAArch64Info.lib
    LLVMAArch64Utils.lib
    LLVMAggressiveInstCombine.lib
    LLVMAMDGPUAsmParser.lib
    LLVMAMDGPUCodeGen.lib
    LLVMAMDGPUDesc.lib
    LLVMAMDGPUDisassembler.lib
    LLVMAMDGPUInfo.lib
    LLVMAMDGPUUtils.lib
    LLVMAnalysis.lib
    LLVMARMAsmParser.lib
    LLVMARMCodeGen.lib
    LLVMARMDesc.lib
    LLVMARMDisassembler.lib
    LLVMARMInfo.lib
    LLVMARMUtils.lib
    LLVMAsmParser.lib
    LLVMAsmPrinter.lib
    LLVMBinaryFormat.lib
    LLVMBitReader.lib
    LLVMBitstreamReader.lib
    LLVMBitWriter.lib
    LLVMBPFAsmParser.lib
    LLVMBPFCodeGen.lib
    LLVMBPFDesc.lib
    LLVMBPFDisassembler.lib
    LLVMBPFInfo.lib
    LLVMCodeGen.lib
    LLVMCore.lib
    LLVMCoroutines.lib
    LLVMCoverage.lib
    LLVMDebugInfoCodeView.lib
    LLVMDebugInfoDWARF.lib
    LLVMDebugInfoGSYM.lib
    LLVMDebugInfoMSF.lib
    LLVMDebugInfoPDB.lib
    LLVMDemangle.lib
    LLVMDlltoolDriver.lib
    LLVMExecutionEngine.lib
    LLVMFuzzMutate.lib
    LLVMGlobalISel.lib
    LLVMHexagonAsmParser.lib
    LLVMHexagonCodeGen.lib
    LLVMHexagonDesc.lib
    LLVMHexagonDisassembler.lib
    LLVMHexagonInfo.lib
    LLVMInstCombine.lib
    LLVMInstrumentation.lib
    LLVMInterpreter.lib
    LLVMipo.lib
    LLVMIRReader.lib
    LLVMJITLink.lib
    LLVMLanaiAsmParser.lib
    LLVMLanaiCodeGen.lib
    LLVMLanaiDesc.lib
    LLVMLanaiDisassembler.lib
    LLVMLanaiInfo.lib
    LLVMLibDriver.lib
    LLVMLineEditor.lib
    LLVMLinker.lib
    LLVMLTO.lib
    LLVMMC.lib
    LLVMMCA.lib
    LLVMMCDisassembler.lib
    LLVMMCJIT.lib
    LLVMMCParser.lib
    LLVMMipsAsmParser.lib
    LLVMMipsCodeGen.lib
    LLVMMipsDesc.lib
    LLVMMipsDisassembler.lib
    LLVMMipsInfo.lib
    LLVMMIRParser.lib
    LLVMMSP430AsmParser.lib
    LLVMMSP430CodeGen.lib
    LLVMMSP430Desc.lib
    LLVMMSP430Disassembler.lib
    LLVMMSP430Info.lib
    LLVMNVPTXCodeGen.lib
    LLVMNVPTXDesc.lib
    LLVMNVPTXInfo.lib
    LLVMObjCARCOpts.lib
    LLVMObject.lib
    LLVMObjectYAML.lib
    LLVMOption.lib
    LLVMOrcJIT.lib
    LLVMPasses.lib
    LLVMPowerPCAsmParser.lib
    LLVMPowerPCCodeGen.lib
    LLVMPowerPCDesc.lib
    LLVMPowerPCDisassembler.lib
    LLVMPowerPCInfo.lib
    LLVMProfileData.lib
    LLVMRemarks.lib
    LLVMRISCVAsmParser.lib
    LLVMRISCVCodeGen.lib
    LLVMRISCVDesc.lib
    LLVMRISCVDisassembler.lib
    LLVMRISCVInfo.lib
    LLVMRISCVUtils.lib
    LLVMRuntimeDyld.lib
    LLVMScalarOpts.lib
    LLVMSelectionDAG.lib
    LLVMSparcAsmParser.lib
    LLVMSparcCodeGen.lib
    LLVMSparcDesc.lib
    LLVMSparcDisassembler.lib
    LLVMSparcInfo.lib
    LLVMSupport.lib
    LLVMSymbolize.lib
    LLVMSystemZAsmParser.lib
    LLVMSystemZCodeGen.lib
    LLVMSystemZDesc.lib
    LLVMSystemZDisassembler.lib
    LLVMSystemZInfo.lib
    LLVMTableGen.lib
    LLVMTarget.lib
    LLVMTextAPI.lib
    LLVMTransformUtils.lib
    LLVMVectorize.lib
    LLVMWebAssemblyAsmParser.lib
    LLVMWebAssemblyCodeGen.lib
    LLVMWebAssemblyDesc.lib
    LLVMWebAssemblyDisassembler.lib
    LLVMWebAssemblyInfo.lib
    LLVMWindowsManifest.lib
    LLVMX86AsmParser.lib
    LLVMX86CodeGen.lib
    LLVMX86Desc.lib
    LLVMX86Disassembler.lib
    LLVMX86Info.lib
    LLVMX86Utils.lib
    LLVMXCoreCodeGen.lib
    LLVMXCoreDesc.lib
    LLVMXCoreDisassembler.lib
    LLVMXCoreInfo.lib
    LLVMXRay.lib
)

#
# Activate Windows virtual terminal processor.
#
if (${KAPRINO_WIN32})
    message(WARNING "[WARN] Win32 option is currently activated")
    target_compile_definitions(kaprino
        PUBLIC KAPRINO_WIN32
    )
endif()

if (${KAPRINO_MONOTONE_LOG})
    message(WARNING "[WARN] Make stdout monotone")
    target_compile_definitions(kaprino
        PUBLIC KAPRINO_MONOTONE_LOG
    )
endif()

#
# Optimizer option
#
if (${KAPRINO_OPTIMIZER_ON})
    message(WARNING "[EXPERIMENTAL] Optimizer on")
    target_compile_definitions(kaprino
        PUBLIC KAPRINO_OPTIMIZER_ON
    )
endif()

#
# llvm-llc option
#
if (${KAPRINO_EMIT_OBJECT_CODE})
    message(WARNING "[EXPERIMENTAL] The tool will generate object codes after running")
    target_compile_definitions(kaprino
        PUBLIC KAPRINO_EMIT_OBJECT_CODE
    )
endif()