#pragma once

#include <string>
#include <unordered_map>

#include "KaprinoAccelerator.h"

class TypeManager {
   public:
    static bool exists(std::string name);
    static void create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name, llvm::Type* type);
    static llvm::Type* gettype(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name);
   private:
    static std::unordered_map<std::string, llvm::Type*> types;
};
