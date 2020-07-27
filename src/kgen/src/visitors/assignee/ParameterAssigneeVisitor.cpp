#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/VariableManager.h"
#include "kgen/visitors/AssigneeObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class ParameterAssigneeObject : public AssigneeObject {
   public:
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto loaded = VariableManager::getptr(builder, module, name);
        return loaded;
    }
};

antlrcpp::Any StatementVisitor::visitParameterAssignee(KaprinoParser::ParameterAssigneeContext* ctx) {
    auto assigneeObj = new ParameterAssigneeObject();

    assigneeObj->setContextPosition(ctx);
    assigneeObj->name = ctx->ID()->getText();

    return (AssigneeObject*)assigneeObj;
}

};  // namespace kaprino::kgen
