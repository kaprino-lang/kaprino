import sys
import subprocess as subproc
import pathlib
import shutil

import scripts.parser.pushd as pushd

def buildParser(lang):
    with pushd.chParserDir():
        ret = subproc.run(["antlr4", "*.g4", "-Dlanguage=" + lang], shell=True)
        if ret.returncode != 0:
            print("antlr4 returns code:" + ret.returncode)
            sys.exit()
        additionalBuild(lang)

def additionalBuild(lang):
    if lang == "Java":
        ret = subproc.run(["javac", "Kaprino*.java"], shell=True)
        if ret.returncode != 0:
            print("javac returns code:" + ret.returncode)
            sys.exit()

if __name__ == "__main__":
    lang = input("What you want to build for ? (Language name) > ")
    buildParser(lang)
