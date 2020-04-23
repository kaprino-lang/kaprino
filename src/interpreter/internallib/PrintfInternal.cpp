#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"

#define LLVM_INT8_PTR_TY llvm::Type::getInt8PtrTy(module->getContext())
#define LLVM_INT32_TY llvm::Type::getInt32Ty(module->getContext())

llvm::Function* internal_printf;

llvm::Function* get_printf(llvm::IRBuilder<>* builder, llvm::Module* module) {
    if (!internal_printf) {
        std::vector<llvm::Type*> printfFuncArgs;
        printfFuncArgs.push_back(LLVM_INT8_PTR_TY);

        auto printfFuncType = llvm::FunctionType::get(
            LLVM_INT32_TY,
            printfFuncArgs,
            true);

        internal_printf = llvm::Function::Create(printfFuncType, llvm::GlobalValue::ExternalLinkage, "printf", module);
        internal_printf->setCallingConv(llvm::CallingConv::C);
    }

    return internal_printf;
}

#undef LLVM_INT8_PTR_TY
#undef LLVM_INT32_TY