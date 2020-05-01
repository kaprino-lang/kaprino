#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"

class NumberExprObject : ExprObject {
   public:
    double value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto doubleVal = llvm::ConstantFP::get(KAPRINO_DOUBLE_TY(module), value);

        return doubleVal;
    }
};

antlrcpp::Any StatementVisitor::visitNumberExpr(KaprinoParser::NumberExprContext* ctx) {
    auto exprObj = new NumberExprObject();

    exprObj->value = atof(ctx->number()->getText().c_str());

    KAPRINO_LOG("Static value ditected: " << exprObj->value);

    return (ExprObject*)exprObj;
}
