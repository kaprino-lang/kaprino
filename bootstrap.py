import os
import sys
import contextlib

#
# Pushd suggested on https://gist.github.com/howardhamilton/537e13179489d6896dd3
#
@contextlib.contextmanager
def pushd(new_dir):
	old_dir = os.getcwd()
	os.chdir(new_dir)
	try:
		yield
	finally:
		os.chdir(old_dir)

#
# Setup vcpkg and install dependencies
#
VCPKG_USE_HASH = "9f2bb7091458d393a424c0ab14cd39c49332446c"

os.system("git clone https://github.com/microsoft/vcpkg.git")

with pushd("vcpkg"):
    os.system(f"git checkout {VCPKG_USE_HASH}")
    if os.name == 'nt':
        os.system(".\\bootstrap-vcpkg.bat")
        os.system(".\\vcpkg install antlr4 llvm[target-all]")
    else:
        os.system("./bootstrap-vcpkg.sh")
        os.system("./vcpkg install antlr4 llvm[target-all]")

#
# Build kaprino
#
if len(sys.argv) == 1:
    CMAKE_BUILD_CONFIG = "Debug"
else:
    CMAKE_BUILD_CONFIG = sys.argv[1]

os.mkdir("build")

with pushd("build"):
    if os.name == 'nt':
        os.system(f"cmake .. -DCMAKE_BUILD_TYPE={CMAKE_BUILD_CONFIG} -DCMAKE_TOOLCHAIN_FILE=..\\vcpkg\\scripts\\buildsystems\\vcpkg.cmake")
        os.system("cmake --build .\\ --target install")
    else:
        os.system(f"cmake .. -DCMAKE_BUILD_TYPE={CMAKE_BUILD_CONFIG} -DCMAKE_TOOLCHAIN_FILE=../vcpkg/scripts/buildsystems/vcpkg.cmake")
        os.system("cmake --build ./ --target install")
