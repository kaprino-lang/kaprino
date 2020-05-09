#pragma once

#include "llvm/IR/Module.h"

void EmitLLVMIR(llvm::Module* module, bool optimize);

std::string EmitObjectCode(llvm::Module* module, bool optimize);

std::string EmitExecutable(llvm::Module* module, bool optimize);
