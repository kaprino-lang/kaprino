import glob

if __name__ == "__main__":
    print("Parser files:")
    parser_files = glob.glob("src/parser/**/*.cpp", recursive = True)
    for parser_file in parser_files:
        print("\"${CMAKE_SOURCE_DIR}/" + parser_file.replace("\\", "/") + "\"")

    print("-----------------------")

    print("Interpreter files:")
    interpreter_files = glob.glob("src/interpreter/**/*.cpp", recursive = True)
    for interpreter_file in interpreter_files:
        print("\"${CMAKE_SOURCE_DIR}/" + interpreter_file.replace("\\", "/") + "\"")