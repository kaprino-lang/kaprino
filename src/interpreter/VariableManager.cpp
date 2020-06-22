#include <iostream>

#include "VariableManager.h"
#include "KaprinoAccelerator.h"

void VariableManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::AllocaInst* allocated) {
    KAPRINO_LOG("Allocated " << paramName);

    params.push_back({ paramName, allocated });
}

llvm::AllocaInst* VariableManager::getptr(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName) {
    for (auto param : params) {
        if (param.name == paramName) {
            return param.alloca_ptr;
        }
    }

    KAPRINO_ERR("Try to access a variable which doesn't exist");
    throw -1;
}

void VariableManager::store(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* value) {
    for (auto param : params) {
        if (param.name == paramName) {
            builder->CreateStore(value, param.alloca_ptr);
            return;
        }
    }

    KAPRINO_ERR("Try to access a variable which doesn't exist");
    throw -1;
}

void VariableManager::add_scope() {
    depth++;
    params.push_back({ std::to_string(depth), nullptr });
}

void VariableManager::remove_scope() {
    while(params.back().name != std::to_string(depth)) {
        params.pop_back();
    }
    params.pop_back();
    depth--;
}

int VariableManager::depth = 0;
std::vector<ParamInfo> VariableManager::params;