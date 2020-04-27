#include <ostream>
#include <sstream>
#include <string>

#include "llvm/IR/Verifier.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"
#include "llvm/Pass.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/TargetRegistry.h"
#include "llvm/Support/TargetSelect.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/Target/TargetOptions.h"
#include "llvm/ADT/Optional.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/Support/InitLLVM.h"

#include "ExecutableGenerator.h"
#include "KaprinoAccelerator.h"

void EmitLLVMIR(llvm::Module* module, bool optimize) {
    KAPRINO_LOG("Emit LLVM IR file to: " << module->getName().str());

    std::error_code errorcode;
    auto stream = new llvm::raw_fd_ostream(module->getName(), errorcode);

    module->print(*stream, nullptr);

    KAPRINO_LOG("Outputing file task: " << errorcode.message());
}

void EmitExecutable(llvm::Module* module, bool optimize) {
    EmitLLVMIR(module, optimize);

    auto llvmir_path = module->getName().str();
    auto bitcode_path = KAPRINO_RM_FILE_EXT(llvmir_path) + ".bc";
    auto executable_path = KAPRINO_RM_FILE_EXT(llvmir_path);

    std::ostringstream conv_bitcode_command;
    conv_bitcode_command << "llvm-as -o ";
    conv_bitcode_command << bitcode_path;
    conv_bitcode_command << " ";
    conv_bitcode_command << llvmir_path;

    KAPRINO_LOG("Execute external tool: " << conv_bitcode_command.str());

    int error_code = system(conv_bitcode_command.str().c_str());
    if (error_code) {
        KAPRINO_ERR("llvm-as faield: " << error_code);
        throw error_code;
    }

    std::ostringstream compile_command;
    compile_command << "clang -o ";
    compile_command << executable_path;

#if _WIN32

    compile_command << ".exe";
    compile_command << " -llegacy_stdio_definitions.lib ";

#endif

    compile_command << " ";
    compile_command << bitcode_path;

    KAPRINO_LOG("Execute external tool: " << compile_command.str());

    error_code = system(compile_command.str().c_str());
    if (error_code) {
        KAPRINO_ERR("Clang faield: " << error_code);
        throw error_code;
    }

    KAPRINO_LOG("Compile complete");
}