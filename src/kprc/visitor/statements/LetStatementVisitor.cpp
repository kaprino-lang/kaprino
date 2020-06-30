#include <memory>
#include <vector>

#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "../../VariableManager.h"
#include "../expr/ExprObject.h"
#include "StatementObject.h"
#include "KaprinoParserBaseVisitor.h"

class LetStatementObject : StatementObject {
   public:
    std::string name;
    std::string type;
    ExprObject* initVal;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto ty = TypeManager::gettype(builder, module, type);
        auto allocated = VariableManager::create(builder, module, name, ty);
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
