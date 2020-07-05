#include "KaprinoAccelerator.h"

using namespace antlr4;

class KaprinoErrorListener : public BaseErrorListener {
   public:
    virtual void syntaxError(Recognizer* recognizer, Token* offendingSymbol, size_t line, size_t charPositionInLine,
        const std::string& msg, std::exception_ptr e) override;
};
