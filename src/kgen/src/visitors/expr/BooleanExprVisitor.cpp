#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class BooleanExprObject : public ExprObject {
   public:
    bool value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto boolVal = llvm::ConstantInt::get(KAPRINO_BOOL_TY(module), value);

        return boolVal;
    }
};

antlrcpp::Any StatementVisitor::visitBooleanExpr(KaprinoParser::BooleanExprContext* ctx) {
    auto exprObj = new BooleanExprObject();

    exprObj->setContextPosition(ctx);

    auto boolean = ctx->getText();
    std::transform(boolean.begin(), boolean.end(), boolean.begin(), (int (*)(int))std::tolower);
    exprObj->value = boolean == "true";

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
