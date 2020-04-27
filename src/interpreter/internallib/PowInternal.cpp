#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"

llvm::Function* internal_pow;

llvm::Function* get_pow(llvm::IRBuilder<>* builder, llvm::Module* module) {
    if (!internal_pow) {
        auto double_type = llvm::Type::getDoubleTy(module->getContext());

        std::vector<llvm::Type*> internal_pow_args;
        internal_pow_args.push_back(double_type);
        internal_pow_args.push_back(double_type);

        auto pow_type = llvm::FunctionType::get(
            double_type,
            internal_pow_args,
            true
        );

        internal_pow = llvm::Function::Create(pow_type, llvm::GlobalValue::ExternalLinkage, "pow", module);
        internal_pow->setCallingConv(llvm::CallingConv::C);
    }

    return internal_pow;
}