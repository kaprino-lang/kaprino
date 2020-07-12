#pragma once

#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class ClassPrameterObject {
   public:
    std::string name;
    llvm::Type* type;
};

class ClassStructureObject {
   public:
    std::vector<ClassPrameterObject*> params;
    int getIndex(std::string id);
    std::vector<llvm::Type*>* getMemberTypes();
};

};  // namespace kaprino::kgen
