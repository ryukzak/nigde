FROM rust:latest as builder
 
RUN apt update && apt upgrade -y
RUN apt install -y gcc-arm-linux-gnueabihf libc6-dev-armhf-cross
 
RUN rustup target add arm-unknown-linux-gnueabihf
RUN rustup toolchain install stable-arm-unknown-linux-gnueabihf
RUN rustup component add clippy


WORKDIR /app 
 
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc CC_arm_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc CXX_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
 

ENV CLIPPY_CMD="cargo clippy --release --target arm-unknown-linux-gnueabihf -- --D warnings"
ENV RUSTFMT_CMD="cargo fmt --target arm-unknown-linux-gnueabihf --check"
ENV TESTS_CMD="cargo test"
ENV BUILD_CMD="cargo build --release --target arm-unknown-linux-gnueabihf"
CMD $RUSTFMT_CMD ; $TESTS_CMD ; $CLIPPY_CMD ; $BUILD_CMD
