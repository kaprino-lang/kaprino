#!/bin/bash

#
# Build LLVM
#

LLVM_ARCHIVE  = "llvm-9.0.1.src.tar.xz"
DOWNLOAD_LLVM_URL = "https://github.com/llvm/llvm-project/releases/download/llvmorg-9.0.1/llvm-9.0.1.src.tar.xz"

wget ${DOWNLOAD_LLVM_URL}
tar xJf ${LLVM_ARCHIVE}
mv ./llvm-${LLVM_FILE_VERSION}.src ./llvm

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
    make install
popd > /dev/null
