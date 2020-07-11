#pragma once

#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"

llvm::Function* get_printf(llvm::IRBuilder<>* builder, llvm::Module* module);

llvm::Function* get_scanf(llvm::IRBuilder<>* builder, llvm::Module* module);

llvm::Function* get_pow(llvm::IRBuilder<>* builder, llvm::Module* module);