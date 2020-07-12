#include "kprc/ArgsManager.h"

namespace kaprino::kprc {

void ArgsManager::setArgs(int argc, const char* argv[]) {
    for (int counter = 0; counter < argc; counter++) {
        args.push_back(argv[counter]);
    }

    if (argc <= 1) {
        kaprino::kgen::logger->error(
            "No input",
            "internal", 0, 0
        );
        throw -1;
    }
}

std::string ArgsManager::getFile() {
    return args[1];
}

bool ArgsManager::getFlag(std::string name) {
    for (auto arg : args) {
        if (arg == name) return true;
    }
    return false;
}

std::vector<std::string> ArgsManager::args;

};  // namespace kaprino::kprc
