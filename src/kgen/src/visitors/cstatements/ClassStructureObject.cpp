#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ClassStructureObject.h"

namespace kaprino::kgen {

int ClassStructureObject::getIndex(std::string id) {
    int size = params.size();
    for (int index = 0; index < size; index++) {
        if (params[index]->name == id) {
            return index;
        }
    }
    logger->error(
        "The member named \"" + id + "\" was not found",
        "internal",
        0,
        0
    );
    throw -1;
}

std::vector<llvm::Type*>* ClassStructureObject::getMemberTypes() {
    auto types = new std::vector<llvm::Type*>(params.size());
    std::transform(
        params.begin(),
        params.end(),
        types->begin(),
        [](ClassPrameterObject* param) { return param->type; });
    return types;
}

};  // namespace kaprino::kgen
