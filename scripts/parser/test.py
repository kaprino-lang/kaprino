import sys
import os
import subprocess as subproc
import pathlib

import scripts.parser.clean as cleanparser
import scripts.parser.build as buildparser
import scripts.parser.pushd as pushd

class ParserTester:
    def __init__(self, do_compile, file_name):
        self.do_compile = do_compile
        self.file_path = pushd.getExamplesDir().joinpath(file_name)

    def run(self):
        if self.do_compile:
            cleanparser.clean()
            buildparser.buildParser("Java")

        with pushd.chParserDir():
            subproc.run(["grun", "Kaprino", "program", str(self.file_path), "-tree", "-gui"], shell=True)

if __name__ == "__main__":
    do_compile = input("Do you want to compile it ? > ") == "y"
    file_name = input("Which one do you want to run ? > ")
    tester = ParserTester(do_compile=do_compile, file_name=file_name)
    tester.run()
