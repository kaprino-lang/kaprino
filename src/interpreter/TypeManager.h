#pragma once

#include "KaprinoAccelerator.h"
#include "visitor/cstatements/ClassStructureObject.h"

class TypeManager {
   public:
    static bool exists(std::string name);
    static void create(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name, llvm::Type* type, ClassStructureObject* obj = nullptr);
    static llvm::Type* gettype(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name);
    static ClassStructureObject* getstructure(llvm::IRBuilder<>* builder, llvm::Module* module, std::string name);
    static bool isDefaultType(llvm::IRBuilder<>* builder, llvm::Module* module, llvm::Type* type);
   private:
    static std::unordered_map<std::string, std::pair<llvm::Type*, ClassStructureObject*>> types;
};
