#include "kgen/NotificationManager.h"

namespace kaprino::kprc {

class KprcNotificationHandler : public kaprino::kgen::NotificationHandler {
   public:
    KprcNotificationHandler();
    void log(std::string message, std::string file, int line, int pos) override;
    void warn(std::string message, std::string file, int line, int pos) override;
    void error(std::string message, std::string file, int line, int pos) override;
    static void use();
};

}; // namespace kaprino::kprc
