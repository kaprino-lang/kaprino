#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class LoopStatementObject : public StatementObject {
   public:
    static std::stack<llvm::BasicBlock*>& comebackpos;
};

class SubmitStatementObject : public StatementObject {
   public:
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        LoopStatementObject::comebackpos = *(new std::stack<llvm::BasicBlock*>());
        if (expr) {
            builder->CreateRet(expr->codegen(builder, module));
            builder->SetInsertPoint((llvm::BasicBlock*)nullptr);
        } else {
            builder->CreateRetVoid();
            builder->SetInsertPoint((llvm::BasicBlock*)nullptr);
        }
    }
};

antlrcpp::Any StatementVisitor::visitSubmitStatement(KaprinoParser::SubmitStatementContext* ctx) {
    auto statementObj = new SubmitStatementObject();

    statementObj->setContextPosition(ctx);

    auto expr = ctx->expr();

    if (expr) {
        statementObj->expr = visit(expr).as<ExprObject*>();
    } else {
        statementObj->expr = nullptr;
    }

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
