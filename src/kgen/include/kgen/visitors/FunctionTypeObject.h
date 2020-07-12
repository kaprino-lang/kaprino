#pragma once

#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

class FunctionTypeObject {
   public:
    std::string outputType;
    std::vector<std::string> inputTypes;
};

};  // namespace kaprino::kgen
