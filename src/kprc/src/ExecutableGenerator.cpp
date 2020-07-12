#include "kgen/solvers/DependencySolver.h"
#include "kprc/ExecutableGenerator.h"

namespace kaprino::kprc {

std::string ExecutableGenerator::emitLLVMIR(llvm::Module* module, bool optimize) {
    auto llvmir_path = module->getName().str();

    std::error_code error_code;
    llvm::raw_fd_ostream dest(llvmir_path, error_code, llvm::sys::fs::OF_None);

    module->print(dest, nullptr);

    if (error_code) {
        kaprino::kgen::logger->error(
            "LLVM IR generation faield: " + error_code.value(),
            "internal", 0, 0
        );
        throw error_code.value();
    }

    return llvmir_path;
}

std::string ExecutableGenerator::emitBitcode(llvm::Module* module, bool optimize) {
    auto llvmir_path = module->getName().str();

    auto bitcode_path = std::filesystem::path(llvmir_path)
        .replace_extension(".bc")
        .string();

    std::error_code error_code;
    llvm::raw_fd_ostream dest(bitcode_path, error_code, llvm::sys::fs::OF_None);

    llvm::WriteBitcodeToFile(*module, dest);

    if (error_code) {
        kaprino::kgen::logger->error(
            "Bitcode generation faield: " + error_code.value(),
            "internal", 0, 0
        );
        throw error_code.value();
    }

    return bitcode_path;
}

std::string ExecutableGenerator::emitExecutable(llvm::Module* module, bool optimize) {
    auto llvmir_path = emitLLVMIR(module, optimize);

#if _WIN32

    auto executable_path = std::filesystem::path(llvmir_path)
        .replace_extension(".exe")
        .string();

#else

    auto executable_path = std::filesystem::path(llvmir_path)
        .replace_extension("")
        .string();

#endif

    std::ostringstream compile_command;
    compile_command << "clang -o ";
    compile_command << "\"" << executable_path << "\"";

#if _WIN32

    compile_command << " -llegacy_stdio_definitions.lib ";

#endif

    compile_command << " ";
    compile_command << "\"" << llvmir_path << "\"";

    for (auto link_file : kaprino::kgen::DependencySolver::linkFiles) {
        compile_command << " ";
        compile_command << "\"" << link_file << "\"";
    }

    kaprino::kgen::logger->log(
        "Execute external tool: " + compile_command.str(),
        "internal", 0, 0
    );

    auto error_code = system(compile_command.str().c_str());
    if (error_code) {
        kaprino::kgen::logger->error(
            "Clang faield: " + error_code,
            "internal", 0, 0
        );
        throw error_code;
    }

    kaprino::kgen::logger->log(
        "Compile complete",
        "internal", 0, 0
    );

    return executable_path;
}

};  // namespace kaprino::kprc
