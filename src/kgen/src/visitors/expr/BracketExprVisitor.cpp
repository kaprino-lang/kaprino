#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class BracketExprObject : ExprObject {
   public:
    ExprObject* value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        return value->codegen(builder, module);
    }
};

antlrcpp::Any StatementVisitor::visitBracketExpr(KaprinoParser::BracketExprContext* ctx) {
    auto exprObj = new BracketExprObject();

    exprObj->value = visit(ctx->expr()).as<ExprObject*>();

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
