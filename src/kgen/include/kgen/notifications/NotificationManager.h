#pragma once

#include <stack>
#include <string>
#include <vector>

namespace kaprino::kgen {

//
// A event handler to NotificationManager.
//
class NotificationHandler {
   public:
    virtual void log(std::string message, std::string file, int line, int pos);
    virtual void warn(std::string message, std::string file, int line, int pos);
    virtual void error(std::string message, std::string file, int line, int pos);
};

//
// Track notifications and redirect them to handlers.
//
class NotificationManager {
   public:
    NotificationManager();

    int line;
    int pos;

    void add_handler(NotificationHandler* handler);
    void remove_handler(NotificationHandler* handler);
    void move_pos(int line, int pos);
    void log(std::string message);
    void warn(std::string message);
    void error(std::string message);
    void asrt(bool expr, std::string message);
    void log(std::string message, std::string file, int line, int pos);
    void warn(std::string message, std::string file, int line, int pos);
    void error(std::string message, std::string file, int line, int pos);
    void asrt(bool expr, std::string message, std::string file, int line, int pos);

   private:
    std::vector<NotificationHandler*>* handlers;
};

};  // namespace kaprino::kgen
