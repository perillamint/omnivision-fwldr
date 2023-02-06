// SPDX-FileCopyrightText: 2023 perillamint
//
// SPDX-License-Identifier: MIT

use std::fs;
use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use log::info;
use rusb::{self, Device, GlobalContext};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "05a9")]
    vid: String,
    #[arg(long, default_value = "0580")]
    pid: String,
    #[arg(long, default_value = "0")]
    iface: u8,
    #[arg(long)]
    firmware: String,
}

fn get_camera_device(vid: u16, pid: u16) -> Option<Device<GlobalContext>> {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if (device_desc.vendor_id() == vid) && (device_desc.product_id() == pid) {
            return Some(device);
        }
    }

    None
}

fn main() -> ExitCode {
    let args = Args::parse();

    let vid = u16::from_str_radix(&args.vid, 16).unwrap();
    let pid = u16::from_str_radix(&args.pid, 16).unwrap();
    let firmware = fs::read(args.firmware).unwrap();

    match get_camera_device(vid, pid) {
        Some(camera_device) => {
            let mut camera = match camera_device.open() {
                Ok(device) => device,
                Err(e) => {
                    panic!("Failed to open device: {e:?}");
                }
            };

            if camera.kernel_driver_active(args.iface).unwrap() {
                camera.detach_kernel_driver(args.iface).unwrap();
            }

            let mut off = 0x00;
            // Write chunk by chunk.
            for chunk in firmware.chunks(512) {
                let idx = (off / 0x10000) as u16 + 0x14;
                let value = (off & 0xFFFF) as u16;
                let len = chunk.len();
                info!("Writing chunk... {off} - {len}");
                info!("Ctrl value = {value}, idx = {idx}");
                let sz = camera
                    .write_control(0x40, 0x00, value, idx, chunk, Duration::from_secs(1))
                    .unwrap();

                off += sz;
                assert_eq!(len, sz);
            }

            // Boot up the camera
            match camera.write_control(0x40, 0x00, 0x2200, 0x8018, &[0x5b], Duration::from_secs(1)) {
                Ok(_) => {Ok(())}
                Err(rusb::Error::NoDevice) => {
                    info!("Device gone (boot successful)");
                    Ok(())
                }
                Err(e) => Err(e)
            }.unwrap();
        }
        None => {
            println!("Device does not exist!");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
