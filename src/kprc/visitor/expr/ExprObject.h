#pragma once

#include "llvm/IR/IRBuilder.h"

class ExprObject {
   public:
    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {
        return nullptr;
    }
};
