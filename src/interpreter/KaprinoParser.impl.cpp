#include <fstream>
#include <iostream>

#include "../parser/KaprinoLexer.h"
#include "../parser/KaprinoParser.h"
#include "ExecutableGenerator.h"
#include "KaprinoAccelerator.h"
#include "StatementVisitor.h"
#include "TypeManager.h"
#include "abstructs/StatementObject.h"

using namespace antlr4;

std::vector<StatementObject*>* ParseFile(std::string text);
void GenerateCode(std::vector<StatementObject*>* programObj, std::string fileName);

std::vector<std::string> args;

bool getCompilerFlags(std::string name) {
    for (auto arg : args) {
        if (arg == name) return true;
    }
    return false;
}

int main_internal(int argc, const char* argv[]) {
    llvm::InitLLVM X(argc, argv);

    for (int counter = 0; counter < argc; counter++) {
        args.push_back(argv[counter]);
    }

    llvm::InitializeNativeTarget();
    llvm::InitializeNativeTargetAsmPrinter();
    llvm::InitializeNativeTargetAsmParser();

    KAPRINO_LOG_INIT();

    KAPRINO_LOG("Kaprino started");

    if (argc <= 1) {
        KAPRINO_ERR("No input");
        throw - 1;
    }

    std::string input_file_path = argv[1];
    std::ifstream input_file(input_file_path);

    if (!input_file.good()) {
        KAPRINO_ERR("Not found input files: \"" << argv[1] << "\"");
        throw - 1;
    }

    std::ostringstream ss;
    ss << input_file.rdbuf();
    std::string input_text;
    input_text = ss.str();

    KAPRINO_LOG("Read input files succeeded \"" << argv[1] << "\"");

    auto programObject = ParseFile(input_text);

    KAPRINO_LOG("Parsing succeeded");

    std::string output_file_path = KAPRINO_RM_FILE_EXT(input_file_path) + ".ll";

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

    TypeManager::create(&builder, module, "Text", KAPRINO_INT8_PTR_TY(module));
    TypeManager::create(&builder, module, "B", KAPRINO_BOOL_TY(module));
    TypeManager::create(&builder, module, "R", KAPRINO_DOUBLE_TY(module));
    TypeManager::create(&builder, module, "Z", KAPRINO_INT64_TY(module));

    for (auto statement : *programObj) {
        statement->codegen(&builder, module);
    }

    builder.CreateRet(llvm::ConstantInt::get(KAPRINO_INT32_TY(module), 0));

    llvm::verifyModule(*module);

#ifdef KAPRINO_EMIT_LLVM_IR_ONLY

    EmitLLVMIR(module, false);

#else

    auto optimize = getCompilerFlags("-O");
    auto llvmir = getCompilerFlags("--emit-llvm");
    auto afterrun = getCompilerFlags("--run") || getCompilerFlags("-r");

    KAPRINO_LOG("Optimization: " << (optimize ? "Aggressive" : "Default"));

    if (llvmir) {
        EmitLLVMIR(module, optimize);
    }

    auto executable_path = EmitExecutable(module, optimize);

    KAPRINO_LOG("Executable generated: " << executable_path);

#endif
}