import os
import pathlib
import glob

import scripts.parser.pushd as pushd

def clean():
    with pushd.chParserDir():
        for file in glob.glob("*.*"):
            if pathlib.Path(file).suffix != ".g4":
                os.remove(file)

if __name__ == "__main__":
    ans = input("You really want to delete the files ? > ")
    if ans == "y":
        clean()
