#pragma once

#include "kgen/visitors/ClassStructureObject.h"
#include "kgen/visitors/ContextObject.h"
#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class CStatementObject : public ContextObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module, ClassStructureObject* members) {}
};

};  // namespace kaprino::kgen
