#include <ostream>
#include <sstream>
#include <string>

#include "ExecutableGenerator.h"
#include "KaprinoAccelerator.h"

std::string EmitLLVMIR(llvm::Module* module, bool optimize) {
    auto llvmir_path = module->getName().str();

    std::error_code error_code;
    llvm::raw_fd_ostream dest(llvmir_path, error_code, llvm::sys::fs::OF_None);

    module->print(dest, nullptr);

    if (error_code) {
        KAPRINO_ERR("LLVM IR generation faield: " << error_code);
        throw error_code.value();
    }

    return llvmir_path;
}

std::string EmitBitcode(llvm::Module* module, bool optimize) {
    auto llvmir_path = module->getName().str();

    auto bitcode_path = KAPRINO_RM_FILE_EXT(llvmir_path) + ".bc";

    std::error_code error_code;
    llvm::raw_fd_ostream dest(bitcode_path, error_code, llvm::sys::fs::OF_None);

    llvm::WriteBitcodeToFile(*module, dest);

    if (error_code) {
        KAPRINO_ERR("Bitcode generation faield: " << error_code);
        throw error_code.value();
    }

    return bitcode_path;
}

std::string EmitExecutable(llvm::Module* module, bool optimize) {
    auto llvmir_path = EmitLLVMIR(module, optimize);

#if _WIN32

    auto executable_path = KAPRINO_RM_FILE_EXT(module->getName().str()) + ".exe";

#else

    auto executable_path = KAPRINO_RM_FILE_EXT(module->getName().str());

#endif

    std::ostringstream compile_command;
    compile_command << "clang -o ";
    compile_command << executable_path;

#if _WIN32

    compile_command << " -llegacy_stdio_definitions.lib ";

#endif

    compile_command << " ";
    compile_command << llvmir_path;

    KAPRINO_LOG("Execute external tool: " << compile_command.str());

    auto error_code = system(compile_command.str().c_str());
    if (error_code) {
        KAPRINO_ERR("Clang faield: " << error_code);
        throw error_code;
    }

    KAPRINO_LOG("Compile complete");

    return executable_path;
}
