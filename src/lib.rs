//! Safe Rust wrapper for Linux PTP (Precision Time Protocol) Hardware Clock driver.
//!
//! This crate provides a safe interface to the Linux PTP kernel driver,
//! exposing the following ioctls:
//! - `ptp_clock_caps` - Get clock capabilities
//! - `ptp_sys_offset` - Get system offset measurements
//! - `ptp_sys_offset_precise` - Get precise system offset measurements
//! - `ptp_sys_offset_extended` - Get extended system offset measurements

use std::{
    fs::File,
    io::{Error, Result},
    mem::MaybeUninit,
    os::{
        fd::AsRawFd,
        raw::c_ulong,
    },
    path::PathBuf,
};

pub mod ptp;
use ptp::*;

// PTP ioctl constants - These are standard Linux PTP driver ioctls
// Based on linux/ptp_clock.h: PTP_CLK_MAGIC = '=' = 0x3D

// Correct ioctl values partially calculated from the header file and partially
// worked out by stracing chrony. If anyone can explain where the top 16 bits
// come from I'd be grateful.

// _IOR(PTP_CLK_MAGIC, 1, struct ptp_clock_caps) -> 0x80503d01
// _IOW(PTP_CLK_MAGIC, 5, struct ptp_sys_offset) -> 0x43403d05
// _IOWR(PTP_CLK_MAGIC, 8, struct ptp_sys_offset_precise) -> 0xc0403d08
// _IOWR(PTP_CLK_MAGIC, 9, struct ptp_sys_offset_extended) -> 0xc4c03d09

const PTP_CLOCK_GETCAPS: c_ulong = 0x80503d01; // _IOR(PTP_CLK_MAGIC, 1, struct ptp_clock_caps)
const PTP_SYS_OFFSET: c_ulong = 0x43403d05;   // _IOW(PTP_CLK_MAGIC, 5, struct ptp_sys_offset)
const PTP_SYS_OFFSET_PRECISE: c_ulong = 0xc0403d08; // _IOWR(PTP_CLK_MAGIC, 8, struct ptp_sys_offset_precise)
const PTP_SYS_OFFSET_EXTENDED: c_ulong = 0xc4c03d09; // _IOWR(PTP_CLK_MAGIC, 9, struct ptp_sys_offset_extended)

/// A safe wrapper for PTP hardware clock devices
pub struct PtpDevice(File);

impl PtpDevice {
    /// Create a new PTP device from a path
    pub fn new(path: PathBuf) -> Result<PtpDevice> {
        Ok(PtpDevice(File::open(path)?))
    }

    /// Perform ioctl request and check result for possible errors
    unsafe fn ioctl<T>(&self, request: c_ulong, value: &mut T) -> Result<()> {
        match libc::ioctl(self.0.as_raw_fd(), request as _, value) {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }

    /// Perform ioctl request with uninitialized memory
    unsafe fn ioctl_uninit<T>(&self, request: c_ulong) -> Result<T> {
        let mut value: MaybeUninit<T> = MaybeUninit::uninit();
        self.ioctl(request, &mut value)?;
        Ok(unsafe { value.assume_init() })
    }

    /// Get the clock capabilities
    pub fn get_caps(&self) -> Result<ptp_clock_caps> {
        // Safety: PTP_CLOCK_GETCAPS writes ptp_clock_caps, for which memory is allocated and returned by ioctl_uninit
        unsafe { self.ioctl_uninit(PTP_CLOCK_GETCAPS) }
    }

    /// Get system offset measurements
    pub fn get_sys_offset(&self) -> Result<ptp_sys_offset> {
        let mut offset = ptp_sys_offset::default();
        // Safety: PTP_SYS_OFFSET expects and writes to a ptp_sys_offset, which lives for the duration of the call
        unsafe { self.ioctl(PTP_SYS_OFFSET, &mut offset)? };
        Ok(offset)
    }

    /// Get precise system offset measurements
    pub fn get_sys_offset_precise(&self) -> Result<ptp_sys_offset_precise> {
        let mut offset = ptp_sys_offset_precise::default();
        // Safety: PTP_SYS_OFFSET_PRECISE expects and writes to a ptp_sys_offset_precise, which lives for the duration of the call
        unsafe { self.ioctl(PTP_SYS_OFFSET_PRECISE, &mut offset)? };
        Ok(offset)
    }

    /// Get extended system offset measurements
    pub fn get_sys_offset_extended(&self) -> Result<ptp_sys_offset_extended> {
        let mut offset = ptp_sys_offset_extended::default();
        // Safety: PTP_SYS_OFFSET_EXTENDED expects and writes to a ptp_sys_offset_extended, which lives for the duration of the call
        unsafe { self.ioctl(PTP_SYS_OFFSET_EXTENDED, &mut offset)? };
        Ok(offset)
    }
}
