#include <memory>
#include <vector>

#include "../../parser/KaprinoParserBaseVisitor.h"
#include "../abstructs/ExprObject.h"
#include "../abstructs/StatementObject.h"
#include "../KaprinoAccelerator.h"
#include "../StatementVisitor.h"
#include "../TypeManager.h"
#include "../VariableManager.h"

antlrcpp::Any StatementVisitor::visitCodeBlockStatement(KaprinoParser::CodeBlockStatementContext* ctx) {
    auto statementObj = new std::vector<StatementObject*>();

    auto statements = ctx->statement();
    for (auto st : statements) {
        statementObj->push_back(visit(st).as<StatementObject*>());
    }

    return statementObj;
}
