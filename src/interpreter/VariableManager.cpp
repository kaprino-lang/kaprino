#include <iostream>

#include "VariableManager.h"
#include "KaprinoLogger.h"

bool VariableManager::exists(std::string paramName) {
    auto found = params.find(paramName);

    return found == params.end();
}

void VariableManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::AllocaInst* allocated) {
    KAPRINO_LOG("Allocated " << paramName);
    params[paramName] = allocated;
}

llvm::Value* VariableManager::load(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName) {
    auto found = params.find(paramName);

    if (found == params.end()) {
        KAPRINO_ERR("Try to access a variable which doesn't exist");
        return nullptr;
    }

    llvm::AllocaInst* allocated = found->second;
    auto loaded = builder->CreateLoad(allocated);

    return loaded;
}

void VariableManager::store(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* value) {
    auto found = params.find(paramName);

    if (found == params.end()) {
        KAPRINO_ERR("Try to access a variable which doesn't exist");
        return;
    }

    llvm::AllocaInst* allocated = found->second;
    builder->CreateStore(value, allocated);
}

std::unordered_map<std::string, llvm::AllocaInst*> VariableManager::params;