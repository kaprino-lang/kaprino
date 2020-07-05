#include <memory>
#include <vector>

#include "../../DependencySolver.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "StatementObject.h"
#include "KaprinoParserBaseVisitor.h"

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

    KAPRINO_LOG("Begin reading package: " << package_name);

    statementObj->statements = DependencySolver::importRequirePackage(package_name);

    KAPRINO_LOG("End reading package: " << package_name);

    return (StatementObject*)statementObj;
}
