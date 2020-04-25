#include <memory>
#include <vector>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../VariableManager.h"
#include "../../StatementVisitor.h"
#include "../../internallib/InternalFuncs.h"
#include "../../abstructs/AssigneeObject.h"
#include "../../KaprinoLogger.h"

class ReadStatementObject : StatementObject {
   public:
    AssigneeObject* assignee;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = assignee->codegen(builder, module);
        auto scanfFunc = get_scanf(builder, module);
        if (val->getType() == llvm::Type::getDoublePtrTy(module->getContext())) {
            auto format = builder->CreateGlobalStringPtr("%lf");
            builder->CreateCall(scanfFunc, { format, val });
        }
        else {
            KAPRINO_ERR("An unexpected value was served to read");
            return;
        }
    }
};

antlrcpp::Any StatementVisitor::visitReadStatement(KaprinoParser::ReadStatementContext* ctx) {
    auto statementObj = new ReadStatementObject();

    statementObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();

    return (StatementObject*)statementObj;
}
