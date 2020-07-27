#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class AddExprObject : public ExprObject {
   public:
    bool isPlus;
    ExprObject* left;
    ExprObject* right;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto l = left->codegen(builder, module);
        auto r = right->codegen(builder, module);
        if (KAPRINO_CONFIRM_INT64(module, l, r)) {
            return isPlus
                       ? builder->CreateAdd(l, r)
                       : builder->CreateSub(l, r);
        } else {
            l = KAPRINO_CAST_SI_FP(builder, module, l);
            r = KAPRINO_CAST_SI_FP(builder, module, r);
            return isPlus
                       ? builder->CreateFAdd(l, r)
                       : builder->CreateFSub(l, r);
        }
    }
};

antlrcpp::Any StatementVisitor::visitAddExpr(KaprinoParser::AddExprContext* ctx) {
    auto exprObj = new AddExprObject();

    exprObj->setContextPosition(ctx);
    exprObj->left = visit(ctx->expr(0)).as<ExprObject*>();
    exprObj->right = visit(ctx->expr(1)).as<ExprObject*>();
    exprObj->isPlus = ctx->add_op()->getText() == "+";

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
