cmake_minimum_required(VERSION 3.15)

project(kaprino VERSION 1.3.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

########################################################
#
# Variables to build
#
########################################################

set(KAPRINO_BUILD_STDLIB ON CACHE BOOL "Build stdlib")
set(KAPRINO_MONOTONE_LOG OFF CACHE BOOL "Make stdout monotone")
set(KAPRINO_OPTIMIZER_ON OFF CACHE BOOL "[Experimental] Optimize output")

########################################################
#
# Dependencies LLVM
#
########################################################

find_package(LLVM CONFIG REQUIRED)

########################################################
#
# Dependencies: Antlr4
#
########################################################

find_path(ANTLR4_INCLUDE_DIRS antlr4-runtime.h)
find_library(ANTLR4_LIBS antlr4-runtime)

########################################################
#
# Build parser
#
########################################################

include(${CMAKE_SOURCE_DIR}/src/kast/CMakeLists.txt)

########################################################
#
# Build kgen
#
########################################################

include(${CMAKE_SOURCE_DIR}/src/kgen/CMakeLists.txt)

########################################################
#
# Build compiler
#
########################################################

include(${CMAKE_SOURCE_DIR}/src/kprc/CMakeLists.txt)

########################################################
#
# Build standard libraries
#
########################################################

if(KAPRINO_BUILD_STDLIB)
    include(${CMAKE_SOURCE_DIR}/src/stdlib/CMakeLists.txt)
else()
    message(WARNING "[WARN] Disable building stdlib")
endif()
