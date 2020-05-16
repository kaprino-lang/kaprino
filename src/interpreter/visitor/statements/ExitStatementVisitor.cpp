#include <memory>
#include <vector>
#include <stack>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "../../VariableManager.h"

class LoopStatementObject : StatementObject {
   public:
    static std::stack<llvm::BasicBlock*>& comebackpos;
};

class ExitStatementObject : StatementObject {
   public:
    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto block = LoopStatementObject::comebackpos.top();
        LoopStatementObject::comebackpos.pop();
        builder->CreateBr(block);
        builder->SetInsertPoint((llvm::BasicBlock*)nullptr);
    }
};

antlrcpp::Any StatementVisitor::visitExitStatement(KaprinoParser::ExitStatementContext* ctx) {
    auto statementObj = new ExitStatementObject();

    return (StatementObject*)statementObj;
}
