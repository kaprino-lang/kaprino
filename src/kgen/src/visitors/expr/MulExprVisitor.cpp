#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class MulExprObject : public ExprObject {
   public:
    bool isMul;
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        if (KAPRINO_CONFIRM_INT64(module, l, r)) {
            return isMul
                       ? builder->CreateMul(l, r)
                       : builder->CreateSDiv(l, r);
        } else {
            l = KAPRINO_CAST_SI_FP(builder, module, l);
            r = KAPRINO_CAST_SI_FP(builder, module, r);
            return isMul
                       ? builder->CreateFMul(l, r)
                       : builder->CreateFDiv(l, r);
        }
    }
};

antlrcpp::Any StatementVisitor::visitMulExpr(KaprinoParser::MulExprContext* ctx) {
    auto exprObj = new MulExprObject();

    exprObj->setContextPosition(ctx);
    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->isMul = ctx->mul_op()->getText() == "*";

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
