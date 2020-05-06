#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../internallib/InternalFuncs.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"

class MulExprObject : ExprObject {
   public:
    bool isMul;
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        if (l->getType() == KAPRINO_INT64_TY(module)) {
            if (r->getType() == KAPRINO_INT64_TY(module)) {
                if (isMul)
                    return builder->CreateMul(l, r);
                else
                    return builder->CreateSDiv(l, r);
            }
            else {
                l = builder->CreateSIToFP(l, KAPRINO_DOUBLE_TY(module));
                if (isMul)
                    return builder->CreateFMul(l, r);
                else
                    return builder->CreateFDiv(l, r);
            }
        }
        else {
            if (r->getType() == KAPRINO_INT64_TY(module)) {
                r = builder->CreateSIToFP(r, KAPRINO_DOUBLE_TY(module));
                if (isMul)
                    return builder->CreateFMul(l, r);
                else
                    return builder->CreateFDiv(l, r);
            }
            else {
                if (isMul)
                    return builder->CreateFMul(l, r);
                else
                    return builder->CreateFDiv(l, r);
            }
        }
    }
};

antlrcpp::Any StatementVisitor::visitMulExpr(KaprinoParser::MulExprContext* ctx) {
    auto exprObj = new MulExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->isMul = ctx->mul_op()->getText() == "*";

    return (ExprObject*)exprObj;
}
