#pragma once

#include "antlr4-runtime.h"

namespace kaprino::kgen {

class ContextObject {
   public:
    virtual void setContextPosition(antlr4::ParserRuleContext* ctx) {
        line = ctx->getStart()->getLine();
        pos = ctx->getStart()->getStartIndex();
    }

   private:
    int line;
    int pos;
};

};  // namespace kaprino::kgen
