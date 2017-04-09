cmake_minimum_required (VERSION 3.7.2)
project (AssortedWidgets CXX)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake/Modules)

include_directories(AssortedWidgets)

message("build for " ${CMAKE_SYSTEM_NAME})

file(GLOB SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/AssortedWidgets/*.cpp")
file(GLOB DEMO_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/demo/*.cpp")


if("${CMAKE_SYSTEM_NAME}" STREQUAL "Emscripten")

if(DEFINED HTML_TEMPLATE)

message("using template " ${HTML_TEMPLATE})

set(TEMPLATE_OPTION "--shell-file ${HTML_TEMPLATE}")

endif()

set(CMAKE_CXX_FLAGS "-O3 -s USE_SDL=2 -s WASM=1 -std=c++11 -s USE_SDL_IMAGE=2 -s FULL_ES2=1 -s SDL2_IMAGE_FORMATS='[\"png\"]' --preload-file ${CMAKE_CURRENT_SOURCE_DIR}/assets@assets ${CMAKE_CXX_FLAGS}")

add_executable (AssortedWidgets     
				${SOURCES}
    			${DEMO_SOURCES})

file(COPY ${CMAKE_CURRENT_SOURCE_DIR}/assets/index.html
        DESTINATION ${CMAKE_CURRENT_BINARY_DIR})

else()

set(CMAKE_CXX_FLAGS "--std=gnu++11 ${CMAKE_CXX_FLAGS}")

if("${CMAKE_SYSTEM_NAME}" STREQUAL "Darwin")
find_package(OpenGL REQUIRED)
set(OPENGLES2_gl_LIBRARY ${OPENGL_LIBRARIES})
else()
find_package(OpenGLES2 REQUIRED)
endif()

find_package(SDL2 REQUIRED)
find_package(SDL2_image REQUIRED)

include_directories(${OPENGLES2_INCLUDE_DIR})
include_directories(${SDL2_INCLUDE_DIR})
include_directories(${SDL2IMAGE_INCLUDE_DIR})

message("sdl location: " ${SDL2_INCLUDE_DIR})
message("opengl es location: " ${OPENGLES2_INCLUDE_DIR})

add_executable (AssortedWidgets     
				${SOURCES}
    			${DEMO_SOURCES})

target_link_libraries(AssortedWidgets ${OPENGLES2_gl_LIBRARY} ${SDL2_LIBRARY} ${SDL2IMAGE_LIBRARY})

file(COPY ${CMAKE_CURRENT_SOURCE_DIR}/assets
        DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/)

endif()




