#pragma once

#include <string>
#include <unordered_map>

#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/TypeManager.h"

namespace kaprino::kgen {

struct ParamInfo {
    std::string name;
    llvm::Value* alloca_ptr;
};

class VariableManager {
   public:
    static llvm::Value* create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Type* type);
    static void create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* allocated);
    static llvm::Value* getptr(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName);
    static void store(llvm::IRBuilder<>* builder, llvm::Module* module, std::string paramName, llvm::Value* value);
    static void add_scope();
    static void remove_scope();

   private:
    static int depth;
    static std::vector<ParamInfo> params;
};

};  // namespace kaprino::kgen
