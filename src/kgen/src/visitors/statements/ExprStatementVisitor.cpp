#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class ExprStatementObject : public StatementObject {
   public:
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        expr->codegen(builder, module);
    }
};

antlrcpp::Any StatementVisitor::visitExprStatement(KaprinoParser::ExprStatementContext* ctx) {
    auto statementObj = new ExprStatementObject();

    statementObj->setContextPosition(ctx);
    statementObj->expr = visit(ctx->expr()).as<ExprObject*>();

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
