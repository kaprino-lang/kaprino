#include "kgen/solvers/VariableManager.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

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

        auto ifblock = llvm::BasicBlock::Create(module->getContext(), "then" + std::to_string(branch_count), parent);
        auto mergeblock = llvm::BasicBlock::Create(module->getContext(), "merged" + std::to_string(branch_count), parent);

        if (hasOtherwise) {
            auto owblock = llvm::BasicBlock::Create(module->getContext(), "else" + std::to_string(branch_count), parent);
            builder->CreateCondBr(match, ifblock, owblock);

            branch_count++;

            builder->SetInsertPoint(owblock);
            VariableManager::add_scope();
            {
                for (auto st : *owst) {
                    st->codegen(builder, module);
                }
                builder->CreateBr(mergeblock);
            }
            VariableManager::remove_scope();
        } else {
            builder->CreateCondBr(match, ifblock, mergeblock);

            branch_count++;
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
    } else {
        statementObj->hasOtherwise = false;
    }

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
