#include <memory>
#include <vector>

#include "../../parser/KaprinoParserVisitor.h"
#include "../StatementVisitor.h"
#include "../abstructs/StatementObject.h"

antlrcpp::Any StatementVisitor::visitProgram(KaprinoParser::ProgramContext *ctx) {
    auto statements = ctx->statement();

    auto statementsObj = new std::vector<StatementObject*>();

    std::cout << "[LOG] This program contains " << statements.size() << "statement(s)" << std::endl;

    for (auto statement : statements) {
        auto statementObjAny = visit(statement);

        auto statementObj = statementObjAny.as<StatementObject*>();
        statementsObj->push_back(statementObj);
    }

    return statementsObj;
}
