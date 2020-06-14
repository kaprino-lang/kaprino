#
# Build LLVM
#

$LLVM_ARCHIVE  = "./llvm-9.0.1.src.tar.xz"
$DOWNLOAD_LLVM_URL = "https://github.com/llvm/llvm-project/releases/download/llvmorg-9.0.1/llvm-9.0.1.src.tar.xz"

Invoke-WebRequest $DOWNLOAD_LLVM_URL -OutFile $LLVM_ARCHIVE

7z x $LLVM_ARCHIVE

cd "llvm-9.0.1.src"
    mkdir build
    cd build
        cmake .. -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release -DLLVM_BUILD_TOOLS=OFF
        mingw32-make
        mingw32-make install
    cd ..
cd ..

#
# Build kaprino
#

mkdir build
cd build
    cmake .. -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release -DLLVM_BUILD_TOOLS=OFF
    mingw32-make
    mingw32-make install
cd ..
