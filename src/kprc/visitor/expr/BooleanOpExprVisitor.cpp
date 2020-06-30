#include <stdlib.h>

#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "ExprObject.h"
#include "KaprinoParserBaseVisitor.h"

class BooleanOpExprObject : ExprObject {
   public:
    std::string op;
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        if (op == "and") {
            return builder->CreateAnd(left->codegen(builder, module), right->codegen(builder, module));
        }
        else if (op == "or"){
            return builder->CreateOr(left->codegen(builder, module), right->codegen(builder, module));
        }
        else {
            KAPRINO_ERR("Undefined operator: " << op);
            throw -1;
        }
    }
};

antlrcpp::Any StatementVisitor::visitBooleanOpExpr(KaprinoParser::BooleanOpExprContext* ctx) {
    auto exprObj = new BooleanOpExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->op = ctx->boolean_op()->getText();

    return (ExprObject*)exprObj;
}
