<div align="center">
  <h1>Kaprino <i>- A language of mathematical thinking</i></h1>
</div>

[![Build](https://github.com/kaprino-lang/kaprino/workflows/Build/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)
[![Latest release](https://img.shields.io/github/v/release/kaprino-lang/kaprino?include_prereleases)](https://github.com/kaprino-lang/kaprino/releases)
[![GitHub](https://img.shields.io/github/license/kaprino-lang/kaprino)](https://github.com/kaprino-lang/kaprino/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/kaprino-lang/kaprino)](https://github.com/kaprino-lang/kaprino/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/kaprino-lang/kaprino)](https://github.com/kaprino-lang/kaprino)

This is a strongly-typed programming language that tastes like a mathematics.

## BREAKING NEWS :loudspeaker:

We switched a language for developing kaprino from C++ to Rust in the view of maintainability. And kaprino written by Rust haven't been stable yet.

Wait a release of kaprino written by Rust!

If you want to access stable kaprino written by C++, please refer to [cpp-HEAD](https://github.com/kaprino-lang/kaprino/tree/cpp-HEAD) branch of this repository.

## Code snippets :memo:

![Code example](https://raw.githubusercontent.com/kaprino-lang/kaprino/master/img/code_example1.png)

## How to run :computer:

Write a code and save it with the extension, `.kpr`.  
Now, you can compile it by executing the following command:  
("kprc" is the shorteness of "kaprino compiler".)

```
$ kprc some-source.kpr
```

kprc will generate an executable.

If you are tired to set your environment, you can use [Kaprino Online Compiler](https://kaprino-lang.github.io/online).

## Installation :cd:

### Download from release page :floppy_disk:

Download achievements from [Release Page](https://github.com/kaprino-lang/kaprino/releases) and put the directory path into PATH.  
Here is the table of available environments.

|OS|Available|Status|
|---|:---:|:---:|
|Linux x64| ✔️ |[![Build](https://github.com/kaprino-lang/kaprino/workflows/Linux%20release/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)|
|Windows x64| ✔️ |[![Build](https://github.com/kaprino-lang/kaprino/workflows/Windows%20release/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)|
|MacOS x64| ✔️ |[![Build](https://github.com/kaprino-lang/kaprino/workflows/MacOS%20release/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)|

### Build from sources :hammer:

Please read [How to build](https://github.com/kaprino-lang/kaprino/blob/master/HowToBuild.md).

### Run kaprino using Docker :whale:

You can use a docker image which is already installed kaprino.

[![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/kaprino/kaprino)](https://hub.docker.com/r/kaprino/kaprino)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/kaprino/kaprino)](https://hub.docker.com/r/kaprino/kaprino/builds)
[![Docker Image Size](https://img.shields.io/docker/image-size/kaprino/kaprino/latest)](https://hub.docker.com/r/kaprino/kaprino)

```
docker run --rm -it kaprino/kaprino:latest
```

## Requirements

- Clang (>=9.0.1)

## License

[MIT license](https://github.com/kaprino-lang/kaprino/blob/master/LICENSE)
