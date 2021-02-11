FROM kaprino/llvm:latest

COPY . /root/kaprino/

RUN \
    ####################################
    #
    # Install dependencies
    #
    ####################################
    apt-get update \
    && apt-get install -y \
        clang \
    ####################################
    #
    # Build kaprino
    #
    ####################################
    && cd /root/kaprino \
    && cargo build --release \
    && cp ./target/release/kpr /usr/local/bin/kpr \
    && cp ./target/release/kprc /usr/local/bin/kprc \
    ####################################
    #
    # Eliminate garbage
    #
    ####################################
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /root/kaprino
