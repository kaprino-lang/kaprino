#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/FunctionManager.h"

namespace kaprino::kgen {

bool FunctionManager::exists(std::string funcName) {
    auto found = funcs.find(funcName);

    return found == funcs.end();
}

void FunctionManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string funcName, llvm::Function* func) {
    funcs[funcName] = func;
}

llvm::Function* FunctionManager::getfunc(llvm::IRBuilder<>* builder, llvm::Module* module, std::string funcName) {
    auto found = funcs.find(funcName);

    logger->asrt(
        found != funcs.end(),
        "Try to access a function which doesn't exist"
    );

    return found->second;
}

std::unordered_map<std::string, llvm::Function*> FunctionManager::funcs;

};  // namespace kaprino::kgen
