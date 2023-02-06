#!/bin/bash

# SPDX-FileCopyrightText: 2023 perillamint
#
# SPDX-License-Identifier: CC0-1.0

set -e

mkdir -p /usr/lib/firmware/sony
cp -v firmware/* /usr/lib/firmware/sony/
cp -v ./target/release/omnivision-fwldr /usr/local/bin/
cp -vi ./assets/udev/* /etc/udev/rules.d/
udevadm control -R
