if (NOT IS_DIRECTORY "${LLVM_ROOT}")
    message(FATAL_ERROR "Set LLVM_ROOT to the installed LLVM toolchain. LLVM is never downloaded by this build.")
endif()
if (NOT LLDB_LIBRARY)
    if (WIN32)
        set(LLDB_LIBRARY "${LLVM_ROOT}/bin/liblldb.dll")
    else()
        set(LLDB_LIBRARY "${LLVM_ROOT}/lib/liblldb${DylibSuffix}")
    endif()
endif()
if (NOT EXISTS "${LLDB_LIBRARY}")
    message(FATAL_ERROR "liblldb was not found at ${LLDB_LIBRARY}. Set LLDB_LIBRARY to the installed library.")
endif()

if (LLVM_SOURCE_DIR)
    find_program(PYTHON NAMES python python3 REQUIRED)
    set(GeneratedInclude "${CMAKE_BINARY_DIR}/include")
    file(MAKE_DIRECTORY "${GeneratedInclude}/lldb/API")
    execute_process(
        COMMAND ${PYTHON} "${LLVM_SOURCE_DIR}/lldb/scripts/generate-sbapi-dwarf-enum.py"
            "${LLVM_SOURCE_DIR}/llvm/include/llvm/BinaryFormat/Dwarf.def"
            -o "${GeneratedInclude}/lldb/API/SBLanguages.h"
        COMMAND_ERROR_IS_FATAL ANY)
    list(APPEND LLDB_INCLUDE_DIRS "${LLVM_SOURCE_DIR}/lldb/include" "${GeneratedInclude}")
endif()
if (NOT LLDB_INCLUDE_DIRS)
    set(LLDB_INCLUDE_DIRS "${LLVM_ROOT}/include")
endif()
find_path(LLDB_API_INCLUDE lldb/API/LLDB.h PATHS ${LLDB_INCLUDE_DIRS} NO_DEFAULT_PATH REQUIRED)
find_path(LLDB_GENERATED_INCLUDE lldb/API/SBLanguages.h PATHS ${LLDB_INCLUDE_DIRS} NO_DEFAULT_PATH REQUIRED)

set(LLDB_INCLUDE "${LLDB_INCLUDE_DIRS}" PARENT_SCOPE)
set(LLDB_LIBRARY "${LLDB_LIBRARY}" PARENT_SCOPE)
add_custom_target(lldb)
