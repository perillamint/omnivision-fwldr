<!--
SPDX-FileCopyrightText: 2023 perillamint

SPDX-License-Identifier: MIT
-->

# PS4/PS5 camera firmware loader (or, Omnivision Boot Firmware Loader)

This project implements the OmniVision USB Boot protocol in Rust, which is used by

* PlayStation camera series

### How to use (easy version, for ps5 camera)

1. Install Rust
2. Clone this repository
3. run `get_firmware.sh`
4. run `cargo build --release`
5. run `install.sh`
6. Replug the camera