#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../VariableManager.h"
#include "../../StatementVisitor.h"

class ParameterExprObject : ExprObject {
   public:
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto value = VariableManager::load(builder, module, name);
        return value;
    }
};

antlrcpp::Any StatementVisitor::visitParameterExpr(KaprinoParser::ParameterExprContext* ctx) {
    auto exprObj = new ParameterExprObject();

    exprObj->name = ctx->name->getText();

    return (ExprObject*)exprObj;
}
