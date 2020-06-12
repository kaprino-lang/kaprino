########################################################
#
# Source files
#
########################################################

set(STDLIB_SOURCES
    "${CMAKE_SOURCE_DIR}/src/stdlib/terminal.impl.c"
)

########################################################
#
# Build with target
#
########################################################

add_custom_target(stdlib ALL)

foreach(LIB ${STDLIB_SOURCES})
    get_filename_component(LIB_DIR "${LIB}" DIRECTORY)
    add_custom_command(TARGET stdlib
        POST_BUILD
        COMMAND clang -emit-llvm -c -S "${LIB}"
        WORKING_DIRECTORY ${LIB_DIR}
    )
endforeach()

########################################################
#
# Install target
#
########################################################

install(DIRECTORY "${CMAKE_SOURCE_DIR}/src/stdlib"
    DESTINATION "include"
)
