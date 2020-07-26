#include "kgen/CodeGenerator.h"
#include "kgen/KgenAccelerator.h"
#include "kprc/ArgsManager.h"
#include "kprc/ExecutableGenerator.h"
#include "kprc/notifications/KprcNotificationHandler.h"

using namespace antlr4;

namespace kaprino::kprc {

int main_internal(int argc, const char* argv[]) {
    llvm::InitLLVM X(argc, argv);

    llvm::InitializeNativeTarget();
    llvm::InitializeNativeTargetAsmPrinter();
    llvm::InitializeNativeTargetAsmParser();

    kaprino::kgen::logger->log(
        "Kaprino started",
        "internal", 0, 0
    );

    ArgsManager::setArgs(argc, argv);

    if (ArgsManager::getFlag("--version") || ArgsManager::getFlag("-v")) {
        std::ostringstream ss;
        ss << "kprc - Kaprino Compiler version: " << KAPRINO_VERSION;

        kaprino::kgen::logger->log(
            ss.str(),
            "internal", 0, 0
        );
        return 0;
    }

    auto input_file_path = ArgsManager::getFile();
    input_file_path = std::filesystem::absolute(input_file_path)
        .string();

    kaprino::kgen::depsolver.files.push(input_file_path);

    auto input_directory = std::filesystem::path(input_file_path)
        .parent_path()
        .string();

    kaprino::kgen::depsolver
        .searchDirectories
        .push_back(input_directory);

    llvm::LLVMContext context;
    llvm::IRBuilder<> builder(context);
    llvm::Module* module;
    kaprino::kgen::CodeGenerator generator;
    ExecutableGenerator executable;

    module = new llvm::Module(generator.getOutFileName(input_file_path), context);

    generator.generate(input_file_path, &builder, module);

    auto optimize = ArgsManager::getFlag("-O");
    auto llvmir = ArgsManager::getFlag("--llvm-ir");
    auto afterrun = ArgsManager::getFlag("--run") || ArgsManager::getFlag("-r");

    if (optimize) {
        kaprino::kgen::logger->log(
            "Optimization: Aggressive",
            "internal", 0, 0
        );
    }

    if (llvmir) {
        executable.emitLLVMIR(module, optimize);
    } else {
        auto executable_path = executable.emitExecutable(module, optimize);

        kaprino::kgen::logger->log(
            "Executable generated: " + executable_path,
            "internal", 0, 0
        );

        if (afterrun) {
            kaprino::kgen::logger->log(
                "------ STARTING ------",
                "internal", 0, 0
            );
#if _WIN32
            int retval = system(("call \"" + executable_path + "\"").c_str());
#else
            int retval = system(executable_path.c_str());
#endif
            kaprino::kgen::logger->log(
                "------ FINISHING ------",
                "internal", 0, 0
            );

            kaprino::kgen::logger->log(
                "Executable returns: " + retval,
                "internal", 0, 0
            );

            return retval;
        }
    }
    return 0;
}

};  // namespace kaprino::kprc

int main(int argc, const char* argv[]) {
    kaprino::kgen::logger = new kaprino::kgen::NotificationManager();
    kaprino::kprc::KprcNotificationHandler::use();

    int ret_code = 0;
    try {
        kaprino::kprc::main_internal(argc, argv);
    } catch (int e) {
        ret_code = e;
    }

    kaprino::kgen::logger->log(
        "Kaprino returns: " + std::to_string(ret_code),
        "internal", 0, 0
    );
    return 0;
}
