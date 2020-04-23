#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"

llvm::Function* get_printf(llvm::IRBuilder<>* builder, llvm::Module* module);