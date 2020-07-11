#include "kgen/KgenAccelerator.h"

namespace kaprino::kprc {

class ArgsManager {
   public:
    static void setArgs(int argc, const char* argv[]);
    static std::string getFile();
    static bool getFlag(std::string name);

   private:
    static std::vector<std::string> args;
};

};  // namespace kaprino::kprc
