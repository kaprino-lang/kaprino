#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class NumberExprObject : public ExprObject {
   public:
    long long value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto llVal = llvm::ConstantInt::get(KAPRINO_INT64_TY(module), value);

        return llVal;
    }
};

antlrcpp::Any StatementVisitor::visitNumberExpr(KaprinoParser::NumberExprContext* ctx) {
    auto exprObj = new NumberExprObject();

    exprObj->setContextPosition(ctx);
    exprObj->value = atoll(ctx->number()->getText().c_str());

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
