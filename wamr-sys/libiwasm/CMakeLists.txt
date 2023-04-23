# Copyright (C) 2019 Intel Corporation.  All rights reserved.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

cmake_minimum_required (VERSION 2.8)

project (iwasm)

set (SGX_SDK_DIR /home/nskernel/linux-sgx-san/linux/installer/bin/sgxsdk)

set (WAMR_BUILD_PLATFORM "linux-sgx")

#set (CMAKE_C_COMPILER "clang")
#set (CMAKE_CXX_COMPILER "clang++")


set (CMAKE_C_COMPILER "/home/nskernel/llvm-project/build/bin/clang")
set (CMAKE_CXX_COMPILER "/home/nskernel/llvm-project/build/bin/clang++")

# Reset default linker flags
set (CMAKE_SHARED_LIBRARY_LINK_C_FLAGS "")
set (CMAKE_SHARED_LIBRARY_LINK_CXX_FLAGS "")

# Set WAMR_BUILD_TARGET
if (NOT DEFINED WAMR_BUILD_TARGET)
  if (CMAKE_SIZEOF_VOID_P EQUAL 8)
    # Build as X86_64 by default in 64-bit platform
    set (WAMR_BUILD_TARGET "X86_64")
  else ()
    # Build as X86_32 by default in 32-bit platform
    set (WAMR_BUILD_TARGET "X86_32")
  endif ()
endif ()

if (NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif ()

if (NOT DEFINED WAMR_BUILD_INTERP)
  # Enable Interpreter by default
  set (WAMR_BUILD_INTERP 1)
endif ()

if (NOT DEFINED WAMR_BUILD_AOT)
  # Enable AOT by default
  # Please install Intel SGX SDKv2.8 or later.
  set (WAMR_BUILD_AOT 1)
endif ()

if (NOT DEFINED WAMR_BUILD_JIT)
  # Disable JIT by default.
  set (WAMR_BUILD_JIT 0)
endif ()

if (NOT DEFINED WAMR_BUILD_LIBC_BUILTIN)
  # Enable libc builtin support by default
  set (WAMR_BUILD_LIBC_BUILTIN 1)
endif ()

if (NOT DEFINED WAMR_BUILD_LIBC_WASI)
  # Enable libc wasi support by default
  set (WAMR_BUILD_LIBC_WASI 1)
endif ()

if (NOT DEFINED WAMR_BUILD_FAST_INTERP)
  # Enable fast interpreter
  set (WAMR_BUILD_FAST_INTERP 1)
endif ()

if (NOT DEFINED WAMR_BUILD_MULTI_MODULE)
  # Enable multiple modules
  set (WAMR_BUILD_MULTI_MODULE 0)
endif ()

if (NOT DEFINED WAMR_BUILD_LIB_PTHREAD)
  # Enable pthread library by default
  set (WAMR_BUILD_LIB_PTHREAD 1)
endif ()

if (NOT DEFINED WAMR_BUILD_LIBC_EMCC)
  set ( WAMR_BUILD_LIBC_EMCC 1)
endif ()

if (NOT DEFINED WAMR_BUILD_SIMD)
  # Disable SIMD by default
  set (WAMR_BUILD_SIMD 0)
endif ()

if (COLLECT_CODE_COVERAGE EQUAL 1)
  set (CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fprofile-arcs -ftest-coverage")
endif ()

set (CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,--gc-sections")
set (CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=gnu99 -ffunction-sections -fdata-sections \
                                     -Wall -Wno-unused-parameter -Wno-pedantic \
                                     -nostdinc -fvisibility=hidden -fpie \
                                     -fno-zero-initialized-in-bss -fno-jump-tables")

set (WAMR_ROOT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../wasm-micro-runtime)

include (${WAMR_ROOT_DIR}/build-scripts/runtime_lib.cmake)
add_library(vmlib ${WAMR_RUNTIME_LIB_SOURCE})

# Build the enclave
## Generate Enclave_t.c
## Will be generated in Makefile

## Build enclave
add_custom_command (
  OUTPUT enclave.signed.so
  COMMAND cp libvmlib.a ${PLATFORM_SHARED_DIR}/Enclave-san/
  COMMAND cd ${PLATFORM_SHARED_DIR}/Enclave-san/ && make SGX_MODE=HW && cd -
  COMMAND cp ${PLATFORM_SHARED_DIR}/Enclave-san/enclave.signed.so ./
  DEPENDS vmlib
)

add_custom_target (enclave_bin ALL DEPENDS enclave.signed.so)

# Generate Enclave_u.c
set(SGX_EDGER8R ${SGX_SDK_DIR}/bin/x64/sgx_edger8r)
add_custom_command (
             OUTPUT ${PLATFORM_SHARED_DIR}/untrusted/Enclave_u.c ${PLATFORM_SHARED_DIR}/untrusted/Enclave_u.h
             COMMAND cd ${PLATFORM_SHARED_DIR}/untrusted/ && ${SGX_EDGER8R} --untrusted ../Enclave-san/Enclave.edl
                     --search-path ../Enclave-san 
		                 --search-path ${SGX_SDK_DIR}/include 
		                 --search-path ${WAMR_ROOT_DIR}/core/shared/platform/linux-sgx && cd -)

add_custom_command (
             OUTPUT libvmlib_untrusted.a
             COMMAND mkdir -p untrusted && cd untrusted &&
                     clang -c -fPIC -I${SGX_SDK_DIR}/include ${PLATFORM_SHARED_SOURCE_UNTRUSTED} ${PLATFORM_SHARED_DIR}/untrusted/Enclave_u.c
             COMMAND ${CMAKE_AR} rc libvmlib_untrusted.a untrusted/*.o
             DEPENDS ${PLATFORM_SHARED_DIR}/untrusted/Enclave_u.c ${PLATFORM_SHARED_DIR}/untrusted/Enclave_u.h)

add_custom_target (vmlib_untrusted ALL DEPENDS libvmlib_untrusted.a)
