#include "KaprinoAccelerator.h"

class ArgsManager {
   public:
    static void setArgs(int argc, const char* argv[]);
    static std::string getFile();
    static bool getFlag(std::string name);
   private:
    static std::vector<std::string> args;
};
