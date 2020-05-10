FROM alpine:3.9.6

COPY . /tmp/kaprino/

########################################################
#
# Install tools which are required.
#
########################################################
WORKDIR /

RUN \
    apk update; \
    apk add --no-cache \
        alpine-sdk \
        wget \
        zip \
        clang \
        python3 \
        openjdk11 \
        pkgconfig \
        libuuid; \
    rm -rf /var/cache/apk/*;

########################################################
#
# Install CMake by building it from the sources.
#
########################################################
WORKDIR /tmp/cmake

RUN \
    wget https://github.com/Kitware/CMake/releases/download/v3.16.5/cmake-3.16.5.tar.gz; \
    tar -zxvf cmake-3.16.5.tar.gz; \
    cd cmake-3.16.5; \
    ./bootstrap -- -DCMAKE_BUILD_TYPE:STRING=Release; \
    make; \
    make install;

########################################################
#
# Install LLVM libs by building it from the sources.
#
########################################################
ENV LLVM_BIN_DIR /tmp/llvm/build/Release/bin
ENV LLVM_INCLUDE_DIR /tmp/llvm/include
ENV LLVM_LIB_DIR /tmp/llvm/build/Release/lib

WORKDIR /tmp/llvm

RUN \
    wget https://github.com/llvm/llvm-project/releases/download/llvmorg-10.0.0/llvm-10.0.0.src.tar.xz; \
    tar -xvf llvm-10.0.0.src.tar.xz; \
    mv /tmp/llvm/llvm-10.0.0.src /tmp/llvm;

WORKDIR /tmp/llvm/build

RUN \
    cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_BUILD_TOOLS=FALSE ..; \
    make;

########################################################
#
# Install ANTLR4 by building it from the sources.
#
########################################################
ENV ANTLR4_DOWNLOAD_URL https://www.antlr.org/download/antlr4-cpp-runtime-4.8-source.zip
ENV ANTLR4_INCLUDE_DIR /tmp/antlr4/runtime/src
ENV ANTLR4_LIB_DIR /tmp/antlr4/dist

ENV CLASSPATH '.:/tmp/antlr4/antlr-4.8-complete.jar:${CLASSPATH}'

WORKDIR /tmp/antlr4

RUN \
    wget $ANTLR4_DOWNLOAD_URL; \
    unzip $(basename $ANTLR4_DOWNLOAD_URL); \
    rm $(basename $ANTLR4_DOWNLOAD_URL); \
    wget https://www.antlr.org/download/antlr-4.8-complete.jar; \
    printf '#!/bin/bash\njava -jar /tmp/antlr4/antlr-4.8-complete.jar "$@"' > /usr/bin/antlr4 && \
        chmod +x /usr/bin/antlr4; \
    printf '#!/bin/bash\njava org.antlr.v4.gui.TestRig "$@"' > /usr/bin/grun && \
        chmod +x /usr/bin/grun;

WORKDIR /tmp/antlr4/build

RUN \
    cmake ..; \
    make;

########################################################
#
# Build Kaprino
#
########################################################
WORKDIR /tmp/kaprino/build

RUN \
    cmake .. \
        -DANTLR4_IncludePath=${ANTLR4_INCLUDE_DIR} \
        -DANTLR4_LibPath=${ANTLR4_LIB_DIR} \
        -DLLVM_IncludePath=${LLVM_INCLUDE_DIR} \
        -DLLVM_LibPath=${LLVM_LIB_DIR}; \
    make; \
    ln -s /tmp/kaprino/build/kaprino /usr/bin/kaprino;