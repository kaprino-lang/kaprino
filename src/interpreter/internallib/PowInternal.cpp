#include "../KaprinoAccelerator.h"

llvm::Function* internal_pow;

llvm::Function* get_pow(llvm::IRBuilder<>* builder, llvm::Module* module) {
    if (!internal_pow) {
        std::vector<llvm::Type*> internal_pow_args;
        internal_pow_args.push_back(LLVM_DOUBLE_TY(module));
        internal_pow_args.push_back(LLVM_DOUBLE_TY(module));

        auto pow_type = llvm::FunctionType::get(
            LLVM_DOUBLE_TY(module),
            internal_pow_args,
            true
        );

        internal_pow = llvm::Function::Create(pow_type, llvm::GlobalValue::ExternalLinkage, "pow", module);
        internal_pow->setCallingConv(llvm::CallingConv::C);
    }

    return internal_pow;
}