#pragma once

#include "llvm/IR/Module.h"

void EmitLLVMIR(llvm::Module* module, bool optimize);

void EmitExecutable(llvm::Module* module, bool optimize);

void Interpret(llvm::Module* module, bool optimize);