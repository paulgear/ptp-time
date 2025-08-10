//! A simple PTP demo program
//!
//! Build with `cargo build --package ptp-time --example demo`
//! Run with: `sudo ./target/debug/examples/demo /dev/ptp0`

use ptp_time::PtpDevice;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Example usage:");
        println!("$ sudo ./target/debug/examples/demo /dev/ptp0");
        return;
    }

    let path = PathBuf::from(&args[1]); // path to PTP device

    println!("Opening PTP device {}", path.display());
    let device = match PtpDevice::new(path) {
        Ok(device) => device,
        Err(e) => {
            eprintln!("Could not open PTP device: {}", e);
            return;
        }
    };

    // Get clock capabilities
    println!("\n=== Getting Clock Capabilities ===");
    match device.get_caps() {
        Ok(caps) => {
            println!("Clock capabilities:");
            println!("  max_adj: {}", caps.max_adj);
            println!("  n_alarm: {}", caps.n_alarm);
            println!("  n_ext_ts: {}", caps.n_ext_ts);
            println!("  n_per_out: {}", caps.n_per_out);
            println!("  pps: {}", caps.pps);
            println!("  n_pins: {}", caps.n_pins);
            println!("  cross_timestamping: {}", caps.cross_timestamping);
            println!("  adjust_phase: {}", caps.adjust_phase);
            println!("  max_phase_adj: {}", caps.max_phase_adj);
        }
        Err(e) => {
            eprintln!("Could not get capabilities: {}", e);
        }
    }

    // Get system offset
    println!("\n=== Getting System Offset ===");
    match device.get_sys_offset() {
        Ok(offset) => {
            println!("System offset ({} samples):", offset.n_samples);
            println!("  First timestamp pair: {}.{:09} -> {}.{:09}",
                offset.ts[0].sec, offset.ts[0].nsec,
                offset.ts[1].sec, offset.ts[1].nsec);
        }
        Err(e) => {
            eprintln!("Could not get system offset: {}", e);
        }
    }

    // Get precise system offset
    println!("\n=== Getting Precise System Offset ===");
    match device.get_sys_offset_precise() {
        Ok(offset) => {
            println!("Precise system offset:");
            println!("  Device time: {}.{:09}", offset.device.sec, offset.device.nsec);
            println!("  System realtime: {}.{:09}", offset.sys_realtime.sec, offset.sys_realtime.nsec);
            println!("  System monotonic raw: {}.{:09}", offset.sys_monoraw.sec, offset.sys_monoraw.nsec);
        }
        Err(e) => {
            eprintln!("Could not get precise system offset: {}", e);
        }
    }

    // Get extended system offset
    println!("\n=== Getting Extended System Offset ===");
    match device.get_sys_offset_extended() {
        Ok(offset) => {
            println!("Extended system offset ({} samples):", offset.n_samples);
            println!("  First timestamp triplet: {}.{:09} -> {}.{:09} -> {}.{:09}",
                offset.ts[0][0].sec, offset.ts[0][0].nsec,
                offset.ts[0][1].sec, offset.ts[0][1].nsec,
                offset.ts[0][2].sec, offset.ts[0][2].nsec);
        }
        Err(e) => {
            eprintln!("Could not get extended system offset: {}", e);
        }
    }

    println!("\nDemo completed successfully!");
}
