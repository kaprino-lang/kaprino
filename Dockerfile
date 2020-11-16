FROM kaprino/llvm:latest

COPY . ~/tmp/kaprino/

RUN \
########################################################
#
# Build Kaprino
#
########################################################
    cd ~/tmp/kaprino \
    && cargo test
