#include <memory>
#include <vector>

#include "../../abstructs/ExprObject.h"
#include "../../abstructs/StatementObject.h"
#include "../../internallib/InternalFuncs.h"
#include "../../KaprinoAccelerator.h"
#include "../../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

class PrintStatementObject : StatementObject {
   public:
    std::vector<ExprObject*> exprs;

    virtual void codegen(llvm::IRBuilder<>* builder, llvm::Module* module) override {
        auto printfFunc = get_printf(builder, module);

        std::string formatText;
        std::vector<llvm::Value*> args;

        for (auto expr : exprs) {
            auto value = expr->codegen(builder, module);
            argconv(value, module, formatText, args);
        }

        formatText += "\n";
        auto formatVal = builder->CreateGlobalStringPtr(formatText);

        args.insert(args.begin(), formatVal);

        builder->CreateCall(printfFunc, args);
    }

   private:
    void argconv(llvm::Value* val, llvm::Module* module, std::string& format, std::vector<llvm::Value*>& args) {
        auto type = val->getType();
        if (type == KAPRINO_DOUBLE_TY(module)) {
            format += "%f";
            args.push_back(val);
        }
        else if (type == KAPRINO_INT64_TY(module)) {
            format += "%lld";
            args.push_back(val);
        }
        else if (type == KAPRINO_INT8_PTR_TY(module)) {
            format += "%s";
            args.push_back(val);
        }
        else {
            KAPRINO_ERR("An unexpected value was served to print");
            throw -1;
        }
    }
};

antlrcpp::Any StatementVisitor::visitPrintStatement(KaprinoParser::PrintStatementContext* ctx) {
    auto statementObj = new PrintStatementObject();

    auto exprs = ctx->expr();
    for (auto expr : exprs) {
        statementObj->exprs.push_back(visit(expr).as<ExprObject*>());
    }

    return (StatementObject*)statementObj;
}
