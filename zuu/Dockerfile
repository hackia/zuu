FROM archlinux/archlinux:latest
ENV TESTS=false \
    FORMAT=false \
    LINT=false \
    AUDIT=false \
    LANGUAGE=rust
RUN pacman -Syyu  --noconfirm nodejs curl npm python3 base-devel cmake gcc rustup && rustup default stable && cargo install cargo-audit cargo-auditable cargo-deny cargo-outdated  --root /usr/local
WORKDIR /app