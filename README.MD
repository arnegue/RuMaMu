# RuMaMu (**Ru**st**Ma**ritime**Mu**ltiplexer)

Multiplexer written in Rust for Maritime devices.

This project is currently setup on STM Nucleo-F411RE

Currently supports:
* Receiving 9Bit serial for Seatalk

See also my python-implementation [NMEA-Seatalk-Multiplexer](https://github.com/arnegue/NMEA-Seatalk-Multiplexer).

# Build
## Compile for target
`cargo build --target thumbv7em-none-eabihf`

## Compile and start on target
`cargo embed`

# Prerequisites:
 * cmake

## Rustup
```
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils probe-rs-tools cargo-expand
``` 