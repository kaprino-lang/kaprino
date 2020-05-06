#include <memory>
#include <vector>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/AssigneeObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../internallib/InternalFuncs.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../../VariableManager.h"

class ReadStatementObject : StatementObject {
   public:
    AssigneeObject* assignee;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = assignee->codegen(builder, module);
        auto scanfFunc = get_scanf(builder, module);
        if (val->getType() == KAPRINO_DOUBLE_PTR_TY(module)) {
            auto format = builder->CreateGlobalStringPtr("%lf");
            builder->CreateCall(scanfFunc, { format, val });
        }
        else if (val->getType() == KAPRINO_INT64_PTR_TY(module)) {
            auto format = builder->CreateGlobalStringPtr("%lld");
            builder->CreateCall(scanfFunc, { format, val });
        }
        else {
            KAPRINO_ERR("An unexpected value was served to read");
            throw -1;
        }
    }
};

antlrcpp::Any StatementVisitor::visitReadStatement(KaprinoParser::ReadStatementContext* ctx) {
    auto statementObj = new ReadStatementObject();

    statementObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();

    return (StatementObject*)statementObj;
}
