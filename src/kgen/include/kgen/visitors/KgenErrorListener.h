#pragma once

#include "kgen/KgenAccelerator.h"

using namespace antlr4;

namespace kaprino::kgen {

class KgenErrorListener : public BaseErrorListener {
   public:
    virtual void syntaxError(Recognizer* recognizer, Token* offendingSymbol, size_t line, size_t charPositionInLine, const std::string& msg, std::exception_ptr e) override;
};

};  // namespace kaprino::kgen
