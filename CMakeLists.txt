cmake_minimum_required(VERSION 3.20)
project(order_matching VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

find_package(Boost 1.83 REQUIRED COMPONENTS system json)
find_package(OpenSSL 3.0 REQUIRED)
find_package(Threads REQUIRED)

include_directories(${CMAKE_SOURCE_DIR}/inc)
include_directories(${Boost_INCLUDE_DIRS})

set(SOURCES
    src/main.cpp
)

add_executable(order_matching ${SOURCES})

# link libraries
target_link_libraries(order_matching
    ${Boost_LIBRARIES}
    OpenSSL::SSL
    OpenSSL::Crypto
    Threads::Threads
    pthread
)

# flags
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_compile_options(order_matching PRIVATE -g -O0 -Wall -Wextra)
else()
    target_compile_options(order_matching PRIVATE -O3 -march=native -flto)
endif()

# build info
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Boost version: ${Boost_VERSION}")
message(STATUS "OpenSSL version: ${OPENSSL_VERSION}")