cmake_minimum_required(VERSION 3.22.1)
project("seedvault_tipc")

add_library(
        seedvault_tipc
        SHARED
        tipc_jni.c)

find_library(log-lib log)

target_link_libraries(
        seedvault_tipc
        ${log-lib})
