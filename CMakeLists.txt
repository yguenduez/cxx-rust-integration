cmake_minimum_required(VERSION 3.20)

project(RustToCppWithCxx)

string(TOLOWER ${CMAKE_BUILD_TYPE} BUILD_TYPE)

# CXX Rust Library with CXX and cargo
add_library(rust_lib ${CMAKE_CURRENT_SOURCE_DIR}/target/cxxbridge/cxx_api/src/lib.rs.cc)
target_link_libraries(rust_lib PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/target/${BUILD_TYPE}/libcxx_api.a)
target_include_directories(rust_lib PUBLIC target/cxxbridge/cxx_api/src)
target_compile_features(rust_lib PRIVATE cxx_std_17)

# Library Dependants
add_executable(main main.cpp)
target_link_libraries(main PRIVATE rust_lib)
target_compile_features(main PRIVATE cxx_std_17)
