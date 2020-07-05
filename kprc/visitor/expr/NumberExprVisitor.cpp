#include <stdlib.h>

#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "ExprObject.h"
#include "KaprinoParserBaseVisitor.h"

class NumberExprObject : ExprObject {
   public:
    long long value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto llVal = llvm::ConstantInt::get(KAPRINO_INT64_TY(module), value);

        return llVal;
    }
};

antlrcpp::Any StatementVisitor::visitNumberExpr(KaprinoParser::NumberExprContext* ctx) {
    auto exprObj = new NumberExprObject();

    exprObj->value = atoll(ctx->number()->getText().c_str());

    KAPRINO_LOG("Static value ditected: " << exprObj->value);

    return (ExprObject*)exprObj;
}
