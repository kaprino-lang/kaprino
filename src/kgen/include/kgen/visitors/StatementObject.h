#pragma once

#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class StatementObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {}
};

};  // namespace kaprino::kgen
