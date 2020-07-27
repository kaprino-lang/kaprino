#pragma once

#include "kgen/visitors/ContextObject.h"

namespace kaprino::kgen {

class ExprObject : public ContextObject {
   public:
    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {
        return nullptr;
    }
};

};  // namespace kaprino::kgen
