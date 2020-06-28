#include <memory>
#include <vector>

#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

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
