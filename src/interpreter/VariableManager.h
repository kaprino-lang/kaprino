#pragma once

#include <string>
#include <unordered_map>

#include "KaprinoAccelerator.h"

struct ParamInfo {
    std::string name;
    llvm::AllocaInst* alloca_ptr;
};

class VariableManager {
   public:
    static void create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::AllocaInst* allocated);
    static llvm::Value* getptr(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName);
    static void store(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* value);
    static void add_scope();
    static void remove_scope();
   private:
    static int depth;
    static std::vector<ParamInfo> params;
};
