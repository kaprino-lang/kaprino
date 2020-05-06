#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../internallib/InternalFuncs.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"

class AddExprObject : ExprObject {
   public:
    bool isPlus;
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        if (l->getType() == KAPRINO_INT64_TY(module)) {
            if (r->getType() == KAPRINO_INT64_TY(module)) {
                if (isPlus)
                    return builder->CreateAdd(l, r);
                else
                    return builder->CreateSub(l, r);
            }
            else {
                l = builder->CreateSIToFP(l, KAPRINO_DOUBLE_TY(module));
                if (isPlus)
                    return builder->CreateFAdd(l, r);
                else
                    return builder->CreateFSub(l, r);
            }
        }
        else {
            if (r->getType() == KAPRINO_INT64_TY(module)) {
                r = builder->CreateSIToFP(r, KAPRINO_DOUBLE_TY(module));
                if (isPlus)
                    return builder->CreateFAdd(l, r);
                else
                    return builder->CreateFSub(l, r);
            }
            else {
                if (isPlus)
                    return builder->CreateFAdd(l, r);
                else
                    return builder->CreateFSub(l, r);
            }
        }
    }
};

antlrcpp::Any StatementVisitor::visitAddExpr(KaprinoParser::AddExprContext* ctx) {
    auto exprObj = new AddExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->isPlus = ctx->add_op()->getText() == "+";

    return (ExprObject*)exprObj;
}
