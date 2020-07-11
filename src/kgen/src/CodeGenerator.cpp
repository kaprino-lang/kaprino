#include "kgen/CodeGenerator.h"
#include "kgen/solvers/DependencySolver.h"
#include "kgen/solvers/TypeManager.h"

namespace kaprino::kgen {

void CodeGenerator::generate(std::string input_file_path, llvm::IRBuilder<>* builder, llvm::Module* module) {
    auto input_text = getText(input_file_path);
    auto statements = DependencySolver::parseFile(input_text);

    generate(statements, builder, module);
}

void CodeGenerator::generate(std::vector<StatementObject*>* statements, llvm::IRBuilder<>* builder, llvm::Module* module) {
    std::vector<llvm::Type*> mainFuncArgs;

    auto mainFuncType = llvm::FunctionType::get(
        KAPRINO_INT32_TY(module),
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

    auto mainBlock = llvm::BasicBlock::Create(module->getContext(), "entry", module->getFunction("main"));
    builder->SetInsertPoint(mainBlock);

    TypeManager::create(builder, module, "Nil", KAPRINO_VOID_TY(module));
    TypeManager::create(builder, module, "Text", KAPRINO_INT8_PTR_TY(module));
    TypeManager::create(builder, module, "B", KAPRINO_BOOL_TY(module));
    TypeManager::create(builder, module, "R", KAPRINO_DOUBLE_TY(module));
    TypeManager::create(builder, module, "Z", KAPRINO_INT64_TY(module));

    for (auto statement : *statements) {
        statement->codegen(builder, module);
    }

    builder->CreateRet(llvm::ConstantInt::get(KAPRINO_INT32_TY(module), 0));

    removeUnreachableCode(builder, module);

    llvm::verifyModule(*module);
}

std::string CodeGenerator::getText(std::string file_path) {
    std::ifstream input_file(file_path);

    std::ostringstream ss;
    ss << input_file.rdbuf();
    std::string input_text;
    input_text = ss.str();

    return input_text;
}

void CodeGenerator::removeUnreachableCode(llvm::IRBuilder<>* builder, llvm::Module* module) {
    for (auto& func_it : *module) {
        for (auto& basic_block : func_it) {
            auto size = basic_block.size();
            if (size == 0) {
                builder->SetInsertPoint(&basic_block);
                builder->CreateUnreachable();
            }
        }
    }
}

}  // namespace kaprino::kgen
