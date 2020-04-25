#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"

#define LLVM_INT8_PTR_TY llvm::Type::getInt8PtrTy(module->getContext())
#define LLVM_INT32_TY llvm::Type::getInt32Ty(module->getContext())

llvm::Function* internal_scanf;

llvm::Function* get_scanf(llvm::IRBuilder<>* builder, llvm::Module* module) {
    if (!internal_scanf) {
        std::vector<llvm::Type*> scanfFuncArgs;
        scanfFuncArgs.push_back(LLVM_INT8_PTR_TY);

        auto scanfFuncType = llvm::FunctionType::get(
            LLVM_INT32_TY,
            scanfFuncArgs,
            true);

        internal_scanf = llvm::Function::Create(scanfFuncType, llvm::GlobalValue::ExternalLinkage, "scanf", module);
        internal_scanf->setCallingConv(llvm::CallingConv::C);
    }

    return internal_scanf;
}

#undef LLVM_INT8_PTR_TY
#undef LLVM_INT32_TY