FROM ubuntu:18.04

COPY . /tmp/kaprino/
WORKDIR /

RUN \
########################################################
#
# Install tools which are required.
#
########################################################
    add-apt-repository ppa:openjdk-r/ppa; \
    apt update; \
    apt upgrade; \
    apt install \
        curl \
        build-essential \
        python3 \
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
    apt remove --purge \
        python3 \
        openjdk10 \
        cmake \
        git \
    rm -rf /var/lib/apt/lists/*;

ENV KAPRINOPKG /usr/local/include
