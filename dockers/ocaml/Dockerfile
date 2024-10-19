FROM alpine:latest
RUN apk add --no-cache bash ocaml ocaml-compiler-libs ocaml-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && source $HOME/.cargo/env \
    && rustup default stable \
    && cargo install cargo-audit cargo-auditable cargo-deny cargo-outdated
RUN cargo install --no-default-features --features ocaml --features cli --root /usr/local
WORKDIR /app
CMD ["zuu"]