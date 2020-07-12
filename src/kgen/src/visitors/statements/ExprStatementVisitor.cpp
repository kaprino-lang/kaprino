#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class ExprStatementObject : StatementObject {
   public:
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = expr->codegen(builder, module);
    }
};

antlrcpp::Any StatementVisitor::visitExprStatement(KaprinoParser::ExprStatementContext* ctx) {
    auto statementObj = new ExprStatementObject();

    statementObj->expr = visit(ctx->expr()).as<ExprObject*>();

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
