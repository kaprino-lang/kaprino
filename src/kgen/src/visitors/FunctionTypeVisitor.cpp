#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/FunctionTypeObject.h"
#include "kgen/visitors/StatementVisitor.h"

namespace kaprino::kgen {

antlrcpp::Any StatementVisitor::visitFunctionType(KaprinoParser::FunctionTypeContext *ctx) {
    auto obj = new FunctionTypeObject();

    auto types = ctx->ID();

    int size = types.size();
    for (int counter = 0; counter < size; counter++) {
        if (counter == size - 1) {
            obj->outputType = types[size - 1]->getText();
        } else {
            obj->inputTypes.push_back(types[counter]->getText());
        }
    }

    return obj;
}

};  // namespace kaprino::kgen
