#include "FunctionManager.h"
#include "KaprinoAccelerator.h"

bool FunctionManager::exists(std::string funcName) {
    auto found = funcs.find(funcName);

    return found == funcs.end();
}

void FunctionManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string funcName, llvm::Function* func) {
    KAPRINO_LOG("Defined " << funcName);
    funcs[funcName] = func;
}

llvm::Function* FunctionManager::getfunc(llvm::IRBuilder<>* builder, llvm::Module* module, std::string funcName) {
    auto found = funcs.find(funcName);

    if (found == funcs.end()) {
        KAPRINO_ERR("Try to access a function which doesn't exist");
        throw -1;
    }

    return found->second;
}

std::unordered_map<std::string, llvm::Function*> FunctionManager::funcs;