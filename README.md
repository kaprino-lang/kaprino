<div align="center">
  <h1>Kaprino <i>- A language of mathematical thinking</i></h1>
</div>

[![Build](https://github.com/kaprino-lang/kaprino/workflows/Build/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)
[![Latest release](https://img.shields.io/github/v/release/kaprino-lang/kaprino?include_prereleases)](https://github.com/kaprino-lang/kaprino/releases)
[![GitHub](https://img.shields.io/github/license/kaprino-lang/kaprino)](https://github.com/kaprino-lang/kaprino/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/kaprino-lang/kaprino)](https://github.com/kaprino-lang/kaprino/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/kaprino-lang/kaprino)](https://github.com/kaprino-lang/kaprino)

<p align="center">
  <img width="314" height="314" src="https://raw.githubusercontent.com/kaprino-lang/kaprino/master/img/icon.png" />
</p>

**kaprino** is a new programming language which lies **great emphasis on the correctness of the result** of programs.  
It is recommended for people who want to do some **mathematical** stuffs **strictly**.

Also, its syntax is as close to mathematics as possible. It's because I would like to see it used by not only people who are familiar with programming but also those who are new to programming!

This project lacks some features yet. I would be appreciate it if you contribute to this project no matter how it is small.

---

## 📢 IMPORTANT NOTICE

We moved a language for developing kaprino from C++ to Rust in the view of maintainability.   
However, kaprino written by Rust haven't been stable yet, so if you want to access stable kaprino, written by C++, please refer to [cpp-HEAD](https://github.com/kaprino-lang/kaprino/tree/cpp-HEAD) branch in this repository.

---

## 💽 Installation

There are three ways available.

### 💾 Download from release page

Download achievements from [Release Page](https://github.com/kaprino-lang/kaprino/releases) and put the directory path into PATH.  
Here is the table of available environments.

|OS|Available|Status|
|---|:---:|:---:|
|Linux x64| ✔️ |[![Build](https://github.com/kaprino-lang/kaprino/workflows/Linux%20release/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)|
|Windows x64| ✔️ |[![Build](https://github.com/kaprino-lang/kaprino/workflows/Windows%20release/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)|
|MacOS x64| ✔️ |[![Build](https://github.com/kaprino-lang/kaprino/workflows/MacOS%20release/badge.svg)](https://github.com/kaprino-lang/kaprino/actions)|

### 🛠️ Build from sources

In preparation.

### 🐳 Run kaprino using Docker

You can use a docker image which is already installed kaprino.

[![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/kaprino/kaprino)](https://hub.docker.com/r/kaprino/kaprino)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/kaprino/kaprino)](https://hub.docker.com/r/kaprino/kaprino/builds)
[![Docker Image Size](https://img.shields.io/docker/image-size/kaprino/kaprino/latest)](https://hub.docker.com/r/kaprino/kaprino)

```
docker run --rm -it kaprino/kaprino:latest
```

## 📋 Requirements

- Clang (>=9.0.1)

## ⚖️ License

kaprino is under [MIT license](https://github.com/kaprino-lang/kaprino/blob/master/LICENSE).
