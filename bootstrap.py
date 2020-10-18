import os
import pathlib
import contextlib
import argparse

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
# Check arguments
#

parser = argparse.ArgumentParser(description="To build kaprino, please run this script.")

parser.add_argument("--config", help="Build kaprino with configuration. \"Debug\" will be used by default.", default="Debug")
parser.add_argument("--vcpkg", help="Path to vcpkg")

args = parser.parse_args()

#
# Bootstrap args
#
CMAKE_BUILD_CONFIG = args.config
VCPKG_PATH = args.vcpkg
VCPKG_USE_HASH = "9f2bb7091458d393a424c0ab14cd39c49332446c"

#
# Setup vcpkg and install dependencies
#
if VCPKG_PATH == None:
    os.system("git clone https://github.com/microsoft/vcpkg.git")

    with pushd("vcpkg"):
        os.system(f"git checkout {VCPKG_USE_HASH}")
        if os.name == 'nt':
            os.system(".\\bootstrap-vcpkg.bat")
            os.system(".\\vcpkg install antlr4 llvm[target-all]")
        else:
            os.system("./bootstrap-vcpkg.sh")
            os.system("./vcpkg install antlr4 llvm[target-all]")

    VCPKG_PATH = pathlib.Path(os.getcwd()).joinpath("vcpkg").as_uri()

#
# Build kaprino
#
os.mkdir("build")

with pushd("build"):
    if os.name == 'nt':
        os.system(f"cmake .. -DCMAKE_BUILD_TYPE={CMAKE_BUILD_CONFIG} -DCMAKE_TOOLCHAIN_FILE={VCPKG_PATH}\\scripts\\buildsystems\\vcpkg.cmake")
        os.system("cmake --build .\\ --target install")
    else:
        os.system(f"cmake .. -DCMAKE_BUILD_TYPE={CMAKE_BUILD_CONFIG} -DCMAKE_TOOLCHAIN_FILE={VCPKG_PATH}/vcpkg/scripts/buildsystems/vcpkg.cmake")
        os.system("cmake --build ./ --target install")
