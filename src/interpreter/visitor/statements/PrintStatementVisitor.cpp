#include <memory>
#include <vector>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../internallib/PrintfInternal.h"
#include "../../StatementVisitor.h"

class PrintStatementObject : StatementObject {
   public:
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto printfFunc = get_printf(builder, module);

        auto formatVal = builder->CreateGlobalStringPtr("%f\n");
        auto value = expr->codegen(builder, module);

        builder->CreateCall(printfFunc, { formatVal, value });
    }
};

antlrcpp::Any StatementVisitor::visitPrintStatement(KaprinoParser::PrintStatementContext* ctx) {
    auto statementObj = new PrintStatementObject();

    statementObj->expr = visit(ctx->expr()).as<ExprObject*>();

    return (StatementObject*)statementObj;
}
