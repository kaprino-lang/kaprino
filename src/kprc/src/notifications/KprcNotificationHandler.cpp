#include "kgen/KgenAccelerator.h"
#include "kprc/notifications/KprcNotificationHandler.h"

#ifdef KAPRINO_MONOTONE_LOG
#   undef ANSI_YELLOW_CODE
#   undef ANSI_RED_CODE
#   undef ANSI_CLEAN_CODE
#   define ANSI_YELLOW_CODE ""
#   define ANSI_RED_CODE ""
#   define ANSI_CLEAN_CODE ""
#else
#   ifdef _WIN32
#       define NOMINMAX
#       include <windows.h>
#       undef NOMINMAX
#       undef ERROR
#   endif
#endif

namespace kaprino::kprc {

#if _WIN32 & !defined(KAPRINO_MONOTONE_LOG)

KprcNotificationHandler::KprcNotificationHandler() {
    DWORD lpMode = 0;
    GetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), &lpMode);
    SetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), lpMode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
}

#else

KprcNotificationHandler::KprcNotificationHandler() { }

#endif

void KprcNotificationHandler::log(std::string message, std::string file, int line, int pos) {
    if (file == "internal") {
        std::cout << "[LOG] " << message << std::endl;
    }
    else {
        std::cout << "[LOG] " << message << " (" << file << " line:" << line << " pos:" << pos << ")" << std::endl;
    }
}

void KprcNotificationHandler::warn(std::string message, std::string file, int line, int pos) {
    if (file == "internal") {
        std::cerr << ANSI_YELLOW_CODE << "[WARNING] " << message << ANSI_CLEAN_CODE << std::endl;
    }
    else {
        std::cerr << ANSI_YELLOW_CODE << "[WARNING] " << message << " (" << file << " line:" << line << " pos:" << pos << ")" << ANSI_CLEAN_CODE << std::endl;
    }
}

void KprcNotificationHandler::error(std::string message, std::string file, int line, int pos) {
    if (file == "internal") {
        std::cerr << ANSI_RED_CODE << "[ERROR] " << message << ANSI_CLEAN_CODE << std::endl;
    }
    else {
        std::cerr << ANSI_RED_CODE << "[ERROR] " << message << " (" << file << " line:" << line << " pos:" << pos << ")" << ANSI_CLEAN_CODE << std::endl;
    }
}

void KprcNotificationHandler::use() {
    auto kprc_handler = new KprcNotificationHandler();
    auto handler = dynamic_cast<kaprino::kgen::NotificationHandler*>(kprc_handler);

    kaprino::kgen::logger->add_handler(handler);
}

} // namespace kaprino::kprc
