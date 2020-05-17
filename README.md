<div align="center">
  <h1>Kaprino <i>- A language of mathematical thinking</i></h1>
</div>

[![Latest release](https://img.shields.io/github/v/release/capra314cabra/kaprino?include_prereleases)](https://hub.docker.com/r/capra314cabra/kaprino)
[![CI](https://github.com/capra314cabra/kaprino/workflows/CI/badge.svg)](https://github.com/capra314cabra/kaprino/actions?query=workflow%3ACI)

This is a programming language that tastes like a mathematics.  
I hope this project may have changed your coding life.

## Example code

```
// An integer variable
#let i = 0 (<- Z).

// A real number variable
#let r = 0.0 (<- R).

// Assign numbers to the variables
i := 5.
r := 4.3.

// Print the value.
#print i + r.

// Create a branch
#if i > 3.0 or r > 4
|>
    #print "OK!".
|>
#otherwise
|>
    #print "Error...".
|>

// Define a function
f(x, y) = x ^ 2 + y ^ 2 (R, R -> R).

// Run a function.
#print f(5.1, 6.7).

// Loop
#let counter = 0 (<- Z).
#loop
|>
    #if counter = 5
    |>
        #print "Exit loop".
        #exit.
    |>
    #print counter.
    counter := counter + 1.
|>
```

Write a code and save as `test.kpr`.  
Now, you can compile it by executing the following command:  
("kprc" is the shorteness of "kaprino compiler".)

```
$ kprc test.kpr
```

kprc will generate an executable.

## Requirements

- Clang (>=9.0.1)
- ANTLR4 C++ runtime (>= 4.8)

## Installation

### Build from sources

#### Build requirements

- CMake (>=3.15)
- LLVM (9.0.0,9.0.1)
- ANTLR4 (>=4.8)
- ANTLR4 C++ runtime (>= 4.8)

Note that you have to make sure able to use `antlr4` command.

#### How to build

Replace `[path]`s to the pathes in your environment and run it.

```
$ git clone https://github.com/capra314cabra/kaprino.git
$ cd kaprino
$ mkdir build
$ cd build
$ cmake .. -DANTLR4_IncludePath=[path] -DANTLR4_LibPath=[path] -DLLVM_IncludePath=[path] -DLLVM_LibPath=[path]
$ make
$ make install
```

### Run kaprino using Docker

You can use a docker image which is already installed kaprino.

```
docker run --rm -it capra314cabra/kaprino:latest
```
