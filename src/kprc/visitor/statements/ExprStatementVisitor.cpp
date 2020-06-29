#include <memory>
#include <vector>

#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../expr/ExprObject.h"
#include "StatementObject.h"
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
