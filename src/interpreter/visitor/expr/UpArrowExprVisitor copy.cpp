#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../StatementVisitor.h"
#include "../../KaprinoAccelerator.h"
#include "../../internallib/InternalFuncs.h"

class UpArrowExprObject : ExprObject {
   public:
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        auto powFunc = get_pow(builder, module);
        return builder->CreateCall(powFunc, {l, r});
    }
};

antlrcpp::Any StatementVisitor::visitUpArrowExpr(KaprinoParser::UpArrowExprContext* ctx) {
    auto exprObj = new UpArrowExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();

    return (ExprObject*)exprObj;
}
