#pragma once

#include "kgen/KgenAccelerator.h"

namespace kaprino::kprc {

class ExecutableGenerator {
   public:
    std::string emitLLVMIR(llvm::Module* module, bool optimize);
    std::string emitBitcode(llvm::Module* module, bool optimize);
    std::string emitExecutable(llvm::Module* module, bool optimize);
};

};  // namespace kaprino::kprc
