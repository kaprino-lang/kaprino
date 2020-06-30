#include <stdlib.h>

#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "../../VariableManager.h"
#include "AssigneeObject.h"
#include "KaprinoParserBaseVisitor.h"

class ParameterAssigneeObject : AssigneeObject {
   public:
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto loaded = VariableManager::getptr(builder, module, name);
        return loaded;
    }
};

antlrcpp::Any StatementVisitor::visitParameterAssignee(KaprinoParser::ParameterAssigneeContext* ctx) {
    auto assigneeObj = new ParameterAssigneeObject();

    assigneeObj->name = ctx->ID()->getText();

    return (AssigneeObject*)assigneeObj;
}
