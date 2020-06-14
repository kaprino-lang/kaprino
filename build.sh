#!/bin/bash

#
# Build LLVM
#

wget https://github.com/llvm/llvm-project/releases/download/llvmorg-9.0.1/llvm-9.0.1.src.tar.xz
tar xJf llvm-9.0.1.src.tar.xz
mv ./llvm-llvm-9.0.1.src ./llvm

pushd llvm > /dev/null
    mkdir build
    pushd build > /dev/null
        cmake .. -DCMAKE_BUILD_TYPE=Release -DLLVM_BUILD_TOOLS=OFF
        make
        make install
    popd > /dev/null
popd > /dev/null

#
# Build kaprino
#

mkdir build
pushd build > /dev/null
    cmake .. -D-DCMAKE_BUILD_TYPE=Release
    make
popd > /dev/null
