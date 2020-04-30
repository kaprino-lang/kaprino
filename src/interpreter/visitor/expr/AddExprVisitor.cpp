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
        if (isPlus)
            return builder->CreateFAdd(l, r);
        else
            return builder->CreateFSub(l, r);
    }
};

antlrcpp::Any StatementVisitor::visitAddExpr(KaprinoParser::AddExprContext* ctx) {
    auto exprObj = new AddExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->isPlus = ctx->add_op()->getText() == "+";

    return (ExprObject*)exprObj;
}
