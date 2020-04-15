#include <iostream>

#include "antlr4-runtime.h"
#include "llvm/IR/Verifier.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"

#include "../parser/KaprinoLexer.h"
#include "../parser/KaprinoParser.h"
#include "StatementVisitor.h"
#include "abstructs/StatementObject.h"

using namespace antlr4;

std::vector<StatementObject*>* ParseFile(std::string fileName);
void GenerateCode(std::vector<StatementObject*>* programObj, std::string fileName);

#if KAPRINO_OPTIMIZER_ON

void OptimizeAll(llvm::Module* module);

#endif

int main(int argc, const char* argv[]) {
    if (argc <= 1) {
        std::cerr << "[ERROR] No input" << std::endl;
        return -1;
    }

    auto programObject = ParseFile(argv[1]);

    GenerateCode(programObject, "code.ll");
}

std::vector<StatementObject*>* ParseFile(std::string fileName) {
    ANTLRFileStream input(fileName);
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
        llvm::Type::getInt32Ty(context),
        mainFuncArgs,
        false
    );

    auto mainFunc = llvm::Function::Create(
        mainFuncType,
        llvm::GlobalValue::ExternalLinkage,
        "main",
        module
    );
    mainFunc->setCallingConv(llvm::CallingConv::C);

    auto mainBlock = llvm::BasicBlock::Create(context, "entry", module->getFunction("main"));
    builder.SetInsertPoint(mainBlock);

    for(auto statement : *programObj) {
        statement->codegen(&builder, module);
    }

    builder.CreateRet(llvm::ConstantInt::get(llvm::Type::getInt32Ty(context), 0));

    llvm::verifyModule(*module);

    std::error_code errorcode;
    auto stream = new llvm::raw_fd_ostream(fileName, errorcode);

    std::cout << "[LOG] Outputing file task returns: " << errorcode.value() << std::endl;

#if KAPRINO_OPTIMIZER_ON

    GenerateCode(module);

#endif

    module->print(*stream, nullptr);
}

#if KAPRINO_OPTIMIZER_ON

void OptimizeAll(llvm::Module* module) {
    llvm::PassBuilder passBuilder;
    llvm::LoopAnalysisManager loopAnalysisManager(true);
    llvm::FunctionAnalysisManager functionAnalysisManager(true);
    llvm::CGSCCAnalysisManager cGSCCAnalysisManager(true);
    llvm::ModuleAnalysisManager moduleAnalysisManager(true);
    llvm::ModulePassManager modulePassManager;
    passBuilder.registerModuleAnalyses(moduleAnalysisManager);
    passBuilder.registerCGSCCAnalyses(cGSCCAnalysisManager);
    passBuilder.registerFunctionAnalyses(functionAnalysisManager);
    passBuilder.registerLoopAnalyses(loopAnalysisManager);
    passBuilder.crossRegisterProxies(
        loopAnalysisManager,
        functionAnalysisManager,
        cGSCCAnalysisManager,
        moduleAnalysisManager
    );
    modulePassManager = passBuilder.buildPerModuleDefaultPipeline(
        llvm::PassBuilder::OptimizationLevel::O3
    );
    modulePassManager.run(*module, moduleAnalysisManager);
}

#endif
