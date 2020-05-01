#include <memory>
#include <vector>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "../../VariableManager.h"

class IfStatementObject : StatementObject {
   public:
    ExprObject* val;
    bool hasOtherwise;
    std::vector<StatementObject*>* ifst;
    std::vector<StatementObject*>* owst;

    static long long branch_count;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto match = val->codegen(builder, module);

        auto parent = builder->GetInsertBlock()->getParent();

        auto ifblock = llvm::BasicBlock::Create(module->getContext(), "if" + branch_count, parent);
        auto mergeblock = llvm::BasicBlock::Create(module->getContext(), "then" + branch_count, parent);

        if (hasOtherwise) {
            auto owblock = llvm::BasicBlock::Create(module->getContext(), "else" + branch_count, parent);
            builder->CreateCondBr(match, ifblock, owblock);

            builder->SetInsertPoint(owblock);
            VariableManager::add_scope();
            {
                for (auto st : *owst) {
                    st->codegen(builder, module);
                }
                builder->CreateBr(mergeblock);
            }
            VariableManager::remove_scope();
        }
        else {
            builder->CreateCondBr(match, ifblock, mergeblock);
        }

        builder->SetInsertPoint(ifblock);
        VariableManager::add_scope();
        {
            for (auto st : *ifst) {
                st->codegen(builder, module);
            }
            builder->CreateBr(mergeblock);
        }
        VariableManager::remove_scope();

        branch_count++;

        builder->SetInsertPoint(mergeblock);
    }
};

long long IfStatementObject::branch_count = 0LL;

antlrcpp::Any StatementVisitor::visitIfStatement(KaprinoParser::IfStatementContext* ctx) {
    auto statementObj = new IfStatementObject();

    statementObj->val = visit(ctx->expr()).as<ExprObject*>();

    auto codeblocks = ctx->codeblock();

    auto ifst = visit(codeblocks[0]).as<std::vector<StatementObject*>*>();
    statementObj->ifst = ifst;

    if (codeblocks.size() == 2) {
        auto owst = visit(codeblocks[1]).as<std::vector<StatementObject*>*>();
        statementObj->hasOtherwise = true;
        statementObj->owst = owst;
    }
    else {
        statementObj->hasOtherwise = false;
    }

    return (StatementObject*)statementObj;
}
