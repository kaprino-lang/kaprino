#if defined(_WIN32) & !defined(KAPRINO_MONOTONE_LOG)
#   include <windows.h>
#endif

#include "kgen/KgenAccelerator.h"
#include "kprc/notifications/KprcNotificationHandler.h"

namespace kaprino::kprc {

#if _WIN32 & !defined(KAPRINO_MONOTONE_LOG)

KprcNotificationHandler::KprcNotificationHandler() {
    SetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), ENABLE_VIRTUAL_TERMINAL_PROCESSING);
}

#else

KprcNotificationHandler::KprcNotificationHandler() { }

#endif

void KprcNotificationHandler::log(std::string message, std::string file, int line, int pos) {
    std::cout << "[LOG] " << message << "(" << file << " line:" << line << " pos:" << pos << ")" << std::endl;
}

void KprcNotificationHandler::warn(std::string message, std::string file, int line, int pos) {
    std::cerr << ANSI_YELLOW_CODE << "[WARNING] " << message << "(" << file << " line:" << line << " pos:" << pos << ")" << ANSI_CLEAN_CODE << std::endl;
}

void KprcNotificationHandler::error(std::string message, std::string file, int line, int pos) {
    std::cerr << ANSI_RED_CODE << "[ERROR] " << message << "(" << file << " line:" << line << " pos:" << pos << ")" << ANSI_CLEAN_CODE << std::endl;
}

void KprcNotificationHandler::use() {
    auto handler = (kaprino::kgen::NotificationHandler*)new KprcNotificationHandler();

    kaprino::kgen::logger->add_handler(handler);
}

} // namespace kaprino::kprc
