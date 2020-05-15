FROM capra314cabra/llvm-alpine-libs:9.0.1 AS build-env

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
        python3 \
        build-base \
        util-linux-dev \
        ninja \
        openjdk11 \
        cmake \
        wget \
        zip;

########################################################
#
# Install LLVM.
#
########################################################
ENV LLVM_INCLUDE_DIR /usr/local/include
ENV LLVM_LIB_DIR /usr/local/lib

########################################################
#
# Install ANTLR4 by building it from the sources.
#
########################################################
ENV ANTLR4_DOWNLOAD_URL https://www.antlr.org/download/antlr4-cpp-runtime-4.8-source.zip
ENV ANTLR4_INCLUDE_DIR /usr/local/include/antlr4-runtime/
ENV ANTLR4_LIB_DIR /usr/local/lib

ENV CLASSPATH '.:/root/antlr4/antlr-4.8-complete.jar:${CLASSPATH}'

WORKDIR /root/antlr4

RUN \
    wget https://www.antlr.org/download/antlr-4.8-complete.jar; \
    echo '#!/bin/sh' > /usr/bin/antlr4; \
    echo 'java -jar /root/antlr4/antlr-4.8-complete.jar "$@"' >> /usr/bin/antlr4; \
    chmod +x /usr/bin/antlr4 \
    echo '#!/bin/sh' > /usr/bin/grun; \
    echo 'java org.antlr.v4.gui.TestRig "$@"' >> /usr/bin/grun; \
    chmod +x /usr/bin/grun;

WORKDIR /tmp/antlr4

RUN \
    wget $ANTLR4_DOWNLOAD_URL; \
    unzip $(basename $ANTLR4_DOWNLOAD_URL); \
    rm $(basename $ANTLR4_DOWNLOAD_URL);

WORKDIR /tmp/antlr4/build

RUN \
    cmake -G Ninja ..; \
    ninja; \
    ninja install;

########################################################
#
# Build Kaprino
#
########################################################
WORKDIR /tmp/kaprino/build

RUN \
    cmake .. \
        -G Ninja \
        -DANTLR4_IncludePath=${ANTLR4_INCLUDE_DIR} \
        -DANTLR4_LibPath=${ANTLR4_LIB_DIR} \
        -DLLVM_IncludePath=${LLVM_INCLUDE_DIR} \
        -DLLVM_LibPath=${LLVM_LIB_DIR}; \
    ninja; \
    ninja install;

########################################################
#
# Deploy stage
#
########################################################

FROM capra314cabra/llvm-alpine-libs:9.0.1

COPY --from=build-env /usr/local/bin/kaprino /usr/local/bin/kaprino

ENV CLASSPATH '.:/root/antlr4/antlr-4.8-complete.jar:${CLASSPATH}'

WORKDIR /root/antlr4

RUN \
    apk update; \
    apk add --no-cache --virtual builddep \
        wget; \
    apk add --no-cache \
        clang \
        python3; \
    wget https://www.antlr.org/download/antlr-4.8-complete.jar; \
    echo '#!/bin/sh' > /usr/bin/antlr4; \
    echo 'java -jar /root/antlr4/antlr-4.8-complete.jar "$@"' >> /usr/bin/antlr4; \
    chmod +x /usr/bin/antlr4 \
    echo '#!/bin/sh' > /usr/bin/grun; \
    echo 'java org.antlr.v4.gui.TestRig "$@"' >> /usr/bin/grun; \
    chmod +x /usr/bin/grun; \
    apk del builddep; \
    rm -rf /var/cache/apk/*;
