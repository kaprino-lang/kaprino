#pragma once

#include <string>
#include <unordered_map>

#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Value.h"

class VariableManager {
   public:
    static bool exists(std::string paramName);
    static void create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::AllocaInst* allocated);
    static llvm::Value* getptr(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName);
    static void store(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* value);
   private:
    static std::unordered_map<std::string, llvm::AllocaInst*> params;
};
