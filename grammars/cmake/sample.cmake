# A sample.
cmake_minimum_required(VERSION 3.20)
project(sample VERSION 1.2.3 LANGUAGES C CXX)

set(CMAKE_C_STANDARD 11)
option(SAMPLE_TESTS "Build the tests" ON)

set(SOURCES
  src/main.c
  src/shape.c
)

add_executable(sample ${SOURCES})
target_include_directories(sample PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_compile_definitions(sample PRIVATE LIMIT=10)

if(SAMPLE_TESTS AND NOT WIN32)
  enable_testing()
  add_test(NAME sample_test COMMAND sample --test)
elseif(MSVC)
  message(STATUS "no tests on MSVC")
else()
  message(WARNING "tests are off")
endif()

foreach(lib IN ITEMS m pthread)
  target_link_libraries(sample PRIVATE ${lib})
endforeach()

function(print_all)
  foreach(arg ${ARGN})
    message("${arg}")
  endforeach()
endfunction()

print_all(${SOURCES})
install(TARGETS sample RUNTIME DESTINATION bin)
