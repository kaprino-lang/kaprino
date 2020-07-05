#pragma once

#include "llvm/IR/IRBuilder.h"

class FunctionTypeObject {
   public:
    std::string outputType;
    std::vector<std::string> inputTypes;
};
