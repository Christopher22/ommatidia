cmake_minimum_required(VERSION 3.15)
project(CPRD 
    VERSION 0.1
    DESCRIPTION "CPRD"
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
find_package(TBB REQUIRED)
find_package(Boost REQUIRED)

add_executable(${PROJECT_NAME} "${CMAKE_CURRENT_SOURCE_DIR}/src/utils.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/PupilDetectorHaar.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/PupilDetectorHaar.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/ConicSection.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/cvx.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/cvx.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/high_resolution_timer.hpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/PupilTracker.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/PupilTracker.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/timer.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/utils.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/Swirski2012/utils.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp" )
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 17)
target_link_libraries(${PROJECT_NAME} PUBLIC Ommatidia::Ommatidia opencv_core opencv_imgproc ${TBB_LIBRARIES} Boost::boost)