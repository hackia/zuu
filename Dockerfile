FROM archlinux/archlinux:latest
ENV TESTS=false 
ENV FORMAT=false
ENV LINT=false
ENV AUDIT=false
RUN pacman -Syyu rustup base-devel cargo-audit cargo-auditable cargo-deny --noconfirm && rustup default stable
WORKDIR /usr/src/zuu
COPY . .
RUN cargo install --path . --root /usr/local
WORKDIR /app