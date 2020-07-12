#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

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

    logger->log(
        "Static value ditected: " + boolean,
        "internal",
        0,
        0
    );

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
