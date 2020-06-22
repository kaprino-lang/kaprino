#include <stdlib.h>

#include "../../abstructs/AssigneeObject.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../../TypeManager.h"
#include "KaprinoParserBaseVisitor.h"

class AccessAssigneeObject : AssigneeObject {
   public:
    AssigneeObject* assignee;
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto ptr = assignee->codegen(builder, module);
        auto type = ptr->getType();
        auto type_name = type->getPointerElementType()->getStructName().str();
        auto structure = TypeManager::getstructure(builder, module, type_name);
        auto index = structure->getIndex(name);
        auto out = builder->CreateStructGEP(ptr, index);
        return out;
    }
};

antlrcpp::Any StatementVisitor::visitAccessAssignee(KaprinoParser::AccessAssigneeContext* ctx) {
    auto assigneeObj = new AccessAssigneeObject();

    assigneeObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();
    assigneeObj->name = ctx->ID()->getText();

    return (AssigneeObject*)assigneeObj;
}
