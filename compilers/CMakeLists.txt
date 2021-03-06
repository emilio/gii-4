cmake_minimum_required(VERSION 3.7)

# In order to use YouCompleteMe with Vim.
set(CMAKE_EXPORT_COMPILE_COMMANDS "ON")

# Download and unpack gtest at configure time
configure_file(cmake/GTest.txt.in gtest-download/CMakeLists.txt)
execute_process(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" .
  RESULT_VARIABLE result
  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/gtest-download )
if(result)
  message(FATAL_ERROR "CMake step for googletest failed: ${result}")
endif()
execute_process(COMMAND ${CMAKE_COMMAND} --build .
  RESULT_VARIABLE result
  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/gtest-download )
if(result)
  message(FATAL_ERROR "Build step for gtest failed: ${result}")
endif()

# Prevent overriding the parent project's compiler/linker
# settings on Windows
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

add_subdirectory(${CMAKE_BINARY_DIR}/googletest-src
                 ${CMAKE_BINARY_DIR}/googletest-build)

add_library(base OBJECT
  src/AST.cc
  src/Parser.cc
  src/Tokenizer.cc
  src/Value.cc
)

include_directories(${CMAKE_SOURCE_DIR}/src)

set(UNIT_TESTS
  Parser
  Tokenizer
  Evaluator
)

enable_testing()
foreach(unit_test ${UNIT_TESTS})
  add_executable(${unit_test}Test tests/${unit_test}Test.cc
    $<TARGET_OBJECTS:base>
  )
  target_link_libraries(${unit_test}Test gtest_main)
  add_test(NAME ${unit_test} COMMAND ${unit_test}Test)
endforeach()

set(EXECUTABLES
  Tokenizer
  Dumper
  Evaluator
)

foreach(exec ${EXECUTABLES})
  add_executable(${exec} bin/${exec}.cc
    $<TARGET_OBJECTS:base>
  )
endforeach()
