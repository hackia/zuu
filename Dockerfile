FROM archlinux/archlinux

RUN pacman -Syyu --noconfirm base
RUN rustup default stable
