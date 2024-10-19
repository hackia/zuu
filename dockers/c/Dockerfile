FROM otechdo/zuu
RUN paman -Syyu cunit lcov check cmocka valgrind splint criterion tcc musl ffmpeg lld lldb lldb clang ctest cmake gdb base-devel libtool polly automake autoconf && cargo install zuu --no-default-features --features c --root /usr/local
