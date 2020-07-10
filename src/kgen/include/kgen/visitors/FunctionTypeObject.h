#pragma once

#include "llvm/IR/IRBuilder.h"

namespace kaprino::kgen {

class FunctionTypeObject {
   public:
    std::string outputType;
    std::vector<std::string> inputTypes;
};

};  // namespace kaprino::kgen
