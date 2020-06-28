#include <memory>
#include <vector>

#include "../abstructs/FunctionTypeObject.h"
#include "../KaprinoAccelerator.h"
#include "../StatementVisitor.h"
#include "KaprinoParserBaseVisitor.h"

antlrcpp::Any StatementVisitor::visitFunctionType(KaprinoParser::FunctionTypeContext *ctx) {
    auto obj = new FunctionTypeObject();

    auto types = ctx->ID();

    int size = types.size();
    for (int counter = 0; counter < size; counter++) {
        if(counter == size - 1) {
            obj->outputType = types[size - 1]->getText();
        }
        else {
            obj->inputTypes.push_back(types[counter]->getText());
        }
    }

    return obj;
}
