#pragma once

#include "KaprinoParserBaseVisitor.h"

class StatementVisitor : public KaprinoParserBaseVisitor {
   public:
    virtual antlrcpp::Any visitProgram(KaprinoParser::ProgramContext *ctx) override;

    virtual antlrcpp::Any visitLetStatement(KaprinoParser::LetStatementContext* ctx) override;

    virtual antlrcpp::Any visitAssignStatement(KaprinoParser::AssignStatementContext* ctx) override;

    virtual antlrcpp::Any visitExprStatement(KaprinoParser::ExprStatementContext* ctx) override;

    virtual antlrcpp::Any visitDefineFunctionStatement(KaprinoParser::DefineFunctionStatementContext* ctx) override;

    virtual antlrcpp::Any visitDefineProcessStatement(KaprinoParser::DefineProcessStatementContext* ctx) override;

    virtual antlrcpp::Any visitIfStatement(KaprinoParser::IfStatementContext* ctx) override;

    virtual antlrcpp::Any visitLoopStatement(KaprinoParser::LoopStatementContext* ctx) override;

    virtual antlrcpp::Any visitSubmitStatement(KaprinoParser::SubmitStatementContext* ctx) override;

    virtual antlrcpp::Any visitExitStatement(KaprinoParser::ExitStatementContext* ctx) override;

    virtual antlrcpp::Any visitPrintStatement(KaprinoParser::PrintStatementContext *ctx) override;

    virtual antlrcpp::Any visitReadStatement(KaprinoParser::ReadStatementContext* ctx) override;

    virtual antlrcpp::Any visitRequireStatement(KaprinoParser::RequireStatementContext* ctx) override;

    virtual antlrcpp::Any visitCodeBlockStatement(KaprinoParser::CodeBlockStatementContext* ctx) override;

    virtual antlrcpp::Any visitBracketExpr(KaprinoParser::BracketExprContext* ctx) override;

    virtual antlrcpp::Any visitUpArrowExpr(KaprinoParser::UpArrowExprContext* ctx) override;

    virtual antlrcpp::Any visitMulExpr(KaprinoParser::MulExprContext* ctx) override;

    virtual antlrcpp::Any visitAddExpr(KaprinoParser::AddExprContext* ctx) override;

    virtual antlrcpp::Any visitCompareExpr(KaprinoParser::CompareExprContext* ctx) override;

    virtual antlrcpp::Any visitBooleanOpExpr(KaprinoParser::BooleanOpExprContext* ctx) override;

    virtual antlrcpp::Any visitNotExpr(KaprinoParser::NotExprContext* ctx) override;

    virtual antlrcpp::Any visitNumberExpr(KaprinoParser::NumberExprContext *ctx) override;

    virtual antlrcpp::Any visitRealNumberExpr(KaprinoParser::RealNumberExprContext* ctx) override;

    virtual antlrcpp::Any visitTextExpr(KaprinoParser::TextExprContext* ctx) override;

    virtual antlrcpp::Any visitBooleanExpr(KaprinoParser::BooleanExprContext* ctx) override;

    virtual antlrcpp::Any visitParameterExpr(KaprinoParser::ParameterExprContext* ctx) override;

    virtual antlrcpp::Any visitFunctionExpr(KaprinoParser::FunctionExprContext* ctx) override;

    virtual antlrcpp::Any visitFunctionType(KaprinoParser::FunctionTypeContext* ctx) override;

    virtual antlrcpp::Any visitParameterAssignee(KaprinoParser::ParameterAssigneeContext* ctx) override;
};