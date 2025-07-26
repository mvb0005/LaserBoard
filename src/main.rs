use clap::{Arg, Command};
use durgod_k320_controller::{init_logging, DeviceScanner};
use log::info;

fn main() -> anyhow::Result<()> {
    init_logging();
    
    let matches = Command::new("durgod-controller")
        .version("0.1.0")
        .author("Your Name")
        .about("Durgod K320 Keyboard Controller")
        .subcommand(
            Command::new("scan")
                .about("Scan for HID devices")
                .arg(
                    Arg::new("keyboards-only")
                        .long("keyboards-only")
                        .help("Only show keyboard devices")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("info")
                .about("Show information about this tool")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("scan", sub_matches)) => {
            info!("Scanning for HID devices...");
            let scanner = DeviceScanner::new()?;
            
            let devices = if sub_matches.get_flag("keyboards-only") {
                scanner.scan_keyboards()?
            } else {
                scanner.scan_all_devices()?
            };
            
            if devices.is_empty() {
                println!("No devices found.");
            } else {
                println!("Found {} devices:", devices.len());
                for device in devices {
                    println!("  - {}", device.description());
                    println!("    VID:PID = {:04x}:{:04x}", device.vendor_id, device.product_id);
                    println!("    Interface: {}, Usage: {:02x}:{:02x}", 
                           device.interface_number, device.usage_page, device.usage);
                    if device.is_likely_keyboard() {
                        println!("    🎹 Detected as keyboard");
                    }
                    println!();
                }
            }
        }
        Some(("info", _)) => {
            println!("Durgod K320 Keyboard Controller v0.1.0");
            println!("========================================");
            println!();
            println!("This tool helps reverse engineer and control the Durgod K320 keyboard.");
            println!();
            println!("Current status: TESTING PHASE");
            println!("- Basic device scanning implemented");
            println!("- Protocol analysis tools ready");
            println!("- Packet capture scripts available");
            println!();
            println!("Next steps:");
            println!("1. Run device scan to find your keyboard");
            println!("2. Use capture scripts to analyze USB traffic");
            println!("3. Build protocol implementation based on findings");
            println!();
            println!("Available commands:");
            println!("  scan                 - Scan for HID devices");
            println!("  scan --keyboards-only - Scan for keyboard devices only");
        }
        _ => {
            println!("Durgod K320 Controller - Use --help for usage information");
            println!("Quick start: cargo run -- scan --keyboards-only");
        }
    }

    Ok(())
} 