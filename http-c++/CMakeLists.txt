cmake_minimum_required(VERSION 3.15)
project(PrivacyHttpSdk VERSION 1.0.5)  # Use a valid version number (no hyphen)

# Define the full version string (including the 'beta' part)
set(PROJECT_VERSION "1.0.5-beta")

# Split the version into major, minor, and patch components
string(REGEX MATCH "^([0-9]+)" PROJECT_VERSION_MAJOR ${PROJECT_VERSION})
string(REGEX MATCH "\\.([0-9]+)" PROJECT_VERSION_MINOR ${PROJECT_VERSION})
string(REGEX MATCH "\\.([0-9]+)[^0-9]" PROJECT_VERSION_PATCH ${PROJECT_VERSION})

# description
set(PROJECT_DESCRIPTION "Privacy HTTP SDK for C++")

# Optionally, you can define a custom suffix for beta/alpha versions
set(PROJECT_VERSION_SUFFIX "beta")

# Add Rust shared library
set(RUST_LIB_PATH "../privacy_http_sdk/target/release")
set(RUST_LIB "${RUST_LIB_PATH}/libprivacy_http_sdk.so")  # Adjust extension based on OS

# Add include path for cxx auto-generated headers
include_directories("../privacy_http_sdk/src")

# Specify the C++ source file
add_executable(http_client main.cpp)

# Link the Rust library
target_link_libraries(http_client "${RUST_LIB}")

# Generate version.h from version.h.in
configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/version.h.in"
    "${CMAKE_CURRENT_BINARY_DIR}/version.h"
)

# Include the generated version file
include_directories("${CMAKE_CURRENT_BINARY_DIR}")
