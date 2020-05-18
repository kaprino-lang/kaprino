#include "PackagePathManager.h"

PackagePathManager::PackagePathManager(std::string name) {
    package_name = name;
}

std::string PackagePathManager::find() {
    auto file = checkdirs(PackagePathManager::KAPRINO_PATH_ENV);
    if (file != "")
    {
        return file;
    }

    file = checkdirs("PATH");
    if (file != "")
    {
        return file;
    }

    KAPRINO_ERR("Not found the package: " << package_name);
    throw -1;
}

std::string PackagePathManager::checkdirs(std::string env_name) {
    auto envcstr = std::getenv(env_name.c_str());

    if (envcstr == nullptr) {
        KAPRINO_WARN("The environment variable, " << env_name << " is not setted");
        return "";
    }

    auto env = KAPRINO_TO_STR(envcstr);

    auto envdirs = getdirs(env);
    for (auto dir : envdirs) {
        auto file = dir + "/" + package_name + ".kpr";
        if (exists(file)) {
            KAPRINO_LOG("Package found: " << file);
            return file;
        }
    }

    return "";
}

std::vector<std::string> PackagePathManager::getdirs(std::string env) {
    size_t pos = 0;
    std::vector<std::string> pathes;
    while ((pos = env.find(SPLIT_CHAR)) != std::string::npos) {
        pathes.push_back(env.substr(0, pos));
        env.erase(0, pos + 1);
    }
    return pathes;
}

inline bool PackagePathManager::exists(std::string name) {
    std::ifstream f(name.c_str());
    return f.good();
}