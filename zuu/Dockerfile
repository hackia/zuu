FROM archlinux/archlinux
ENV TESTS=false \
    FORMAT=false \
    LINT=false \
    AUDIT=false \
    LANGUAGE=rust
RUN pacman -Syyu cargo-about cargo-license cargo-asm cargo-about cargo-update cargo-tarpaulin cargo-flamegraph cargo-binstall cargo-spellcheck cargo-generate-rpm cargo-deb cargo-llvm-cov cargo-modules wget cargo-make cargo-update cargo-msrv cargo-machete cargo-outdated cargo-insta cargo-hack cargo-edit cargo-feature cargo-generate cargo-expand cargo-dist cargo-depgraph cargo-asm cargo-audit cargo-auditable cargo-binstall 	cargo-bloat  cargo-bundle-licenses --noconfirm nodejs curl git nodejs npm python3 base-devel cmake gcc rustup && rustup default stable 
WORKDIR /app