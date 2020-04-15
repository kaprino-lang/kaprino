#include <stdlib.h>

#include "../../../parser/KaprinoParserVisitor.h"
#include "../../StatementVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"

class NumberExprObject : ExprObject {
   public:
    double value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto doubleTy = llvm::Type::getDoubleTy(module->getContext());

        return llvm::ConstantFP::get(doubleTy, value);
    }
};

antlrcpp::Any StatementVisitor::visitNumberExpr(KaprinoParser::NumberExprContext* ctx) {
    auto exprObj = new NumberExprObject();

    exprObj->value = atof(ctx->number()->getText().c_str());

    std::cout << "[LOG] Static value ditected: " << exprObj->value << std::endl;

    return (ExprObject*)exprObj;
}
