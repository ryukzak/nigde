# Ubuntu 20.04

```
sudo apt-get install -qq gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-musleabihf

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc

# if C dependencies are involved
export CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc

cargo build --target armv7-unknown-linux-musleabihf

```
