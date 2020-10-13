########################################################
#
# Source files
#
########################################################

set(KPRC_INCLUDE_DIR "${CMAKE_SOURCE_DIR}/src/kprc/include")
set(KPRC_SOURCE_DIR "${CMAKE_SOURCE_DIR}/src/kprc/src")

set(KPRC_SOURCE
    "${KPRC_SOURCE_DIR}/ArgsManager.cpp"
    "${KPRC_SOURCE_DIR}/ExecutableGenerator.cpp"
    "${KPRC_SOURCE_DIR}/KprcProgram.cpp"
    "${KPRC_SOURCE_DIR}/notifications/KprcNotificationHandler.cpp"
)

########################################################
#
# Define kaprino, the application which we want to get.
#
########################################################

add_executable(kprc ${KPRC_SOURCE})

target_link_libraries(kprc kgen)

target_include_directories(kprc PRIVATE ${KPRC_INCLUDE_DIR})
target_include_directories(kprc PRIVATE ${KGEN_INCLUDE_DIR})

target_compile_definitions(kprc
    PRIVATE KAPRINO_VERSION=\"${kaprino_VERSION}\"
)

########################################################
#
# Link ANTLR4 libraries.
#
########################################################

target_include_directories(kprc PRIVATE ${ANTLR4_INCLUDE_DIRS})
target_include_directories(kprc PRIVATE ${KAST_INCLUDE_DIR})

########################################################
#
# Link LLVM libraries.
#
########################################################

target_include_directories(kprc PRIVATE ${LLVM_INCLUDE_DIRS})

########################################################
#
# Make output monotone.
#
########################################################

if(${KAPRINO_MONOTONE_LOG})
    message(WARNING "[WARN] Make stdout monotone")
    target_compile_definitions(kprc
        PRIVATE KAPRINO_MONOTONE_LOG
    )
endif()

########################################################
#
# Define kaprino, the application which we want to get.
#
########################################################

if(${KAPRINO_OPTIMIZER_ON})
    message(WARNING "[EXPERIMENTAL] Optimizer on")
    target_compile_definitions(kprc
        PRIVATE KAPRINO_OPTIMIZER_ON
    )
endif()

########################################################
#
# Create install rule
#
########################################################

install(TARGETS kprc)
