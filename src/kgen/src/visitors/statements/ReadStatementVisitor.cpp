#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementVisitor.h"
#include "kgen/internallib/InternalFuncs.h"
#include "kgen/visitors/AssigneeObject.h"
#include "kgen/visitors/StatementObject.h"

namespace kaprino::kgen {

class ReadStatementObject : StatementObject {
   public:
    AssigneeObject* assignee;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = assignee->codegen(builder, module);
        auto scanfFunc = get_scanf(builder, module);
        if (val->getType() == KAPRINO_DOUBLE_PTR_TY(module)) {
            auto format = builder->CreateGlobalStringPtr("%lf");
            builder->CreateCall(scanfFunc, {format, val});
        } else if (val->getType() == KAPRINO_INT64_PTR_TY(module)) {
            auto format = builder->CreateGlobalStringPtr("%lld");
            builder->CreateCall(scanfFunc, {format, val});
        } else {
            logger->error(
                "An unexpected value was served to read",
                "internal",
                0,
                0
            );
            throw -1;
        }
    }
};

antlrcpp::Any StatementVisitor::visitReadStatement(KaprinoParser::ReadStatementContext* ctx) {
    auto statementObj = new ReadStatementObject();

    statementObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
