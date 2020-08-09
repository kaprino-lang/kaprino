#pragma once

#include "kgen/visitors/ContextObject.h"
#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class AssigneeObject : public ContextObject {
   public:
    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) {
        return nullptr;
    }
};

};  // namespace kaprino::kgen
