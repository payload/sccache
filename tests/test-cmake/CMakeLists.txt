cmake_minimum_required(VERSION 3.11.1)
enable_testing()

project(test_cmake)

add_library(lib_two_cpp_files src/hello_world.cpp src/bye_world.cpp)
target_include_directories(lib_two_cpp_files PUBLIC .)

#add_library(lib_two_cpp_files_shared SHARED src/hello_world.cpp src/bye_world.cpp)
#target_include_directories(lib_two_cpp_files PUBLIC .)

add_executable(app_one_cpp_file_static_lib src/app_hello_world.cpp)
target_link_libraries(app_one_cpp_file_static_lib lib_two_cpp_files)

#add_executable(app_one_cpp_file_shared_lib src/app_hello_world.cpp)
#target_link_libraries(app_one_cpp_file_shared_lib lib_two_cpp_files_shared)
