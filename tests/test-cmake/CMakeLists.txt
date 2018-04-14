cmake_minimum_required(VERSION 3.10.2)
enable_testing()

add_library(hello_world hello_world.cpp bye_world.cpp)
target_include_directories(hello_world INTERFACE .)
set_target_properties(hello_world
  PROPERTIES
    COMPILE_PDB_NAME hello_world_cl
)

add_executable(app_hello_world app_hello_world.cpp)
add_test(app_hello_world app_hello_world)
target_link_libraries(app_hello_world PRIVATE hello_world)
set_target_properties(app_hello_world
  PROPERTIES
    COMPILE_PDB_NAME app_hello_world_cl
)

add_executable(app_exe_replacer app_exe_replacer.cpp)
set_target_properties(app_exe_replacer
  PROPERTIES
    COMPILE_PDB_NAME app_exe_replacer_cl
)

add_library(hello_world_shared SHARED hello_world.cpp bye_world.cpp)
target_include_directories(hello_world_shared INTERFACE .)
set_target_properties(hello_world_shared
  PROPERTIES
    COMPILE_PDB_NAME hello_world_shared_cl
)