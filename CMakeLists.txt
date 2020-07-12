cmake_minimum_required(VERSION 3.15)

project(kaprino VERSION 1.2.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

########################################################
#
# Dependencies LLVM
#
########################################################

find_package(LLVM 9.0.0 REQUIRED)

########################################################
#
# Dependencies: Antlr4
#
########################################################

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake)

add_definitions(-DANTLR4CPP_STATIC)

set(ANTLR4_WITH_STATIC_CRT OFF)
set(ANTLR_EXECUTABLE "${CMAKE_SOURCE_DIR}/antlr-4.8-complete.jar" CACHE FILEPATH "A path to antlr4 executable")

include(ExternalAntlr4Cpp)

find_package(ANTLR REQUIRED)

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
