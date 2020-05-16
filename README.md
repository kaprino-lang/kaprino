<div align="center">
  <h1>Kaprino <i>- A language of mathematical thinking</i></h1>
</div>

[![Latest release](https://img.shields.io/github/v/release/capra314cabra/kaprino?include_prereleases)](https://hub.docker.com/r/capra314cabra/kaprino)
[![CI](https://github.com/capra314cabra/kaprino/workflows/CI/badge.svg)](https://github.com/capra314cabra/kaprino/actions?query=workflow%3ACI)

This is a programming language that tastes like a mathematics.  
I hope this project may have changed your coding life.

## Example code

```
// A integer variable
#let i = 0 (<- Z).

// A real number variable
#let r = 0.0 (<- R).

// Assign the values to the variables
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

## Installation

### Run kaprino using Docker

```
docker run --rm -it capra314cabra/kaprino:latest
```
