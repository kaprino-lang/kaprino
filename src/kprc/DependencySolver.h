#include "KaprinoAccelerator.h"
#include "visitor/statements/StatementObject.h"

class DependencySolver {
   public:
    static std::vector<StatementObject*>* importRequirePackage(std::string require_package);
    static std::string findPackageFile(std::string package_name);
    static std::vector<std::string> linkFiles;

   private:
    static std::vector<StatementObject*>* readFile(std::string path);
    static std::string checkdirs(std::string package_name, std::string env_name);
    static std::vector<std::string> getdirs(std::string env);
    static std::vector<std::string> alreadyImported;
};
