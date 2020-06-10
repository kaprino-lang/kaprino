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
        git \
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

COPY --from=build-env /usr/local/bin/kprc /usr/local/bin/kprc
COPY --from=build-env /usr/local/lib/libantlr4-runtime.so.4.8 /usr/local/lib/libantlr4-runtime.so.4.8

WORKDIR /

RUN \
    apk update; \
    apk add --no-cache --virtual builddep \
        wget; \
    apk add --no-cache \
        binutils \
        clang \
        musl-dev \
        gcc; \
    apk del builddep; \
    mkdir /tmp; \
    rm -rf /var/cache/apk/*;
