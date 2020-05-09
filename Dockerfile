FROM klee/llvm:90_O_D_A_ubuntu_bionic-20200112

SHELL ["/bin/bash", "-c"]

COPY . /tmp/kaprino/

########################################################
#
# Install tools which are required.
#
########################################################
WORKDIR /

RUN \
    apt update && apt upgrade -y; \
    apt install -y \
        build-essential \
        wget \
        zip \
        g++ \
        clang \
        python3 \
        default-jre \
        pkg-config \
        libssl-dev \
        uuid-dev; \
    apt clean; \
    rm -rf /var/lib/apt/lists/*;

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
# Prepare to use LLVM
#
########################################################
ENV LLVM_BIN_DIR /tmp/llvm-90-install_O_D_A/bin
ENV LLVM_INCLUDE_DIR /tmp/llvm-90-install_O_D_A/include
ENV LLVM_LIB_DIR /tmp/llvm-90-install_O_D_A/lib

WORKDIR /tmp/llvm-90-install_O_D_A

RUN \
    ln -s ${LLVM_BIN_DIR}/lli /usr/bin/lli; \
    ln -s ${LLVM_BIN_DIR}/llvm-as /usr/bin/llvm-as;

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