cmake_minimum_required(VERSION 3.15)
project(PuRe
    VERSION 0.1
    DESCRIPTION "PuRe"
    LANGUAGES CXX
)

# Forbid in-source builds 
if(PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
  message(
    FATAL_ERROR
      "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there."
  )
endif()

# Find the required dependencies
find_package(Ommatidia REQUIRED)
find_package(OpenCV REQUIRED COMPONENTS core)

add_executable(${PROJECT_NAME} "${CMAKE_CURRENT_SOURCE_DIR}/src/PuRe.hpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/PuRe.cpp")
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 17)
target_link_libraries(${PROJECT_NAME} PUBLIC Ommatidia::Ommatidia opencv_core opencv_imgproc)