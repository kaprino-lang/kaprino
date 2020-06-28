#include <fstream>
#include <iostream>

#include "KaprinoLexer.h"
#include "KaprinoParser.h"
#include "abstructs/StatementObject.h"
#include "ArgsManager.h"
#include "ExecutableGenerator.h"
#include "KaprinoAccelerator.h"
#include "StatementVisitor.h"
#include "TypeManager.h"

using namespace antlr4;

#ifndef KAPRINO_VERSION
#define KAPRINO_VERSION "0.0.0"
#endif

std::vector<StatementObject*>* ParseFile(std::string text);
void GenerateCode(std::vector<StatementObject*>* programObj, std::string fileName);

int main_internal(int argc, const char* argv[]) {
    llvm::InitLLVM X(argc, argv);

    llvm::InitializeNativeTarget();
    llvm::InitializeNativeTargetAsmPrinter();
    llvm::InitializeNativeTargetAsmParser();

    KAPRINO_LOG_INIT();

    KAPRINO_LOG("Kaprino started");

    ArgsManager::setArgs(argc, argv);

    if (ArgsManager::getFlag("--version") || ArgsManager::getFlag("-v")) {
        KAPRINO_LOG("kprc - Kaprino Compiler version: " << KAPRINO_VERSION);
        return 0;
    }

    std::string input_file_path = ArgsManager::getFile();
    std::ifstream input_file(input_file_path);

    if (!input_file.good()) {
        KAPRINO_ERR("Not found input files: \"" << ArgsManager::getFile() << "\"");
        throw -1;
    }

    std::ostringstream ss;
    ss << input_file.rdbuf();
    std::string input_text;
    input_text = ss.str();

    KAPRINO_LOG("Read input files succeeded \"" << ArgsManager::getFile() << "\"");

    auto programObject = ParseFile(input_text);

    KAPRINO_LOG("Parsing succeeded");

    auto output_file_path = std::filesystem::path(input_file_path)
        .replace_extension(".ll")
        .string();

    GenerateCode(programObject, output_file_path);
}

int main(int argc, const char* argv[]) {
    try {
        main_internal(argc, argv);
    }
    catch(int e) {
        KAPRINO_LOG("Kaprino returns: " << e);
        return e;
    }

    KAPRINO_LOG("Kapino returns: 0");
    return 0;
}

std::vector<StatementObject*>* ParseFile(std::string text) {
    ANTLRInputStream input(text);
    KaprinoLexer lexer(&input);
    CommonTokenStream tokens(&lexer);
    KaprinoParser parser(&tokens);

    auto program = parser.program();

    StatementVisitor visitor;
    auto programObject = visitor.visitProgram(program).as<std::vector<StatementObject*>*>();

    return programObject;
}

void ProcessDeadCode(llvm::IRBuilder<>& builder, llvm::Module* module) {
    for (auto& func_it : *module) {
        for (auto& basic_block : func_it) {
            auto size = basic_block.size();
            if (size == 0) {
                builder.SetInsertPoint(&basic_block);
                builder.CreateUnreachable();
            }
        }
    }
}

void GenerateCode(std::vector<StatementObject*>* programObj, std::string fileName) {
    llvm::LLVMContext context;
    llvm::IRBuilder<> builder(context);
    llvm::Module* module;

    module = new llvm::Module(fileName, context);

    std::vector<llvm::Type*> mainFuncArgs;

    auto mainFuncType = llvm::FunctionType::get(
        KAPRINO_INT32_TY(module),
        mainFuncArgs,
        false);

    auto mainFunc = llvm::Function::Create(
        mainFuncType,
        llvm::GlobalValue::ExternalLinkage,
        "main",
        module);
    mainFunc->setCallingConv(llvm::CallingConv::C);

    auto mainBlock = llvm::BasicBlock::Create(context, "entry", module->getFunction("main"));
    builder.SetInsertPoint(mainBlock);

    TypeManager::create(&builder, module, "Nil", KAPRINO_VOID_TY(module));
    TypeManager::create(&builder, module, "Text", KAPRINO_INT8_PTR_TY(module));
    TypeManager::create(&builder, module, "B", KAPRINO_BOOL_TY(module));
    TypeManager::create(&builder, module, "R", KAPRINO_DOUBLE_TY(module));
    TypeManager::create(&builder, module, "Z", KAPRINO_INT64_TY(module));

    for (auto statement : *programObj) {
        statement->codegen(&builder, module);
    }

    builder.CreateRet(llvm::ConstantInt::get(KAPRINO_INT32_TY(module), 0));

    ProcessDeadCode(builder, module);

    llvm::verifyModule(*module);

    auto optimize = ArgsManager::getFlag("-O");
    auto llvmir = ArgsManager::getFlag("--llvm-ir");
    auto afterrun = ArgsManager::getFlag("--run") || ArgsManager::getFlag("-r");

    KAPRINO_LOG("Optimization: " << (optimize ? "Aggressive" : "Default"));

    if (llvmir) {
        EmitLLVMIR(module, optimize);
    }
    else {
        auto executable_path = EmitExecutable(module, optimize);

        KAPRINO_LOG("Executable generated: " << executable_path);

        if (afterrun) {
            KAPRINO_LOG("------ STARTING ------");
#if _WIN32
            int retval = system(("call \"" + executable_path + "\"").c_str());
#else
            int retval = system(executable_path.c_str());
#endif
            KAPRINO_LOG("------ FINISHING ------");
            KAPRINO_LOG("Executable returns: " << retval);
        }
    }
}