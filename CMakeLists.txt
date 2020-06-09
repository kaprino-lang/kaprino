cmake_minimum_required(VERSION 3.15)

project(kaprino VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)

########################################################
#
# Dependencies pathes
#
########################################################

find_package(LLVM 9.0.0 REQUIRED)

set(KAPRINO_MONOTONE_LOG OFF CACHE BOOL "Make stdout monotone")
set(KAPRINO_OPTIMIZER_ON OFF CACHE BOOL "Optimize output")
set(KAPRINO_EMIT_LLVM_IR_ONLY OFF CACHE BOOL "Generate LLVM IR and finish program")
set(ANTLR4_IncludePath "" CACHE STRING "ANTLR4 include files")
set(ANTLR4_LibPath "" CACHE STRING "ANTLR4 library files")

########################################################
#
# Build parser
#
########################################################

include(${CMAKE_SOURCE_DIR}/src/parser/CMakeLists.txt)

########################################################
#
# Build interpreter
#
########################################################

include(${CMAKE_SOURCE_DIR}/src/interpreter/CMakeLists.txt)
