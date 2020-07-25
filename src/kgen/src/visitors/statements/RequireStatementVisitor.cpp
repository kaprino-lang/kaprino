#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/DependencySolver.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

std::vector<StatementObject*>* ParseFile(std::string text);

class RequireStatementObject : StatementObject {
   public:
    std::vector<StatementObject*>* statements;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        for (auto statement : *statements) {
            statement->codegen(builder, module);
        }
    }
};

antlrcpp::Any StatementVisitor::visitRequireStatement(KaprinoParser::RequireStatementContext* ctx) {
    auto statementObj = new RequireStatementObject();

    auto package_name = ctx->text()->getText();
    package_name = package_name.substr(1, package_name.size() - 2);

    logger->log(
        "Begin reading package: " + package_name,
        "internal",
        0,
        0
    );

    statementObj->statements = depsolver.importRequirePackage(package_name);

    logger->log(
        "End reading package: " + package_name,
        "internal",
        0,
        0
    );

    return (StatementObject*)statementObj;
}

};  // namespace kaprino::kgen
