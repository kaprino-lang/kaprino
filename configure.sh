#!/bin/bash

git clone --depth 1 --branch 2020.07 https://github.com/microsoft/vcpkg.git
./vcpkg/bootstrap-vcpkg.bat
./vcpkg/vcpkg install antlr4 llvm[target-all]
mkdir build
cmake -B build -DCMAKE_TOOLCHAIN_FILE=./vcpkg/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build build
