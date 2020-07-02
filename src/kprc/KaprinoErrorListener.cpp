#include "KaprinoErrorListener.h"

void KaprinoErrorListener::syntaxError(Recognizer* recognizer, Token* offendingSymbol, size_t line, size_t charPositionInLine,
    const std::string& msg, std::exception_ptr e)
{
    KAPRINO_ERR(msg << " [" << recognizer->getInputStream()->getSourceName() << " line:" << line << " pos:" << charPositionInLine + 1 << "]");
    throw -1;
}
