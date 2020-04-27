#include <iostream>

#include "VariableManager.h"
#include "KaprinoAccelerator.h"

bool VariableManager::exists(std::string paramName) {
    auto found = params.find(paramName);

    return found == params.end();
}

void VariableManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::AllocaInst* allocated) {
    KAPRINO_LOG("Allocated " << paramName);
    params[paramName] = allocated;
}

llvm::Value* VariableManager::getptr(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName) {
    auto found = params.find(paramName);

    if (found == params.end()) {
        KAPRINO_ERR("Try to access a variable which doesn't exist");
        throw -1;
    }

    llvm::AllocaInst* allocated = found->second;
    return allocated;
}

void VariableManager::store(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* value) {
    auto found = params.find(paramName);

    if (found == params.end()) {
        KAPRINO_ERR("Try to access a variable which doesn't exist");
        throw -1;
    }

    llvm::AllocaInst* allocated = found->second;
    builder->CreateStore(value, allocated);
}

std::unordered_map<std::string, llvm::AllocaInst*> VariableManager::params;