FROM ubuntu:18.04

COPY . /tmp/kaprino/
WORKDIR /

RUN \
########################################################
#
# Install tools which are required.
#
########################################################
    apt update; \
    apt-get install -y \
        software-properties-common; \
    add-apt-repository ppa:openjdk-r/ppa; \
    add-apt-repository ppa:deadsnakes/ppa; \
    apt update; \
    apt upgrade; \
    apt install -y \
        pkg-config \
        curl \
        unzip \
        tar \
        build-essential \
        python3.8 \
        openjdk-11-jdk \
        cmake \
        clang \
        git; \
########################################################
#
# Build Kaprino
#
########################################################
    cd /tmp/kaprino; \
    python3 bootstrap.py --config Release; \
########################################################
#
# Clean unused files
#
########################################################
    apt remove --purge -y \
        python3.8 \
        openjdk-11-jdk \
        cmake \
        git \
    rm -rf /var/lib/apt/lists/*;

ENV KAPRINOPKG /usr/local/include
