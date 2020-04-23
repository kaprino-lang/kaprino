#include <iostream>
#include <fstream>

#include "antlr4-runtime.h"
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

#include "KaprinoLogger.h"
#include "../parser/KaprinoLexer.h"
#include "../parser/KaprinoParser.h"
#include "StatementVisitor.h"
#include "abstructs/StatementObject.h"

using namespace antlr4;

std::vector<StatementObject*>* ParseFile(std::string text);
void GenerateCode(std::vector<StatementObject*>* programObj, std::string fileName);

#ifdef KAPRINO_EMIT_OBJECT_CODE

void EmitObjectCode(llvm::Module* module);

#endif

#ifdef KAPRINO_OPTIMIZER_ON

void OptimizeAll(llvm::Module* module);

#endif

int main(int argc, const char* argv[]) {
    llvm::InitLLVM X(argc, argv);

    KAPRINO_LOG_INIT();

    KAPRINO_LOG("Kaprino started");

    if (argc <= 1) {
        KAPRINO_ERR("No input");
        return -1;
    }

    std::string input_file_path = argv[1];
    std::ifstream input_file(input_file_path);

    if (!input_file.good()) {
        KAPRINO_ERR("Not found input files: \"" << argv[1] << "\"");
        return -1;
    }

    std::ostringstream ss;
    ss << input_file.rdbuf();
    std::string input_text;
    input_text = ss.str();

    KAPRINO_LOG("Read input files succeeded \"" << argv[1] << "\"");

    auto programObject = ParseFile(input_text);

    KAPRINO_LOG("Parsing succeeded");

    std::string input_without_extension = input_file_path.substr(0, input_file_path.find_last_of("."));
    std::string output_file_path = input_without_extension + ".ll";

    GenerateCode(programObject, output_file_path);
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

    KAPRINO_LOG("Outputing file task: " << errorcode.message());

#ifdef KAPRINO_OPTIMIZER_ON

    OptimizeAll(module);

#endif

    module->print(*stream, nullptr);

#ifdef KAPRINO_EMIT_OBJECT_CODE

    EmitObjectCode(module);

#endif
}

#ifdef KAPRINO_EMIT_OBJECT_CODE

void EmitObjectCode(llvm::Module* module) {
    auto TargetTriple = llvm::sys::getDefaultTargetTriple();

    KAPRINO_LOG("Target: " << TargetTriple);

    llvm::InitializeAllTargetInfos();
    llvm::InitializeAllTargets();
    llvm::InitializeAllTargetMCs();
    llvm::InitializeAllAsmParsers();
    llvm::InitializeAllAsmPrinters();

    std::string Error;
    auto Target = llvm::TargetRegistry::lookupTarget(TargetTriple, Error);

    if (!Target) {
        KAPRINO_ERR(Error);
        return;
    }

    auto CPU = "generic";
    auto Features = "";

    llvm::TargetOptions opt;
    auto RM = llvm::Optional<llvm::Reloc::Model>();
    auto TargetMachine = Target->createTargetMachine(TargetTriple, CPU, Features, opt, RM);

    module->setDataLayout(TargetMachine->createDataLayout());
    module->setTargetTriple(TargetTriple);

    auto Filename = "output.o";
    std::error_code errorcode;
    llvm::raw_fd_ostream dest(Filename, errorcode);

    KAPRINO_LOG("Emit object file task: " << errorcode.message());

    llvm::legacy::PassManager pass;
    auto FileType = llvm::TargetMachine::CGFT_ObjectFile;

    if (TargetMachine->addPassesToEmitFile(pass, dest, nullptr, FileType)) {
        KAPRINO_ERR("TargetMachine can't emit a file of this type");
        return;
    }

    pass.run(*module);
    dest.flush();
}

#endif

#ifdef KAPRINO_OPTIMIZER_ON

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
