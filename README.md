# hackxpansion module 1: SD Card Reader and Volume Control

**Crates: pkg:cargo/SD_card_driver@0.1.1**

A custom SD card module with three buttons for music storage and controls, using Xpanse API for hardware/module integration.

# Images

![final view](https://cdn.hackclub.com/01a05aec-dbe9-77a6-8141-e38b6f955840/image.png)

![3d model](https://cdn.hackclub.com/01a035e9-1e94-7a2f-9cb1-4ce6dbba37ed/image.png)

![pcb](https://cdn.hackclub.com/01a0bb3c-b34c-7de7-be92-ac019984ff51/image.png)
### Firmware

The firmware is responsible for communicating with the hardware.

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

See the [Xpanse API docs](https://docs.rs/xpanse-api/latest/xpanse_api/index.html)

---

## SD Card Filesystem

Its purpose is to abstract away the lower-level SD card communication so the application can work with files rather than dealing directly with SPI commands.

The filesystem layer provides functionality for:

* Opening the SD card
* Opening the first volume
* Opening the root directory
* Reading files
* Writing files
* Listing files in the root directory
* Closing directories and volumes

This is what the stack looks like:

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
Application
```

This means the application itself does not need to know how the SD card communicates over SPI.

---

This was all possible thanks to [Hackspansion: A hackclub YSWS](http://hackxpansion.hackclub.com/)

