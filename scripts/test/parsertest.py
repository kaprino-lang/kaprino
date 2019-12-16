import sys
import os
import subprocess as subproc
import pathlib

import scripts.cleanparser as cleanparser

class ParserTester:
    def __init__(self, do_compile, file_name):
        self.file_dir = pathlib.Path(__file__).resolve().parent
        self.previous_current_dir = os.getcwd()
        self.working_dir = os.path.join(self.file_dir, "../../src/parser")
        self.do_compile = do_compile
        self.file_path = os.path.join(self.file_dir, "../../examples", file_name)

    def run(self):
        cleanparser.clean()

        os.chdir(self.working_dir)

        if self.do_compile:
            print("Task running : Build To Java")
            self.buildToJava()
            print("Task finished")

        subproc.run(["grun", "Kaprino", "program", self.file_path, "-tree", "-gui"], shell=True)

        os.chdir(self.previous_current_dir)

    def buildToJava(self):
        ret = subproc.run(["antlr4", "*.g4"], shell=True)
        if ret.returncode != 0:
            print("antlr4 returns code:" + ret.returncode)
            sys.exit()
        ret = subproc.run(["javac", "Kaprino*.java"], shell=True)
        if ret.returncode != 0:
            print("javac returns code:" + ret.returncode)
            sys.exit()

if __name__ == "__main__":
    do_compile = input("Do you want to compile it ? > ") == "y"
    file_name = input("Which one do you want to run ? > ")
    tester = ParserTester(do_compile=do_compile, file_name=file_name)
    tester.run()
