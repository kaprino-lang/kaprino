#include <stdlib.h>

#include "../../abstructs/ExprObject.h"
#include "../../FunctionManager.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

class FunctionExprObject : ExprObject {
   public:
    std::string name;
    std::vector<ExprObject*> args;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto func = FunctionManager::getfunc(builder, module, name);
        std::vector<llvm::Value*> argVals;
        for (auto arg : args) {
            argVals.push_back(arg->codegen(builder, module));
        }
        auto ret = builder->CreateCall(func, argVals);
        return ret;
    }
};

antlrcpp::Any StatementVisitor::visitFunctionExpr(KaprinoParser::FunctionExprContext* ctx) {
    auto exprObj = new FunctionExprObject();

    exprObj->name = ctx->ID()->getText();
    for (auto expr : ctx->expr()) {
        exprObj->args.push_back(visit(expr).as<ExprObject*>());
    }

    return (ExprObject*)exprObj;
}
