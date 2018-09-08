cmake_minimum_required(VERSION 3.12)
project(Adapter)

set(CMAKE_CXX_STANDARD 14)

add_executable(Adapter main.cpp Adapter.cpp Adapter.h IPlayer.cpp IPlayer.h OldPlayer.cpp OldPlayer.h)