#pragma once

#include "../../KaprinoAccelerator.h"

class ClassPrameterObject {
   public:
    std::string name;
    llvm::Type* type;
};

class ClassStructureObject {
   public:
    std::vector<ClassPrameterObject*> params;
    std::vector<llvm::Type*>* getMemberTypes();
};

