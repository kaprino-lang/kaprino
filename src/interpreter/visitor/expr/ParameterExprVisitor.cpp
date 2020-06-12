#include <stdlib.h>

#include "../../abstructs/ExprObject.h"
#include "../../StatementVisitor.h"
#include "../../VariableManager.h"
#include "KaprinoParserBaseVisitor.h"

class ParameterExprObject : ExprObject {
   public:
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto valuePtr = VariableManager::getptr(builder, module, name);
        auto value = builder->CreateLoad(valuePtr);
        return value;
    }
};

antlrcpp::Any StatementVisitor::visitParameterExpr(KaprinoParser::ParameterExprContext* ctx) {
    auto exprObj = new ParameterExprObject();

    exprObj->name = ctx->name->getText();

    return (ExprObject*)exprObj;
}
