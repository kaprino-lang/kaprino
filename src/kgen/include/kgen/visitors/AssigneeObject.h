#pragma once

#include "llvm/IR/IRBuilder.h"

namespace kaprino::kgen {

class AssigneeObject {
   public:
    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {
        return nullptr;
    }
};

};  // namespace kaprino::kgen
