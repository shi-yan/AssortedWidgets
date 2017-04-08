cmake_minimum_required (VERSION 3.7.2)
project (AssortedWidgets CXX)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake/Modules)

set (CMAKE_CXX_FLAGS "--std=gnu++11 ${CMAKE_CXX_FLAGS}")

option(BUILD_WASM "build WebAssembly" OFF)

include_directories(AssortedWidgets)

find_package(OpenGLES2 REQUIRED)
find_package(SDL2 REQUIRED)
find_package(SDL2_image REQUIRED)

include_directories(${OPENGLES2_INCLUDE_DIR})
include_directories(${SDL2_INCLUDE_DIR})
include_directories(${SDL2IMAGE_INCLUDE_DIR})

# message(${OPENGLES2_INCLUDE_DIR})
# message(${OPENGLES2_gl_LIBRARY})

file(GLOB SOURCES "AssortedWidgets/*.cpp")
file(GLOB DEMO_SOURCES "demo/*.cpp")

add_executable (AssortedWidgets     
				${SOURCES}
    			${DEMO_SOURCES})


target_link_libraries(AssortedWidgets ${OPENGLES2_gl_LIBRARY} ${SDL2_LIBRARY} ${SDL2IMAGE_LIBRARY})