#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class TextExprObject : public ExprObject {
   public:
    std::string value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto val = builder->CreateGlobalStringPtr(value);
        return val;
    }
};

antlrcpp::Any StatementVisitor::visitTextExpr(KaprinoParser::TextExprContext* ctx) {
    auto exprObj = new TextExprObject();

    exprObj->setContextPosition(ctx);

    auto text = ctx->text()->getText();
    exprObj->value = text.substr(1, text.size() - 2);

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
