#pragma once

#include <iostream>
#include <random>
#include <string>

#include "antlr4-runtime.h"

#include "llvm/ADT/Optional.h"
#include "llvm/ADT/STLExtras.h"
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

//
// ------ Log system ------
//

#ifdef KAPRINO_MONOTONE_LOG

#define KAPRINO_LOG(msg) std::cout << "[LOG] " << msg << std::endl
#define KAPRINO_WARN(msg) std::cout << "[WARNING] " << msg << std::endl
#define KAPRINO_ERR(msg) std::cerr << "[ERROR] " << msg << std::endl

#else

#ifdef _WIN32

#include <windows.h>
#define KAPRINO_LOG_INIT() SetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), ENABLE_VIRTUAL_TERMINAL_PROCESSING)

#endif

#define ANSI_YELLOW_CODE "\u001b[33;1m"
#define ANSI_RED_CODE "\u001b[31;1m"
#define ANSI_CLEAN_CODE "\u001b[0m"

#define KAPRINO_LOG(msg) std::cout << "[LOG] " << msg << std::endl
#define KAPRINO_WARN(msg) std::cout << ANSI_YELLOW_CODE << "[WARNING] " << msg << ANSI_CLEAN_CODE << std::endl
#define KAPRINO_ERR(msg) std::cerr << ANSI_RED_CODE << "[ERROR] " << msg << ANSI_CLEAN_CODE << std::endl

#endif

#ifndef KAPRINO_LOG_INIT

#define KAPRINO_LOG_INIT()

#endif

//
// ------ LLVM type ------
//

#define KAPRINO_BOOL_TY(mod) llvm::Type::getInt1Ty((mod)->getContext())
#define KAPRINO_INT8_PTR_TY(mod) llvm::Type::getInt8PtrTy((mod)->getContext())
#define KAPRINO_INT32_TY(mod) llvm::Type::getInt32Ty((mod)->getContext())
#define KAPRINO_INT64_TY(mod) llvm::Type::getInt64Ty((mod)->getContext())
#define KAPRINO_INT64_PTR_TY(mod) llvm::Type::getInt64PtrTy((mod)->getContext())
#define KAPRINO_DOUBLE_TY(mod) llvm::Type::getDoubleTy((mod)->getContext())
#define KAPRINO_DOUBLE_PTR_TY(mod) llvm::Type::getDoublePtrTy((mod)->getContext())

#define KAPRINO_CONFIRM_INT64(mod, x, y) ((x)->getType() == KAPRINO_INT64_TY(mod)) && ((y)->getType() == KAPRINO_INT64_TY(mod))
#define KAPRINO_CAST_SI_FP(bldr, mod, x) ((x)->getType() == KAPRINO_INT64_TY(mod)) ? bldr->CreateSIToFP(x, KAPRINO_DOUBLE_TY(mod)) : (x);

//
// ------ File system ------
//

#define KAPRINO_RM_FILE_EXT(file_path) (file_path).substr(0, (file_path).find_last_of("."))