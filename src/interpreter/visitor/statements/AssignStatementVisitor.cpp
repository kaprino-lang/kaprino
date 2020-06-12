#include <memory>
#include <vector>

#include "../../abstructs/AssigneeObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

class AssignStatementObject : StatementObject {
   public:
    AssigneeObject* assignee;
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto ptr = assignee->codegen(builder, module);
        auto val = expr->codegen(builder, module);
        builder->CreateStore(val, ptr);
    }
};

antlrcpp::Any StatementVisitor::visitAssignStatement(KaprinoParser::AssignStatementContext* ctx) {
    auto statementObj = new AssignStatementObject();

    statementObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();
    statementObj->expr = visit(ctx->assigner).as<ExprObject*>();

    return (StatementObject*)statementObj;
}
