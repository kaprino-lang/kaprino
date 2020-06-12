#include <stdlib.h>

#include "../../abstructs/ExprObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

class NotExprObject : ExprObject {
   public:
    ExprObject* value;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        return builder->CreateNot(value->codegen(builder, module));
    }
};

antlrcpp::Any StatementVisitor::visitNotExpr(KaprinoParser::NotExprContext* ctx) {
    auto exprObj = new NotExprObject();

    exprObj->value = visit(ctx->expr()).as<ExprObject*>();

    return (ExprObject*)exprObj;
}
