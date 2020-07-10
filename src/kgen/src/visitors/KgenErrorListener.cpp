#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/KgenErrorListener.h"

namespace kaprino::kgen {

void KgenErrorListener::syntaxError(Recognizer* recognizer, Token* offendingSymbol, size_t line, size_t charPositionInLine, const std::string& msg, std::exception_ptr e) {
    logger->error(
        msg + " (" + recognizer->getInputStream()->getSourceName(),
        "internal",
        line,
        charPositionInLine + 1
    );
    throw - 1;
}

};  // namespace kaprino::kgen
