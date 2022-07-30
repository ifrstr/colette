cmake_minimum_required(VERSION 3.0)

############################################################
# Project
############################################################

project(colette
    VERSION 0.1.0
    DESCRIPTION "Convert between truecolor, xterm(1) 256 color and 16 color."
    HOMEPAGE_URL "https://gopkg.ilharper.com/x/colette"
    LANGUAGES C CXX
)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 11)

set(CMAKE_MESSAGE_LOG_LEVEL VERBOSE)

############################################################
# Library
############################################################

target_include_directories(colette
    PUBLIC
        ${PROJECT_SOURCE_DIR}/include
)

############################################################
# Compile Flags
############################################################

target_compile_definitions(colette
    PUBLIC
        UNICODE
        _UNICODE
)

if(MSVC)
    target_compile_options(colette
        PUBLIC
            /utf-8
    )
else()
    target_compile_options(colette
        PUBLIC
            -fexec-charset=UTF-8
            -finput-charset=UTF-8
    )
endif()

############################################################
# Test
############################################################

add_subdirectory(3rd_party/googletest)

add_executable(colette_test
    test/colette_test.cc
)

target_link_libraries(colette_test
    colette
    gtest_main
)

add_test(NAME test_all COMMAND colette_test)

############################################################
# Install
############################################################

# Library
if(WIN32)
    install (TARGETS colette
        LIBRARY DESTINATION lib
        RUNTIME DESTINATION bin
        COMPONENT colette_component
        OPTIONAL)
else()
    install (TARGETS colette
        LIBRARY DESTINATION lib
        COMPONENT colette_component
        OPTIONAL)
endif()

# Header files
install(DIRECTORY ${PROJECT_SOURCE_DIR}/include/
    DESTINATION include)

############################################################
# Uninstall
############################################################

if(NOT TARGET uninstall)
    configure_file(
        "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
        IMMEDIATE @ONLY)

    add_custom_target(uninstall
        COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
endif()

############################################################
# Package
############################################################

set(CPACK_PACKAGE_VENDOR "Il Harper")
set(CPACK_PACKAGE_DESCRIPTION_FILE "${CMAKE_CURRENT_SOURCE_DIR}/README.md")
set(CPACK_PACKAGE_DESCRIPTION ${PROJECT_DESCRIPTION})
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY ${PROJECT_DESCRIPTION})
set(CPACK_PACKAGE_INSTALL_DIRECTORY ${PROJECT_NAME})
set(CPACK_RESOURCE_FILE_README "${CMAKE_CURRENT_SOURCE_DIR}/README.md")
set(CPACK_GENERATOR "DEB;RPM")

set(CPACK_ARCHIVE_COMPONENT_INSTALL ON)
set(CPACK_RPM_COMPONENT_INSTALL ON)
set(CPACK_DEB_COMPONENT_INSTALL ON)

set(CPACK_DEBIAN_PACKAGE_MAINTAINER "Il Harper")
set(CPACK_DEBIAN_PACKAGE_SECTION "Libs")

# Include CPack
include(CPack)

# Add Component
cpack_add_component_group(colette
    DISPLAY_NAME ${PROJECT_NAME}
    DESCRIPTION ${PROJECT_DESCRIPTION}
    BOLD_TITLE true
)
cpack_add_component(colette_component
    DISPLAY_NAME ${PROJECT_NAME}
    DESCRIPTION ${PROJECT_DESCRIPTION}
    GROUP colette
)
