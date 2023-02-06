#!/bin/bash

# SPDX-FileCopyrightText: 2023 perillamint
#
# SPDX-License-Identifier: CC0-1.0

set -e

wget -O firmware/CFI-ZEY1.bin https://raw.githubusercontent.com/Hackinside/PS5_camera_files/45e256877219f9e9b07723d29f8be5ce949bf3f2/firmware.bin
wget -O firmware/CUH-ZEY2.bin https://raw.githubusercontent.com/Hackinside/PS4-CAMERA-DRIVERS/81084d76b2d8f49d17c7b9fee75a107fcbc02898/firmware.bin

cat <<EOF | sha256sum -c
10af1aee76fe0057a88db7ebf5f3ebf32430633effb93722be4cd0a9ed4fce54  firmware/CFI-ZEY1.bin
09513d1068b9b5e23b09e03bca5640ed4d01830b034b9802395006b14a927b01  firmware/CUH-ZEY2.bin
EOF