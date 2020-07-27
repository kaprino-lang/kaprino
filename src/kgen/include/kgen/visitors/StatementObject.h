#pragma once

#include "llvm/IR/IRBuilder.h"

#include "kgen/visitors/ContextObject.h"

namespace kaprino::kgen {

class StatementObject :public ContextObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {}
};

};  // namespace kaprino::kgen
