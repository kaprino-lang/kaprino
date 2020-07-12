#pragma once

#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class ExprObject {
   public:
    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {
        return nullptr;
    }
};

};  // namespace kaprino::kgen
