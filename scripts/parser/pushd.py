"""
This script is based on https://gist.github.com/howardhamilton/537e13179489d6896dd3
"""

import pathlib
from contextlib import contextmanager

def getParserDir():
    file_path = pathlib.Path(__file__).resolve().parent
    working_dir = file_path.joinpath("../../src/parser")
    return working_dir

@contextmanager
def chParserDir():
    previous_dir = pathlib.Path.cwd()
    working_dir = getParserDir()
    pathlib.os.chdir(working_dir)
    try:
        yield
    finally:
        pathlib.os.chdir(previous_dir)

def getExamplesDir():
    file_path = pathlib.Path(__file__).resolve().parent
    working_dir = file_path.joinpath("../../examples")
    return working_dir

@contextmanager
def chExamplesDir():
    previous_dir = pathlib.Path.cwd()
    working_dir = getExamplesDir()
    pathlib.os.chdir(working_dir)
    try:
        yield
    finally:
        pathlib.os.chdir(previous_dir)
