#include "kgen/solvers/TypeManager.h"
#include "kgen/solvers/VariableManager.h"
#include "kgen/visitors/CStatementObject.h"
#include "kgen/visitors/ClassStructureObject.h"
#include "kgen/visitors/ExprObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class LetCStatementObject : CStatementObject {
   public:
    std::string name;
    std::string type;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module, ClassStructureObject* members) override {
        auto type_obj = TypeManager::gettype(builder, module, type);
        auto param_obj = new ClassPrameterObject();
        param_obj->name = name;
        param_obj->type = type_obj;
        members->params.push_back(param_obj);
    }
};

antlrcpp::Any StatementVisitor::visitLetCStatement(KaprinoParser::LetCStatementContext* ctx) {
    auto statementObj = new LetCStatementObject();

    statementObj->name = ctx->name->getText();
    statementObj->type = ctx->types->getText();

    return (CStatementObject*)statementObj;
}

};  // namespace kaprino::kgen
