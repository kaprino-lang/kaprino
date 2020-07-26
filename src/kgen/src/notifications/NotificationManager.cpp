#include "kgen/KgenAccelerator.h"

namespace kaprino::kgen {

NotificationManager* logger;

void NotificationHandler::log(std::string message, std::string file, int line, int pos) { }
void NotificationHandler::warn(std::string message, std::string file, int line, int pos) { }
void NotificationHandler::error(std::string message, std::string file, int line, int pos) { }

NotificationManager::NotificationManager() {
    handlers = new std::vector<NotificationHandler*>();
}

void NotificationManager::add_handler(NotificationHandler* handler) {
    handlers->push_back(handler);
}

void NotificationManager::remove_handler(NotificationHandler* handler) {
    auto item = std::find(handlers->begin(), handlers->end(), handler);

    logger->asrt(
        item != handlers->end(),
        "Failed to remove NotificationHandler", "internal", 0, 0
    );

    handlers->erase(item);
}

void NotificationManager::move_pos(int line, int pos) {
    this->line = line;
    this->pos = pos;
}

void NotificationManager::log(std::string message) {
    log(message, depsolver.files.top(), line, pos);
}

void NotificationManager::warn(std::string message) {
    warn(message, depsolver.files.top(), line, pos);
}

void NotificationManager::error(std::string message) {
    error(message, depsolver.files.top(), line, pos);
}

void NotificationManager::asrt(bool expr, std::string message) {
    asrt(expr, message, depsolver.files.top(), line, pos);
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
    if (!expr) {
        error(message, file, line, pos);
        throw -1;
    }
}

};  // namespace kaprino::codegen
