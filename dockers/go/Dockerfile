FROM otechdo/zuu
RUN pacman -Syyu delve gopls go-tools tinygo goreleaser yaegi staticcheck gox revive go ko--noconfirm  && cargo install --no-default-features --features go  --features cli  --root /usr/local
