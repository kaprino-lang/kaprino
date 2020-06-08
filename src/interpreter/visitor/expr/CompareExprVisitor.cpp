#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"

class CompareExprObject : ExprObject {
   public:
    ExprObject* left;
    ExprObject* right;
    std::string op;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);

        if (op == ">") {
            return compare_greater(builder, module, l, r);
        }
        else if (op == "<") {
            return compare_greater(builder, module, r, l);
        }
        else if (op == "=") {
            return compare_equal(builder, module, l, r);
        }
        else if (op == ">=" || op == "=>") {
            return compare_eqgreater(builder, module, l, r);
        }
        else if (op == "<=" || op == "=<") {
            return compare_eqgreater(builder, module, r, l);
        }
        else {
            KAPRINO_ERR("Undefined operator: " << op);
            throw -1;
        }
    }

   private:
    llvm::Value* compare_greater(llvm::IRBuilder<>* builder, llvm::Module* module, llvm::Value* g, llvm::Value* l) {
        if (KAPRINO_CONFIRM_INT64(module, g, l)) {
            return builder->CreateICmpSGT(g, l);
        }
        else {
            g = KAPRINO_CAST_SI_FP(builder, module, g);
            l = KAPRINO_CAST_SI_FP(builder, module, l);
            return builder->CreateFCmpOGT(g, l);
        }
    }

    llvm::Value* compare_equal(llvm::IRBuilder<>* builder, llvm::Module* module, llvm::Value* g, llvm::Value* l) {
        if (KAPRINO_CONFIRM_INT64(module, g, l)) {
            return builder->CreateICmpEQ(g, l);
        }
        else {
            g = KAPRINO_CAST_SI_FP(builder, module, g);
            l = KAPRINO_CAST_SI_FP(builder, module, l);
            return builder->CreateFCmpOEQ(g, l);
        }
    }

    llvm::Value* compare_eqgreater(llvm::IRBuilder<>* builder, llvm::Module* module, llvm::Value* g, llvm::Value* l) {
        if (KAPRINO_CONFIRM_INT64(module, g, l)) {
            return builder->CreateICmpSGE(g, l);
        }
        else {
            g = KAPRINO_CAST_SI_FP(builder, module, g);
            l = KAPRINO_CAST_SI_FP(builder, module, l);
            return builder->CreateFCmpOGE(g, l);
        }
    }
};

antlrcpp::Any StatementVisitor::visitCompareExpr(KaprinoParser::CompareExprContext* ctx) {
    auto exprObj = new CompareExprObject();

    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->op = ctx->compare_op()->getText();

    return (ExprObject*)exprObj;
}
