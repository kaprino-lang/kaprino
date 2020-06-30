#pragma once

#include "llvm/IR/IRBuilder.h"

class StatementObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) { }
};
