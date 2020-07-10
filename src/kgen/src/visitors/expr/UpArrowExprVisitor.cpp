#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementVisitor.h"
#include "kgen/internallib/InternalFuncs.h"
#include "kgen/visitors/ExprObject.h"

namespace kaprino::kgen {

class UpArrowExprObject : ExprObject {
   public:
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        auto powFunc = get_pow(builder, module);
        if (l->getType() == KAPRINO_INT64_TY(module)) {
            l = builder->CreateSIToFP(l, KAPRINO_DOUBLE_TY(module));
        }
        if (r->getType() == KAPRINO_INT64_TY(module)) {
            r = builder->CreateSIToFP(r, KAPRINO_DOUBLE_TY(module));
        }
        return builder->CreateCall(powFunc, {l, r});
    }
};

antlrcpp::Any StatementVisitor::visitUpArrowExpr(KaprinoParser::UpArrowExprContext* ctx) {
    auto exprObj = new UpArrowExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
