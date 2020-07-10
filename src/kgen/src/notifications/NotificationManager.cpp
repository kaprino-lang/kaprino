#if defined(_WIN32) & !defined(KAPRINO_MONOTONE_LOG)
#   include <windows.h>
#endif

#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

NotificationManager::NotificationManager() {
    handlers = new std::vector<NotificationHandler*>();

#if _WIN32 & !defined(KAPRINO_MONOTONE_LOG)

    SetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), ENABLE_VIRTUAL_TERMINAL_PROCESSING);

#endif
}

void NotificationManager::add_handler(NotificationHandler* handler) {
    handlers->push_back(handler);
}

void NotificationManager::remove_handler(NotificationHandler* handler) {
    auto item = std::find(handlers->begin(), handlers->end(), handler);

    logger->asrt(
        item == handlers->end(),
        "Failed to remove NotificationHandler", "internal", 0, 0
    );

    handlers->erase(item);
}

void NotificationManager::log(std::string message, std::string file, int line, int pos) {
    for(auto handle : *handlers) {
        handle->log(message, file, line, pos);
    }
}

void NotificationManager::warn(std::string message, std::string file, int line, int pos) {
    for(auto handle : *handlers) {
        handle->warn(message, file, line, pos);
    }
}

void NotificationManager::error(std::string message, std::string file, int line, int pos) {
    for(auto handle : *handlers) {
        handle->error(message, file, line, pos);
    }
}

void NotificationManager::asrt(bool expr, std::string message, std::string file, int line, int pos) {
    if (expr) {
        error(message, file, line, pos);
        throw -1;
    }
}

};  // namespace kaprino::codegen
