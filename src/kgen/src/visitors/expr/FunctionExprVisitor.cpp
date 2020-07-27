#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/FunctionManager.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class FunctionExprObject : public ExprObject {
   public:
    std::string name;
    std::vector<ExprObject*> args;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

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

    exprObj->setContextPosition(ctx);
    exprObj->name = ctx->ID()->getText();
    for (auto expr : ctx->expr()) {
        exprObj->args.push_back(visit(expr).as<ExprObject*>());
    }

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
