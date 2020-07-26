#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/KgenErrorListener.h"

namespace kaprino::kgen {

void KgenErrorListener::syntaxError(Recognizer* recognizer, Token* offendingSymbol, size_t line, size_t charPositionInLine, const std::string& msg, std::exception_ptr e) {
    logger->move_pos(line, charPositionInLine + 1);
    logger->error("Unknown syntax was found. If you want us to support this syntax, please make PR :P");
    throw -1;
}

};  // namespace kaprino::kgen
