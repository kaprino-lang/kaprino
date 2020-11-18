FROM kaprino/llvm:latest

COPY . /tmp/kaprino/

RUN \
    ####################################
    #
    # Install dependencies
    #
    ####################################
    cd /tmp/kaprino \
    && apt-get update \
    && apt-get install -y libffi-dev \
    ####################################
    #
    # Run tests
    #
    ####################################
    && cargo test
