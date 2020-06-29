#include <memory>
#include <vector>

#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../expr/ExprObject.h"
#include "StatementObject.h"
#include "KaprinoParserBaseVisitor.h"

class LoopStatementObject : StatementObject {
   public:
    static std::stack<llvm::BasicBlock*>& comebackpos;
};

class SubmitStatementObject : StatementObject {
   public:
    ExprObject* expr;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        LoopStatementObject::comebackpos = *(new std::stack<llvm::BasicBlock*>());
        if (expr) {
            builder->CreateRet(expr->codegen(builder, module));
            builder->SetInsertPoint((llvm::BasicBlock*)nullptr);
        }
        else {
            builder->CreateRetVoid();
            builder->SetInsertPoint((llvm::BasicBlock*)nullptr);
        }
    }
};

antlrcpp::Any StatementVisitor::visitSubmitStatement(KaprinoParser::SubmitStatementContext* ctx) {
    auto statementObj = new SubmitStatementObject();

    auto expr = ctx->expr();

    if (expr) {
        statementObj->expr = visit(expr).as<ExprObject*>();
    }
    else {
        statementObj->expr = nullptr;
    }

    return (StatementObject*)statementObj;
}
