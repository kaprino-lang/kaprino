#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementVisitor.h"
#include "kgen/visitors/ExprObject.h"

namespace kaprino::kgen {

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

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
