#pragma once

#include "../../KaprinoAccelerator.h"
#include "ClassStructureObject.h"

class CStatementObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module, ClassStructureObject* members) { }
};
