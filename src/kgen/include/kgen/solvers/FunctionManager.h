#pragma once

#include <string>
#include <unordered_map>

#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class FunctionManager {
   public:
    static bool exists(std::string funcName);
    static void create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string funcName, llvm::Function* func);
    static llvm::Function* getfunc(llvm::IRBuilder<>* builder, llvm::Module* module, std::string funcName);

   private:
    static std::unordered_map<std::string, llvm::Function*> funcs;
};

};  // namespace kaprino::kgen
