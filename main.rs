extern crate nix;
use nix::ioctl_read;
use std::fs::File;
use std::os::unix::io::{IntoRawFd, RawFd};

const IOCTL_MAGIC: u8 = 0xEC;
const MAGIC_READ_CL: u8 = IOCTL_MAGIC+ 1;
const FAN1_ADDR: u8 = 0x10;
const FAN2_ADDR: u8 = 0x11;
const FAN3_ADDR: u8 = 0x12;

ioctl_read!(io_fan1_raw, MAGIC_READ_CL, FAN1_ADDR, u64);
ioctl_read!(io_fan2_raw, MAGIC_READ_CL, FAN2_ADDR, u64);
ioctl_read!(io_fan3_raw, MAGIC_READ_CL, FAN3_ADDR, u64);

fn get_fan_speed_percent(fan_raw_speed: u64) -> f64 {
    let raw_speed = (fan_raw_speed & 0xFF) as f64;
    return ((raw_speed / 0xFF as f64) * 100 as f64) as f64
}

fn main() {
    let file = File::open("/dev/tuxedo_io");
    let raw_fd: RawFd = file.unwrap().into_raw_fd();

    let mut fan_raw_speed = 0;
    let ret = unsafe { io_fan1_raw(raw_fd, &mut fan_raw_speed) };
    println!("returned {:#?}, fanRawSpeed = {}", ret, get_fan_speed_percent(fan_raw_speed));
    let ret = unsafe { io_fan2_raw(raw_fd, &mut fan_raw_speed) };
    println!("returned {:#?}, fanRawSpeed = {}", ret, get_fan_speed_percent(fan_raw_speed));
    let ret = unsafe { io_fan3_raw(raw_fd, &mut fan_raw_speed) };
    println!("returned {:#?}, fanRawSpeed = {}", ret, get_fan_speed_percent(fan_raw_speed));
}

