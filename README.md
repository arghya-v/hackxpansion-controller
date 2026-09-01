# hackxpansion module 1: SD Card Reader and Volume Control

**For mp3 player app, visit:** ![hackxpansion mp3 player](https://github.com/arghya-v/hackxpansion-mp3-player)

A custom embedded MP3 player built around the hackxpansion , using an SD card for music storage, Xpanse API for hardware/module integration, and Slint for the graphical user interface.

The goal of this project is to create a compact, modular music player that can read music directly from an SD card and provide a simple dedicated interface for controlling playback.
![final view](https://cdn.hackclub.com/01a05aec-dbe9-77a6-8141-e38b6f955840/image.png)

![3d model](https://cdn.hackclub.com/01a035e9-1e94-7a2f-9cb1-4ce6dbba37ed/image.png)

![pcb](https://cdn.hackclub.com/01a05eb2-c827-7c26-9e02-c40adda546ac/image.png)
### Firmware

The firmware is responsible for communicating with the hardware.

It uses:

* `embassy-rp` for RP-series microcontroller peripherals
* `xpanse-api` for HackXPansion hardware resources
* `embedded-sdmmc` for interacting with the SD card filesystem
* `embedded-hal` / `embedded-hal-bus` for hardware abstraction

The SD card is connected through SPI.

The current hardware mapping uses:

| GPIO  | Function   |
| ----- | ---------- |
| GPIO2 | SPI SCK    |
| GPIO3 | SPI MISO   |
| GPIO4 | SPI MOSI   |
| GPIO5 | Button A   |
| GPIO6 | Button B   |
| GPIO7 | Button X   |
| GPIO9 | SD card CS |

The exact hardware configuration is defined by the Xpanse module and firmware implementation.

---

## SD Card Filesystem

`filesystem.rs` provides the filesystem layer used by the MP3 player.

Its purpose is to abstract away the lower-level SD card communication so the application can work with files rather than dealing directly with SPI commands.

The filesystem layer provides functionality for:

* Opening the SD card
* Opening the first volume
* Opening the root directory
* Reading files
* Writing files
* Listing files in the root directory
* Closing directories and volumes

Conceptually, the stack looks like this:

```text
SD Card
   │
   ▼
SPI
   │
   ▼
Xpanse SpiBusHandle
   │
   ▼
embedded-sdmmc
   │
   ▼
Filesystem
   │
   ▼
MP3 Player Application
```

This means the application does not need to know how the SD card communicates over SPI.

---


# Xpanse Integration

Xpanse acts as the hardware abstraction layer between the application and the physical modules.

Instead of directly controlling every GPIO from the application, hardware resources can be registered and retrieved through the Xpanse registry.

For example, buttons are exposed as resources:

```rust
Box<dyn Button<A>>
Box<dyn Button<B>>
Box<dyn Button<X>>
```

An application can then request the resources it needs:

```rust
let button = registry.take_resource::<Box<dyn Button<A>>>()?;
```

This allows applications to remain relatively independent from the underlying hardware implementation.


