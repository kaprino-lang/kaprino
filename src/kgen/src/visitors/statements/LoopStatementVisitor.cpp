#include "kgen/solvers/VariableManager.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class LoopStatementObject : public StatementObject {
   public:
    std::vector<StatementObject*>* body;

    static long long loop_count;
    static std::stack<llvm::BasicBlock*>& comebackpos;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto parent = builder->GetInsertBlock()->getParent();

        auto loopblock = llvm::BasicBlock::Create(module->getContext(), "loop" + std::to_string(loop_count), parent);
        auto mergeblock = llvm::BasicBlock::Create(module->getContext(), "lmerge" + std::to_string(loop_count), parent);

        loop_count++;

        comebackpos.push(mergeblock);

        builder->CreateBr(loopblock);

        builder->SetInsertPoint(loopblock);

        VariableManager::add_scope();
        {
            for (auto st : *body) {
                st->codegen(builder, module);
            }
            builder->CreateBr(loopblock);
        }
        VariableManager::remove_scope();

        if (!comebackpos.empty()) {
            comebackpos.pop();
        }

        builder->SetInsertPoint(mergeblock);
    }
};

long long LoopStatementObject::loop_count = 0LL;
std::stack<llvm::BasicBlock*>& LoopStatementObject::comebackpos =
    *(new std::stack<llvm::BasicBlock*>());

antlrcpp::Any StatementVisitor::visitLoopStatement(KaprinoParser::LoopStatementContext* ctx) {
    auto statementObj = new LoopStatementObject();

    statementObj->setContextPosition(ctx);
    statementObj->body = visit(ctx->codeblock()).as<std::vector<StatementObject*>*>();

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
