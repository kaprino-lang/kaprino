@ECHO OFF

git clone https://github.com/microsoft/vcpkg.git
git checkout 9f2bb7091458d393a424c0ab14cd39c49332446c
.\vcpkg\bootstrap-vcpkg.bat
.\vcpkg\vcpkg install antlr4 llvm[target-all]
mkdir build
cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=./vcpkg/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build ./ --target install
