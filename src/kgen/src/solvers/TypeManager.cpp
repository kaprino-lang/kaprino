#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/TypeManager.h"

namespace kaprino::kgen {

bool TypeManager::exists(std::string name) {
    auto found = types.find(name);

    return found == types.end();
}

void TypeManager::create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name, llvm::Type* type, ClassStructureObject* obj) {
    types[name] = {type, obj};
}

llvm::Type* TypeManager::gettype(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name) {
    auto found = types.find(name);

    logger->asrt(
        found == types.end(),
        "Try to access a type which doesn't exist",
        "internal",
        0,
        0
    );

    return found->second.first;
}

ClassStructureObject* TypeManager::getstructure(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name) {
    auto found = types.find(name);

    logger->asrt(
        found == types.end(),
        "Try to access a type which doesn't exist",
        "internal",
        0,
        0
    );

    return found->second.second;
}

bool TypeManager::isDefaultType(llvm::IRBuilder<>* builder, llvm::Module* module, llvm::Type* type) {
    return type == KAPRINO_BOOL_PTR_TY(module) ||
           type == KAPRINO_INT64_PTR_TY(module) ||
           type == KAPRINO_DOUBLE_PTR_TY(module);
}

std::unordered_map<std::string, std::pair<llvm::Type*, ClassStructureObject*>> TypeManager::types;

};  // namespace kaprino::kgen
