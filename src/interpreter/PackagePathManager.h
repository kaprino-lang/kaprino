#include "KaprinoAccelerator.h"

class PackagePathManager {
   public:
    PackagePathManager(std::string package_name);
    std::string find();

   private:
    std::string package_name;
    const std::string KAPRINO_PATH_ENV = "KAPRINOPKG";

#if _WIN32

    const std::string SPLIT_CHAR = ";";

#else

    const std::string SPLIT_CHAR = ":";

#endif

    std::string checkdirs(std::string env_name);
    std::vector<std::string> getdirs(std::string env);
};