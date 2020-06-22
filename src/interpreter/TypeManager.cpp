#include "TypeManager.h"
#include "KaprinoAccelerator.h"

bool TypeManager::exists(std::string name) {
    auto found = types.find(name);

    return found == types.end();
}

void TypeManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name, llvm::Type* type, ClassStructureObject* obj) {
    KAPRINO_LOG("Defined " << name);
    types[name] = {type, obj};
}

llvm::Type* TypeManager::gettype(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name) {
    auto found = types.find(name);

    if (found == types.end()) {
        KAPRINO_ERR("Try to access a type which doesn't exist");
        throw -1;
    }

    return found->second.first;
}

ClassStructureObject* TypeManager::getstructure(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name) {
    auto found = types.find(name);

    if (found == types.end()) {
        KAPRINO_ERR("Try to access a type which doesn't exist");
        throw - 1;
    }

    return found->second.second;
}

 bool TypeManager::isDefaultType(llvm::IRBuilder<>* builder, llvm::Module* module, llvm::Value* val) {
    return
        val->getType() == KAPRINO_BOOL_PTR_TY(module) ||
        val->getType() == KAPRINO_INT64_PTR_TY(module) ||
        val->getType() == KAPRINO_DOUBLE_PTR_TY(module);
}

std::unordered_map<std::string, std::pair<llvm::Type*, ClassStructureObject*>> TypeManager::types;