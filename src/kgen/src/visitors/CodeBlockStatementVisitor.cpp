#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

antlrcpp::Any StatementVisitor::visitCodeBlockStatement(KaprinoParser::CodeBlockStatementContext* ctx) {
    auto statementObj = new std::vector<StatementObject*>();

    auto statements = ctx->statement();
    for (auto st : statements) {
        statementObj->push_back(visit(st).as<StatementObject*>());
    }

    return statementObj;
}

};  // namespace kaprino::kgen
