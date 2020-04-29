#include <iostream>

#include "TypeManager.h"
#include "KaprinoAccelerator.h"

bool TypeManager::exists(std::string name) {
    auto found = types.find(name);

    return found == types.end();
}

void TypeManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name, llvm::Type* type) {
    KAPRINO_LOG("Defined " << name);
    types[name] = type;
}

llvm::Type* TypeManager::gettype(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name) {
    auto found = types.find(name);

    if (found == types.end()) {
        KAPRINO_ERR("Try to access a type which doesn't exist");
        throw -1;
    }

    return found->second;
}

std::unordered_map<std::string, llvm::Type*> TypeManager::types;