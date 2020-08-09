#include "kgen/KgenAccelerator.h"
#include "kgen/internallib/InternalFuncs.h"
#include "kgen/visitors/AssigneeObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class ReadStatementObject : public StatementObject {
   public:
    AssigneeObject* assignee;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

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

    statementObj->setContextPosition(ctx);
    statementObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
