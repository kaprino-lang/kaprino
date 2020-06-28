#include <memory>
#include <vector>

#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "../../VariableManager.h"
#include "KaprinoParserBaseVisitor.h"

class LetStatementObject : StatementObject {
   public:
    std::string name;
    std::string type;
    ExprObject* initVal;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        llvm::AllocaInst* allocated;
        auto ty = TypeManager::gettype(builder, module, type);
        if (TypeManager::isDefaultType(builder, module, ty->getPointerTo())) {
            allocated = builder->CreateAlloca(ty);
        }
        else {
            auto content = builder->CreateAlloca(ty->getPointerElementType());
            allocated = builder->CreateAlloca(ty);
            builder->CreateStore(content, allocated);
        }
        VariableManager::create(builder, module, name, allocated);
        if (initVal) {
            auto init_val = initVal->codegen(builder, module);
            builder->CreateStore(init_val, allocated);
        }
    }
};

antlrcpp::Any StatementVisitor::visitLetStatement(KaprinoParser::LetStatementContext* ctx) {
    auto statementObj = new LetStatementObject();

    statementObj->name = ctx->name->getText();
    statementObj->type = ctx->types->getText();
    if (ctx->expr()) {
        statementObj->initVal = visit(ctx->expr()).as<ExprObject*>();
    }

    return (StatementObject*)statementObj;
}
