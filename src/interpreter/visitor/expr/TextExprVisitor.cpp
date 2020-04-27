#include <stdlib.h>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/StatementObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../StatementVisitor.h"
#include "../../KaprinoAccelerator.h"

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

    KAPRINO_LOG("Static value ditected: " << exprObj->value);

    return (ExprObject*)exprObj;
}
