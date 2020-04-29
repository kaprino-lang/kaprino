#include "../KaprinoAccelerator.h"

llvm::Function* internal_scanf;

llvm::Function* get_scanf(llvm::IRBuilder<>* builder, llvm::Module* module) {
    if (!internal_scanf) {
        std::vector<llvm::Type*> scanfFuncArgs;
        scanfFuncArgs.push_back(LLVM_INT8_PTR_TY(module));

        auto scanfFuncType = llvm::FunctionType::get(
            LLVM_INT32_TY(module),
            scanfFuncArgs,
            true
        );

        internal_scanf = llvm::Function::Create(scanfFuncType, llvm::GlobalValue::ExternalLinkage, "scanf", module);
        internal_scanf->setCallingConv(llvm::CallingConv::C);
    }

    return internal_scanf;
}