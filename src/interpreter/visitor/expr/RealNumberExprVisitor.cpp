#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"

class RealNumberExprObject : ExprObject {
   public:
    double value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto doubleVal = llvm::ConstantFP::get(KAPRINO_DOUBLE_TY(module), value);

        return doubleVal;
    }
};

antlrcpp::Any StatementVisitor::visitRealNumberExpr(KaprinoParser::RealNumberExprContext* ctx) {
    auto exprObj = new RealNumberExprObject();

    exprObj->value = atof(ctx->real_number()->getText().c_str());

    KAPRINO_LOG("Static value ditected: " << exprObj->value);

    return (ExprObject*)exprObj;
}
