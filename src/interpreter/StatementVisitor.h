#pragma once

#include "../parser/KaprinoParserBaseVisitor.h"

class StatementVisitor : public KaprinoParserBaseVisitor {
   public:
    virtual antlrcpp::Any visitProgram(KaprinoParser::ProgramContext *ctx) override;

    virtual antlrcpp::Any visitLetStatement(KaprinoParser::LetStatementContext* ctx) override;

    virtual antlrcpp::Any visitPrintStatement(KaprinoParser::PrintStatementContext *ctx) override;

    virtual antlrcpp::Any visitReadStatement(KaprinoParser::ReadStatementContext* ctx) override;

    virtual antlrcpp::Any visitNumberExpr(KaprinoParser::NumberExprContext *ctx) override;

    virtual antlrcpp::Any visitTextExpr(KaprinoParser::TextExprContext* ctx) override;

    virtual antlrcpp::Any visitParameterExpr(KaprinoParser::ParameterExprContext* ctx) override;

    virtual antlrcpp::Any visitParameterAssignee(KaprinoParser::ParameterAssigneeContext* ctx) override;
};