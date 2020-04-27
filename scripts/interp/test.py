import pathlib
import subprocess as subproc
import scripts.parser.pushd as pushd

EXECUTABLE_FILE_PATH = "build/kaprino.exe"

def runTest(file_name):
    file_without_extension = pathlib.Path(file_name).stem
    file_path = pushd.getExamplesDir().joinpath(file_name)
    subproc.run([EXECUTABLE_FILE_PATH, str(file_path)])
    print("----------")
    file_ir_path = pushd.getExamplesDir().joinpath(file_without_extension + ".ll")
    subproc.run(["lli", str(file_ir_path)])

if __name__ == "__main__":
    file_name = input("Which one do you want to run ? > ")
    runTest(file_name)
