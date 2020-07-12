#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

llvm::Function* internal_printf;

llvm::Function* get_printf(llvm::IRBuilder<>* builder, llvm::Module* module) {
    if (!internal_printf) {
        std::vector<llvm::Type*> printfFuncArgs;
        printfFuncArgs.push_back(KAPRINO_INT8_PTR_TY(module));

        auto printfFuncType = llvm::FunctionType::get(
            KAPRINO_INT32_TY(module),
            printfFuncArgs,
            true
        );

        internal_printf = llvm::Function::Create(printfFuncType, llvm::GlobalValue::ExternalLinkage, "printf", module);
        internal_printf->setCallingConv(llvm::CallingConv::C);
    }

    return internal_printf;
}

};
