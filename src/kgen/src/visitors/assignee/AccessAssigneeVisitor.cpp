#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/TypeManager.h"
#include "kgen/visitors/AssigneeObject.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

class AccessAssigneeObject : public AssigneeObject {
   public:
    AssigneeObject* assignee;
    std::string name;

    virtual llvm::Value* codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        logger->move_pos(line, pos);

        auto ptr = assignee->codegen(builder, module);
        auto type = ptr->getType();
        if (TypeManager::isDefaultType(builder, module, type)) {
            logger->error(
                "The type named \"" + type->getStructName().str() + "\" has no members.",
                "internal",
                0,
                0
            );
            throw -1;
        }
        auto type_name = type->getPointerElementType()->getPointerElementType()->getStructName().str();
        auto index = TypeManager::getstructure(builder, module, type_name)->getIndex(name);

        auto out = builder->CreateStructGEP(builder->CreateLoad(ptr), index);
        return out;
    }
};

antlrcpp::Any StatementVisitor::visitAccessAssignee(KaprinoParser::AccessAssigneeContext* ctx) {
    auto assigneeObj = new AccessAssigneeObject();

    assigneeObj->setContextPosition(ctx);
    assigneeObj->assignee = visit(ctx->assignee()).as<AssigneeObject*>();
    assigneeObj->name = ctx->ID()->getText();

    return (AssigneeObject*)assigneeObj;
}

};  // namespace kaprino::kgen
