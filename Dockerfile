FROM kaprino/llvm:latest

COPY . /tmp/kaprino/

RUN \
########################################################
#
# Build Kaprino
#
########################################################
    cd /tmp/kaprino \
    && apt-get install libffi-dev \
    && cargo test
