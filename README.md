# XMC4700 Relax (Lite) Kit – Rust Blink mit PlatformIO

## Voraussetzungen
- PlatformIO (VS Code oder CLI)
- Rust über rustup (`rust-toolchain.toml` installiert das Target automatisch)
- SEGGER J-Link Software (On-Board-Debugger des Kits)

## Benutzen
    pio run                  # baut via cargo
    pio run -t upload        # flasht über J-Link

Reiner Cargo-Build: `cargo build --release`

## Hinweise
- PlatformIO kennt `xmc4700_relax_kit`; das Lite Kit nutzt denselben XMC4700-Chip.
- LED-Pins (P5.8 / P5.9) und Taktung ggf. an die Board-Revision anpassen.
