# nigde

## Description

REST Api service to run

## Project structure

The project consists of rust packages combined into a single workspace
**Packages:**
- nigde - Rest service, for controlling the processor via the API
- fpga_hps_programmator - Firmware loading into FPGA and other low-level interaction between HPS and FPGA

**Other packages:**
- simple_led_driver - simplest example of rust userspace driver

## Dependencies

- [axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio, Tower, and Hyper
- [libc](https://github.com/rust-lang/libc) - Raw bindings to platform APIs for Rust

## Build project

If you need to build for `arm-unknown-linux-gnueabihf` platform, use Dockerfile:

```
docker built -t nigde_arm_builder
docker run --rm -v .:/app nigde_arm_builder:latest
```

After completion, the build output will be placed to `./target/arm-unknown-linux-gnueabihf/release/nigde`

## Build led driver example

```bash
cd simple_led_driver
cargo build
```
