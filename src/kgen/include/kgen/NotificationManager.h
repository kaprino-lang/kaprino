#include <string>
#include <vector>

namespace kaprino::kgen {

//
// A event handler to NotificationManager.
//
class NotificationHandler {
   public:
    void log(std::string message, std::string file, int line, int pos);
    void warn(std::string message, std::string file, int line, int pos);
    void error(std::string message, std::string file, int line, int pos);
};

//
// Track notifications and redirect them to handlers.
//
class NotificationManager {
   public:
    NotificationManager();
    void add_handler(NotificationHandler* handler);
    void remove_handler(NotificationHandler* handler);
    void log(std::string message, std::string file, int line, int pos);
    void warn(std::string message, std::string file, int line, int pos);
    void error(std::string message, std::string file, int line, int pos);
    void asrt(bool expr, std::string message, std::string file, int line, int pos);

   private:
    std::vector<NotificationHandler*>* handlers;
};

};  // namespace kaprino::kgen
