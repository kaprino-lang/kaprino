#include <stdlib.h>

#include "../../abstructs/ExprObject.h"
#include "../../StatementVisitor.h"
#include "../../VariableManager.h"
#include "../../TypeManager.h"
#include "KaprinoParserBaseVisitor.h"

class ParameterExprObject : ExprObject {
   public:
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = VariableManager::getptr(builder, module, name);
        if (TypeManager::isDefaultType(builder, module, val)) {
            return builder->CreateLoad(val);
        }
        else {
            return val;
        }
    }
};

antlrcpp::Any StatementVisitor::visitParameterExpr(KaprinoParser::ParameterExprContext* ctx) {
    auto exprObj = new ParameterExprObject();

    exprObj->name = ctx->name->getText();

    return (ExprObject*)exprObj;
}
