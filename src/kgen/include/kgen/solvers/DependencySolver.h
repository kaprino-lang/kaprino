#pragma once

#include <stack>
#include <string>
#include <vector>

#include "kgen/visitors/StatementObject.h"

namespace kaprino::kgen {

class DependencySolver {
   public:
    DependencySolver();
    std::vector<StatementObject*>* parseFile(std::string file_path);
    std::vector<StatementObject*>* importRequirePackage(std::string require_package);
    std::string findPackageFile(std::string package_name);
    std::string findPackageFile(std::string directory_name, std::string package_name);
    std::stack<std::string> files;
    std::vector<std::string> searchDirectories;
    std::vector<std::string> linkFiles;

   private:
    std::vector<std::string>* processEnv(std::string env);
    std::vector<std::string> alreadyImported;
};

extern DependencySolver depsolver;

};  // namespace kaprino::kgen
