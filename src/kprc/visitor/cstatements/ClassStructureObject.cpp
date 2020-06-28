#include "ClassStructureObject.h"

int ClassStructureObject::getIndex(std::string id) {
    int size = params.size();
    for (int index = 0; index < size; index++) {
        if (params[index]->name == id) {
            return index;
        }
    }
    KAPRINO_ERR("The member named \"" << id << "\" was not found");
    throw -1;
}

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
