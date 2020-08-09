#include "kgen/solvers/TypeManager.h"
#include "kgen/solvers/VariableManager.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class LetStatementObject : public StatementObject {
   public:
    std::string name;
    std::string type;
    ExprObject* initVal;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto ty = TypeManager::gettype(builder, module, type);
        auto allocated = VariableManager::create(builder, module, name, ty);
        if (initVal) {
            auto init_val = initVal->codegen(builder, module);

            logger->asrt(
                allocated->getType() == init_val->getType() ||
                    allocated->getType()->getPointerElementType() == init_val->getType(),
                "Types are not matched"
            );

            builder->CreateStore(init_val, allocated);
        }
    }
};

antlrcpp::Any StatementVisitor::visitLetStatement(KaprinoParser::LetStatementContext* ctx) {
    auto statementObj = new LetStatementObject();

    statementObj->setContextPosition(ctx);
    statementObj->name = ctx->name->getText();
    statementObj->type = ctx->types->getText();
    if (ctx->expr()) {
        statementObj->initVal = visit(ctx->expr()).as<ExprObject*>();
    }

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
