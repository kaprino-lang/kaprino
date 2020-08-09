#pragma once

#include <filesystem>
#include <iostream>
#include <memory>
#include <ostream>
#include <random>
#include <sstream>
#include <stack>
#include <string>
#include <unordered_map>
#include <vector>

#include "antlr4-runtime.h"

#include "llvm/ADT/Optional.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/Bitcode/BitcodeWriter.h"
#include "llvm/ExecutionEngine/ExecutionEngine.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/IR/Verifier.h"
#include "llvm/Pass.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/InitLLVM.h"
#include "llvm/Support/TargetRegistry.h"
#include "llvm/Support/TargetSelect.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/Target/TargetOptions.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"

#include "KaprinoLexer.h"
#include "KaprinoParser.h"

#include "kgen/notifications/NotificationManager.h"
#include "kgen/solvers/DependencySolver.h"

namespace kaprino::kgen {

//
// ------ Log system ------
//

#define ANSI_YELLOW_CODE "\u001b[33;1m"
#define ANSI_RED_CODE "\u001b[31;1m"
#define ANSI_CLEAN_CODE "\u001b[0m"

//
// ------ LLVM type ------
//

#define KAPRINO_VOID_TY(mod) llvm::Type::getVoidTy((mod)->getContext())
#define KAPRINO_BOOL_TY(mod) llvm::Type::getInt1Ty((mod)->getContext())
#define KAPRINO_BOOL_PTR_TY(mod) llvm::Type::getInt1PtrTy((mod)->getContext())
#define KAPRINO_INT8_TY(mod) llvm::Type::getInt8Ty((mod)->getContext())
#define KAPRINO_INT8_PTR_TY(mod) llvm::Type::getInt8PtrTy((mod)->getContext())
#define KAPRINO_INT32_TY(mod) llvm::Type::getInt32Ty((mod)->getContext())
#define KAPRINO_INT32_PTR_TY(mod) llvm::Type::getInt32PtrTy((mod)->getContext())
#define KAPRINO_INT64_TY(mod) llvm::Type::getInt64Ty((mod)->getContext())
#define KAPRINO_INT64_PTR_TY(mod) llvm::Type::getInt64PtrTy((mod)->getContext())
#define KAPRINO_DOUBLE_TY(mod) llvm::Type::getDoubleTy((mod)->getContext())
#define KAPRINO_DOUBLE_PTR_TY(mod) llvm::Type::getDoublePtrTy((mod)->getContext())

#define KAPRINO_CONFIRM_INT64(mod, x, y) ((x)->getType() == KAPRINO_INT64_TY(mod)) && ((y)->getType() == KAPRINO_INT64_TY(mod))
#define KAPRINO_CAST_SI_FP(bldr, mod, x) ((x)->getType() == KAPRINO_INT64_TY(mod)) ? bldr->CreateSIToFP(x, KAPRINO_DOUBLE_TY(mod)) : (x);

//
// ------ File system ------
//

#define KAPRINO_PATH_ENV "KAPRINOPKG"
#ifdef _WIN32
#   define KAPRINO_ENV_SPLIT_CHAR ";"
#else
#   define KAPRINO_ENV_SPLIT_CHAR ":"
#endif

//
// ------ Cast ------
//

#define KAPRINO_TO_STR(ptr) [](char* p) { std::string str(p); return str; }(ptr)

};  // namespace kaprino::kgen
