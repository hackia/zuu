FROM alpine:latest
RUN apk add --no-cache nodejs npm python3 make g++ bash rustup  \
    && rustup-init -y --default-toolchain stable \
    && source $HOME/.cargo/env \
    && cargo install cargo-audit cargo-auditable cargo-deny cargo-outdated \
    && cargo install --no-default-features --features ruby --features cli  --root /usr/local
WORKDIR /app
