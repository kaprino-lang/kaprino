#include "../../abstructs/ExprObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "../cstatements/ClassStructureObject.h"
#include "KaprinoParserBaseVisitor.h"

class AccessExprObject : ExprObject {
   public:
    ExprObject* expr;
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto val = expr->codegen(builder, module);
        auto type = val->getType();
        auto type_name = type->getPointerElementType()->getStructName().str();
        auto structure = TypeManager::getstructure(builder, module, type_name);
        auto index = structure->getIndex(name);
        auto out = builder->CreateStructGEP(val, index);
        if (TypeManager::isDefaultType(builder, module, out)) {
            return builder->CreateLoad(out);
        }
        else {
            return out;
        }
    }
};

antlrcpp::Any StatementVisitor::visitAccessExpr(KaprinoParser::AccessExprContext* ctx) {
    auto exprObj = new AccessExprObject();

    exprObj->expr = visit(ctx->expr()).as<ExprObject*>();
    exprObj->name = ctx->ID()->getText();

    return (ExprObject*)exprObj;
}
