#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../StatementVisitor.h"
#include "../../KaprinoAccelerator.h"
#include "../../internallib/InternalFuncs.h"

class MulExprObject : ExprObject {
   public:
    bool isMul;
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        if (isMul)
            return builder->CreateFMul(l, r);
        else
            return builder->CreateFDiv(l, r);
    }
};

antlrcpp::Any StatementVisitor::visitMulExpr(KaprinoParser::MulExprContext* ctx) {
    auto exprObj = new MulExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->isMul = ctx->mul_op()->getText() == "*";

    return (ExprObject*)exprObj;
}
