import os
import pathlib
import glob

def clean():
    file_path = pathlib.Path(__file__).resolve().parent
    parser_files = file_path.joinpath("../src/parser/*.*")
    for file in glob.glob(str(parser_files)):
        if pathlib.Path(file).suffix != ".g4":
            os.remove(file)

if __name__ == "__main__":
    ans = input("You really want to delete the files ? > ")
    if ans == "y":
        clean()
