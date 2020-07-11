#pragma once

#include "kgen/visitors/StatementObject.h"
#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen
{

    class CodeGenerator {
       public:
        void generate(std::string file_path, llvm::IRBuilder<>* builder, llvm::Module* module);
        void generate(std::vector<StatementObject*>* statements, llvm::IRBuilder<>* builder, llvm::Module* module);

       private:
        std::string getText(std::string file_path);
        void removeUnreachableCode(llvm::IRBuilder<>* builder, llvm::Module* module);
    };

} // namespace kaprino::kgen
