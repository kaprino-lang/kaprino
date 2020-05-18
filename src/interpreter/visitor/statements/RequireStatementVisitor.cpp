#include <memory>
#include <vector>

#include "../../../parser/KaprinoParserBaseVisitor.h"
#include "../../abstructs/AssigneeObject.h"
#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../internallib/InternalFuncs.h"
#include "../../KaprinoAccelerator.h"
#include "../../PackagePathManager.h"
#include "../../StatementVisitor.h"
#include "../../VariableManager.h"

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

    PackagePathManager pm(package_name);

    auto filename = pm.find();

    std::ifstream input_file(filename);

    std::ostringstream ss;
    ss << input_file.rdbuf();
    std::string input_text;
    input_text = ss.str();

    statementObj->statements = ParseFile(input_text);

    KAPRINO_LOG("End reading package: " << package_name);

    return (StatementObject*)statementObj;
}
