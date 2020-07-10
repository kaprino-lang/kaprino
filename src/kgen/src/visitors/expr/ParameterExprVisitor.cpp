#include "kgen/solvers/TypeManager.h"
#include "kgen/solvers/VariableManager.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class ParameterExprObject : ExprObject {
   public:
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = VariableManager::getptr(builder, module, name);
        return builder->CreateLoad(val);
    }
};

antlrcpp::Any StatementVisitor::visitParameterExpr(KaprinoParser::ParameterExprContext* ctx) {
    auto exprObj = new ParameterExprObject();

    exprObj->name = ctx->name->getText();

    return (ExprObject*)exprObj;
}

};  // namespace kaprino::kgen
