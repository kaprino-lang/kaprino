#pragma once

#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ClassStructureObject.h"

namespace kaprino::kgen {

class CStatementObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module, ClassStructureObject* members) {}
};

};  // namespace kaprino::kgen
