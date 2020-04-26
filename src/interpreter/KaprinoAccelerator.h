#pragma once

#include <iostream>
#include <string>

#if !defined(KAPRINO_MONOTONE_LOG)

#if defined(KAPRINO_WIN32)

#include <windows.h>
#define KAPRINO_LOG_INIT() SetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), ENABLE_VIRTUAL_TERMINAL_PROCESSING)

#endif

#define ANSI_YELLOW_CODE "\u001b[33;1m"
#define ANSI_RED_CODE "\u001b[31;1m"
#define ANSI_CLEAN_CODE "\u001b[0m"

#define KAPRINO_LOG(msg) std::cout << "[LOG] " << msg << std::endl
#define KAPRINO_WARN(msg) std::cout << ANSI_YELLOW_CODE << "[WARNING] " << msg << ANSI_CLEAN_CODE << std::endl
#define KAPRINO_ERR(msg) std::cerr << ANSI_RED_CODE << "[ERROR] " << msg << ANSI_CLEAN_CODE << std::endl

#else

#define KAPRINO_LOG(msg) std::cout << "[LOG] " << msg << std::endl
#define KAPRINO_WARN(msg) std::cout << "[WARNING] " << msg << std::endl
#define KAPRINO_ERR(msg) std::cerr << "[ERROR] " << msg << std::endl

#endif

#ifndef KAPRINO_LOG_INIT

#define KAPRINO_LOG_INIT()

#endif