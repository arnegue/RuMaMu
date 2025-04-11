# RuMaMu (**Ru**st**Ma**ritime**Mu**ltiplexer)

Multiplexer written in Rust for Maritime devices.

This project is currently setup on STM Nucleo-F411RE

Currently supports:
* Receiving 9Bit serial for Seatalk

See also my python-implementation [NMEA-Seatalk-Multiplexer](https://github.com/arnegue/NMEA-Seatalk-Multiplexer).

# Build
## Compile
Either or:
* `cargo build --thumbv7em-none-eabihf`
* `cargo build --target x86_64-pc-windows-msvc`
* `cargo build --target x86_64-unknown-linux-gnu`

## Compile and start on target
`cargo embed`

# Test
`cargo test --target x86_64-pc-windows-msvc`

# Prerequisites:
 * cmake

## Rustup
```
rustup update
rustup target add thumbv7em-none-eabihf
``` 
