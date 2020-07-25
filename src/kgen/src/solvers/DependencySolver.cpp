#include "kgen/KgenAccelerator.h"
#include "kgen/solvers/DependencySolver.h"
#include "kgen/visitors/KgenErrorListener.h"
#include "kgen/visitors/StatementObject.h"
#include "kgen/visitors/StatementVisitor.h"

using namespace antlr4;

namespace kaprino::kgen {

DependencySolver depsolver;

DependencySolver::DependencySolver() {
    auto env_dirs = processEnv(KAPRINO_PATH_ENV);
    searchDirectories.insert(
        searchDirectories.end(),
        env_dirs->begin(),
        env_dirs->end()
    );

    env_dirs = processEnv("PATH");
    searchDirectories.insert(
        searchDirectories.end(),
        env_dirs->begin(),
        env_dirs->end()
    );
}

std::vector<StatementObject*>* DependencySolver::parseFile(std::string file_path) {
    logger->asrt(
        std::filesystem::exists(file_path),
        "Not found an input file: \"" + file_path + "\"",
        "internal", 0, 0
    );

    ANTLRFileStream input(file_path);
    KaprinoLexer lexer(&input);
    CommonTokenStream tokens(&lexer);
    KaprinoParser parser(&tokens);
    KgenErrorListener errorListener;

    lexer.removeErrorListeners();
    lexer.addErrorListener(&errorListener);

    parser.removeErrorListeners();
    parser.addErrorListener(&errorListener);

    auto program = parser.program();

    StatementVisitor visitor;
    auto programObject = visitor.visitProgram(program).as<std::vector<StatementObject*>*>();

    return programObject;
}

std::vector<StatementObject*>* DependencySolver::importRequirePackage(std::string require_package) {
    auto statements = new std::vector<StatementObject*>();

    for (auto imported : alreadyImported) {
        if (require_package == imported) {
            return statements;
        }
    }

    auto file_name = findPackageFile(require_package);
    statements = parseFile(file_name);

    auto llvmir_name = std::filesystem::path(file_name)
        .replace_extension(".impl.ll")
        .string();

    if (std::filesystem::exists(llvmir_name)) {
        linkFiles.push_back(llvmir_name);
    }
    return statements;
}

std::string DependencySolver::findPackageFile(std::string package_name) {
    for (auto importDirectory : searchDirectories) {
        auto package = findPackageFile(importDirectory, package_name);
        if (package != "") {
            return package;
        }
    }

    logger->error(
        "Not found the package: " + package_name,
        "internal", 0, 0
    );
    throw -1;
}

std::string DependencySolver::findPackageFile(std::string directory_name, std::string package_name) {
    auto file = directory_name + "/" + package_name + ".kpr";

    if (std::filesystem::exists(file)) {
        logger->log(
            "Package found: " + file
        );
        return file;
    }
    else {
        return "";
    }
}

std::vector<std::string>* DependencySolver::processEnv(std::string env_name) {
    auto envcstr = std::getenv(env_name.c_str());

    if (envcstr == nullptr) {
        logger->warn(
            "The environment variable, " + env_name + " is not setted",
            "internal", 0, 0
        );
        return nullptr;
    }

    auto pathes = new std::vector<std::string>();
    std::string env(envcstr);

    if (env.find(KAPRINO_ENV_SPLIT_CHAR) == std::string::npos) {
        pathes->push_back(env);
        return pathes;
    }

    size_t pos = 0;
    while ((pos = env.find(KAPRINO_ENV_SPLIT_CHAR)) != std::string::npos) {
        pathes->push_back(env.substr(0, pos));
        env.erase(0, pos + 1);
    }
    return pathes;
}

};  // namespace kaprino::kgen
