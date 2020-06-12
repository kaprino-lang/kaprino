#include <stdlib.h>

#include "../../abstructs/ExprObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

class BooleanExprObject : ExprObject {
   public:
    bool value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto boolVal = llvm::ConstantInt::get(KAPRINO_BOOL_TY(module), value);

        return boolVal;
    }
};

antlrcpp::Any StatementVisitor::visitBooleanExpr(KaprinoParser::BooleanExprContext* ctx) {
    auto exprObj = new BooleanExprObject();

    auto boolean = ctx->getText();
    std::transform(boolean.begin(), boolean.end(), boolean.begin(), (int (*)(int))std::tolower);
    exprObj->value = boolean == "true";

    KAPRINO_LOG("Static value ditected: " << boolean);

    return (ExprObject*)exprObj;
}
