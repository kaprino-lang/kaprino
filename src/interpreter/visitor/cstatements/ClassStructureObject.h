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
    int getIndex(std::string id);
    std::vector<llvm::Type*>* getMemberTypes();
};

