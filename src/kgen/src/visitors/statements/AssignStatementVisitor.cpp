#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/AssigneeObject.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class AssignStatementObject : StatementObject {
   public:
    AssigneeObject* assignee;
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto ptr = assignee->codegen(builder, module);
        auto val = expr->codegen(builder, module);

        logger->asrt(
            ptr->getType()->getPointerTo() == val->getType(),
            "Types are not matched",
            "internal",
            0,
            0
        );

        builder->CreateStore(val, ptr);
    }
};

antlrcpp::Any StatementVisitor::visitAssignStatement(KaprinoParser::AssignStatementContext* ctx) {
    auto statementObj = new AssignStatementObject();

    statementObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();
    statementObj->expr = visit(ctx->assigner).as<ExprObject*>();

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
