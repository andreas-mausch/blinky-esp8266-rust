This project uses the [esp8266-hal](https://github.com/esp-rs/esp8266-hal)
to show you can develop for the ESP8266 in Rust.

Blog post about this project:
[https://andreas-mausch.de/blog/2024-10-20-esp01s/](https://andreas-mausch.de/blog/2024-10-20-esp01s/)

Note: `esp8266-hal` is deprecated and missing important features like WiFi.
But I was still interested if this would work at all.

I found a project which does exactly what I am trying to achieve:
[blinky-esp8266-rust](https://github.com/coenraadhuman/blinky-esp8266-rust/)

However, it is a bit dated and uses a `esprs/espflash` docker image,
which is also dated.
I prefer to run the compilation without docker on my machine.

So here is my version of it.

# Requirements

- rustup (>= 1.27.1)
- rust (>= 1.82.0)
- cargo (>= 1.82.0)
- cargo-espflash (exactly 2.1.0)
  Can be installed via `cargo binstall cargo-espflash@2.1.0`
- Rust xtensa-lx106 toolchain
  Can be installed via `espup install --toolchain-version 1.80.0.0`

Note: Newer versions might not have esp8266 support,
see here: [Support for xtensa-esp8266-none-elf removed in 1.81](https://github.com/esp-rs/rust/issues/237)

`cargo-espflash` has removed support for the ESP8266, see here:
[RFC: Remove support for ESP8266](https://github.com/esp-rs/espflash/issues/519)

# Build

Make sure to source the esp toolchain:

```bash
source ~/export-esp.sh
```

```bash
cargo build
# or:
cargo build --release
```

# Flash to ESP-01S

```bash
cargo espflash flash --baud=115200 --flash-mode=qio --flash-freq=40mhz --flash-size=1mb --port=/dev/ttyUSB1 --release --monitor
```
