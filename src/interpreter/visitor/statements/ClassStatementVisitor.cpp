#include <memory>
#include <vector>

#include "../../abstructs/AssigneeObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "../cstatements/ClassStructureObject.h"
#include "../cstatements/CStatementObject.h"
#include "KaprinoParserBaseVisitor.h"

class ClassStatementObject : StatementObject {
   public:
    std::string name;
    std::vector<CStatementObject*>* statements;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto structure = new ClassStructureObject();
        auto size = statements->size();

        for (int counter = 0; counter < size; counter++) {
            (*statements)[counter]->codegen(builder, module, structure);
        }

        auto struct_obj = llvm::StructType::create(module->getContext(), name);
        struct_obj->setBody(*structure->getMemberTypes());

        TypeManager::create(builder, module, name, struct_obj, structure);
    }
};

antlrcpp::Any StatementVisitor::visitClassStatement(KaprinoParser::ClassStatementContext* ctx) {
    auto statementObj = new ClassStatementObject();
    statementObj->name = ctx->ID()->getText();
    statementObj->statements = new std::vector<CStatementObject*>();

    auto statements = ctx->classstatement();
    for (auto stmt : statements) {
        auto obj = visit(stmt).as<CStatementObject*>();
        statementObj->statements->push_back(obj);
    }

    return (StatementObject*)statementObj;
}
