FROM otechdo/zuu
RUN git clone https://github.com/vlang/v /v \
    && cd /v && make
RUN cargo install --no-default-features --features vlang --root /usr/local