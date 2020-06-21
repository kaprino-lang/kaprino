#include "ClassStructureObject.h"

std::vector<llvm::Type*>* ClassStructureObject::getMemberTypes() {
    auto types = new std::vector<llvm::Type*>(params.size());
    std::transform(
        params.begin(),
        params.end(),
        types->begin(),
        [](ClassPrameterObject* param) { return param->type; }
    );
    return types;
}
