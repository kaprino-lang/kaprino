#include "kgen/KgenAccelerator.h"
#include "kgen/visitors/StatementObject.h"

namespace kaprino::kgen {

class DependencySolver {
   public:
    static std::vector<StatementObject*>* parseFile(std::string text);
    static std::vector<StatementObject*>* importRequirePackage(std::string require_package);
    static std::string findPackageFile(std::string package_name);
    static std::vector<std::string> linkFiles;

   private:
    static std::vector<StatementObject*>* readFile(std::string path);
    static std::string checkdirs(std::string package_name, std::string env_name);
    static std::vector<std::string> getdirs(std::string env);
    static std::vector<std::string> alreadyImported;
};

};  // namespace kaprino::kgen
