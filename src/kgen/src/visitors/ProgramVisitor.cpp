#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

antlrcpp::Any StatementVisitor::visitProgram(KaprinoParser::ProgramContext *ctx) {
    auto statements = ctx->statement();

    auto statementsObj = new std::vector<StatementObject *>();

    logger->move_pos(0, 0);
    logger->log(
        "This program contains " + std::to_string(statements.size()) + "statement(s)"
    );

    for (auto statement : statements) {
        auto statementObjAny = visit(statement);

        auto statementObj = statementObjAny.as<StatementObject *>();
        statementsObj->push_back(statementObj);
    }

    return statementsObj;
}

};  // namespace kaprino::kgen
