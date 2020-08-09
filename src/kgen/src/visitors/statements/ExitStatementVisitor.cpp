#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class LoopStatementObject : public StatementObject {
   public:
    static std::stack<llvm::BasicBlock*>& comebackpos;
};

class ExitStatementObject : public StatementObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto block = LoopStatementObject::comebackpos.top();
        builder->CreateBr(block);
        builder->SetInsertPoint((llvm::BasicBlock*)nullptr);
    }
};

antlrcpp::Any StatementVisitor::visitExitStatement(KaprinoParser::ExitStatementContext* ctx) {
    auto statementObj = new ExitStatementObject();

    statementObj->setContextPosition(ctx);

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
