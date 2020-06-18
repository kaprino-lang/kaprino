# How to build

## Requirements

- CMake (>=3.15)
- LLVM (9.0.0,9.0.1)
- JDK (latest)
- A compiler which supports C++17

## Build commands

Replace `[path]`s to the pathes in your environment and run the following commands.

```
$ git clone https://github.com/kaprino-lang/kaprino.git
$ cd kaprino
$ mkdir build
$ cd build
$ cmake ..
$ make
$ make install
```

## Build options

 We have some build options and you can change the following options
by passing them as CMake options like `-DKAPRINO_BUILD_STDLIB=OFF`.

|Name|Type|Description|Default|
|---|---|---|---|
|ANTLR_EXECUTABLE|FILEPATH|A path to antlr4 executable|${CMAKE_SOURCE_DIR}/antlr-4.8-complete.jar|
|KAPRINO_BUILD_STDLIB|BOOL|Build standard libraries|ON|
|KAPRINO_MONOTONE_LOG|BOOL|Make stdout monotone|OFF|
|KAPRINO_OPTIMIZER_ON|BOOL|Optimize output (**Experimental**)|OFF|
