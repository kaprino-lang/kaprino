#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class TextExprObject : ExprObject {
   public:
    std::string value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = builder->CreateGlobalStringPtr(value);

        return val;
    }
};

antlrcpp::Any StatementVisitor::visitTextExpr(KaprinoParser::TextExprContext* ctx) {
    auto exprObj = new TextExprObject();

    auto text = ctx->text()->getText();
    exprObj->value = text.substr(1, text.size() - 2);

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
