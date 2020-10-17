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
# Build Kaprino
#
########################################################
WORKDIR /tmp/kaprino

RUN \
    chmod +x bootstrap.sh; \
    . bootstrap.sh;

########################################################
#
# Deploy stage
#
########################################################

FROM capra314cabra/llvm-alpine-libs:9.0.1

COPY --from=build-env /usr/local/bin/kprc /usr/local/bin/kprc
COPY --from=build-env /usr/local/include/stdlib /usr/local/include/stdlib

WORKDIR /

RUN \
    apk update; \
    apk add --no-cache \
        binutils \
        clang \
        musl-dev \
        gcc; \
    mkdir /tmp; \
    rm -rf /var/cache/apk/*;

ENV KAPRINOPKG /usr/local/include
