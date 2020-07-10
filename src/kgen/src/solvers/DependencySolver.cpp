#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/DependencySolver.h"

namespace kaprino::kgen {

std::vector<StatementObject*>* ParseFile(std::string text);

std::vector<StatementObject*>* DependencySolver::importRequirePackage(std::string require_package) {
    auto statements = new std::vector<StatementObject*>();

    for (auto imported : alreadyImported) {
        if (require_package == imported) {
            return statements;
        }
    }

    auto file_name = findPackageFile(require_package);
    statements = readFile(file_name);

    auto llvmir_name = std::filesystem::path(file_name)
        .replace_extension(".impl.ll")
        .string();
    if (std::filesystem::exists(llvmir_name)) {
        linkFiles.push_back(llvmir_name);
    }
    return statements;
}

std::string DependencySolver::findPackageFile(std::string package_name) {
    auto executable_file = current_file;

    auto file = std::filesystem::path(executable_file)
        .parent_path()
        .append(package_name + ".kpr")
        .string();
    if (std::filesystem::exists(file)) {
        logger->log(
            "Package found: " + file,
            "internal",
            0,
            0
        );
        return file;
    }

    file = checkdirs(package_name, KAPRINO_PATH_ENV);
    if (file != "") {
        return file;
    }

    file = checkdirs(package_name, "PATH");
    if (file != "") {
        return file;
    }

    logger->error(
        "Not found the package: " + package_name,
        "internal",
        0,
        0
    );
    throw -1;
}

std::vector<std::string> DependencySolver::linkFiles;
//    *(new std::vector<std::string>());

std::vector<StatementObject*>* DependencySolver::readFile(std::string path) {
    std::ifstream input_file(path);

    std::ostringstream ss;
    ss << input_file.rdbuf();
    std::string input_text;
    input_text = ss.str();

    return ParseFile(input_text);
}

std::string DependencySolver::checkdirs(std::string package_name, std::string env_name) {
    auto envcstr = std::getenv(env_name.c_str());

    if (envcstr == nullptr) {
        logger->warn(
            "The environment variable, " + env_name + " is not setted",
            "internal",
            0,
            0
        );
        return "";
    }

    auto env = KAPRINO_TO_STR(envcstr);

    auto envdirs = getdirs(env);
    for (auto dir : envdirs) {
        auto file = dir + "/" + package_name + ".kpr";
        if (std::filesystem::exists(file)) {
            logger->log(
                "Package found: " + file,
                "internal",
                0,
                0
            );
            return file;
        }
    }

    return "";
}

std::vector<std::string> DependencySolver::getdirs(std::string env) {
    std::vector<std::string> pathes;
    if (env.find(KAPRINO_ENV_SPLIT_CHAR) == std::string::npos) {
        pathes.push_back(env);
        return pathes;
    }

    size_t pos = 0;
    while ((pos = env.find(KAPRINO_ENV_SPLIT_CHAR)) != std::string::npos) {
        pathes.push_back(env.substr(0, pos));
        env.erase(0, pos + 1);
    }
    return pathes;
}

std::vector<std::string> DependencySolver::alreadyImported;
//    *(new std::vector<std::string>());;

};  // namespace kaprino::kgen
