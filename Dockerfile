FROM alpine:edge

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
        clang \
        python3;

RUN \
    apk add --no-cache --virtual builddep \
        build-base \
        util-linux-dev \
        openjdk11 \
        cmake \
        wget \
        zip;

########################################################
#
# Install LLVM by building it from the sources.
#
########################################################
ENV LLVM_BIN_DIR /tmp/llvm/build/Release/bin
ENV LLVM_INCLUDE_DIR /tmp/llvm/include
ENV LLVM_LIB_DIR /tmp/llvm/build/Release/lib

WORKDIR /tmp/llvm

RUN \
    wget https://github.com/llvm/llvm-project/releases/download/llvmorg-9.0.1/llvm-9.0.1.src.tar.xz; \
    tar -xvf llvm-9.0.1.src.tar.xz; \
    mv /tmp/llvm/llvm-9.0.1.src /tmp && mv /tmp/llvm-9.0.1.src /tmp/llvm;

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
    wget https://www.antlr.org/download/antlr-4.8-complete.jar;

RUN \
    echo '#!/bin/sh' > /usr/bin/antlr4; \
    echo 'java -jar /tmp/antlr4/antlr-4.8-complete.jar "$@"' >> /usr/bin/antlr4; \
    chmod +x /usr/bin/antlr4;

RUN \
    echo '#!/bin/sh' > /usr/bin/grun; \
    echo 'java org.antlr.v4.gui.TestRig "$@"' >> /usr/bin/grun; \
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

########################################################
#
# Clean workspaces
#
########################################################

RUN \
    apk del builddep; \
    rm -rf /var/cache/apk/*;