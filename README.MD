
See https://medium.com/digitalfrontiers/rust-on-a-stm32-microcontroller-90fac16f6342

Prerequisites: 
 * cmake

Rustup installations:
```
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils probe-rs-tools cargo-expand
``` 

# Kompilieren
`cargo build --target thumbv7em-none-eabihf`

# Starten auf dem Target (alles in einem):
`cargo embed`

# ST-Link / J-Link
## ST-Link
Mit ST-Link kann man nicht debuggen (oder?). Aber flashen/rtt funktioniert. Sollte OTG funktionieren

Wichtige Jumper einstellen:
* Beide ST-LINK Jumper drauf
* PWR-Jumper auf U5V

## J-Link
Damit J-Link nutzbar ist, vorher mit Zadic den generischen WinUSB-Treiber installieren. Es kann sein, dass die Probe nicht aufgelistet ist. Dies über "Options" -> "List All Devices" anpassen

### J-Link Pins
Wichtige Jumper einstellen:
* Beide ST-LINK Jumper drauf
* PWR-Jumper auf E5V

Das Gerät über USB versorgen. Danach J-Link anschließen

SWD-Pinout Von oben nach unten:
1. 3.3V
2. SWCLK
3. GND
4. SW(d)IO
5. NRST
6. (SWO)

Wichtig: Pin 1 muss aktiv betrieben werden. Zb über den Arduino-Pin 3.3V!
