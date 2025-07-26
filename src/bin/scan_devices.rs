use durgod_k320_controller::{DeviceScanner, init_logging};
use anyhow::Result;

fn main() -> Result<()> {
    init_logging();
    
    println!("HID Device Scanner");
    println!("==================");
    println!();
    
    let scanner = DeviceScanner::new()?;
    
    println!("🔍 Scanning for all HID devices...");
    let all_devices = scanner.scan_all_devices()?;
    println!("Found {} total HID devices", all_devices.len());
    
    println!();
    println!("🎹 Scanning for keyboard devices...");
    let keyboards = scanner.scan_keyboards()?;
    println!("Found {} potential keyboard devices", keyboards.len());
    
    if keyboards.is_empty() {
        println!();
        println!("❌ No keyboard devices detected!");
        println!("This could mean:");
        println!("  - No keyboards are connected");
        println!("  - Permission issues (try running with sudo)");
        println!("  - The keyboard doesn't identify as a standard HID keyboard");
        
        println!();
        println!("📋 All HID devices found:");
        for (i, device) in all_devices.iter().enumerate() {
            println!("  {}. {} ({:04x}:{:04x})", 
                   i + 1, device.description(), device.vendor_id, device.product_id);
            println!("      Usage: {:02x}:{:02x}, Interface: {}", 
                   device.usage_page, device.usage, device.interface_number);
        }
    } else {
        println!();
        println!("🎹 Detected keyboard devices:");
        for (i, device) in keyboards.iter().enumerate() {
            println!("  {}. {} ({:04x}:{:04x})", 
                   i + 1, device.description(), device.vendor_id, device.product_id);
            println!("      Usage: {:02x}:{:02x}, Interface: {}", 
                   device.usage_page, device.usage, device.interface_number);
            println!("      Path: {}", device.path);
        }
        
        println!();
        println!("🔍 Looking specifically for Durgod devices...");
        let durgod_devices = scanner.find_durgod_keyboards()?;
        if durgod_devices.is_empty() {
            println!("❌ No Durgod devices found by name");
            println!("💡 Your Durgod keyboard might not identify itself as 'Durgod'");
            println!("   Check the devices above - your K320 is likely one of them!");
        } else {
            println!("✅ Found {} potential Durgod devices:", durgod_devices.len());
            for device in durgod_devices {
                println!("  - {} ({:04x}:{:04x})", 
                       device.description(), device.vendor_id, device.product_id);
            }
        }
    }
    
    println!();
    println!("💡 Next steps:");
    println!("1. If you see your keyboard above, note its VID:PID");
    println!("2. Run: ./scripts/capture_traffic.sh -v <VID> -p <PID>");
    println!("3. Press some keys while capturing to generate traffic");
    println!("4. Analyze the captured packets");
    
    Ok(())
} 