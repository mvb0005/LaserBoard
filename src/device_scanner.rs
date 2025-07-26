use hidapi::{HidApi, DeviceInfo as HidDeviceInfo};
use log::{debug, info, warn};
use crate::{DeviceInfo, DurgodError};
use anyhow::Result;

/// Scanner for finding HID devices, particularly keyboards
pub struct DeviceScanner {
    api: HidApi,
}

impl DeviceScanner {
    /// Create a new device scanner
    pub fn new() -> Result<Self> {
        let api = HidApi::new().map_err(|e| DurgodError::HidInitError(e.to_string()))?;
        Ok(Self { api })
    }

    /// Scan for all HID devices
    pub fn scan_all_devices(&self) -> Result<Vec<ScannedDevice>> {
        info!("Scanning for all HID devices...");
        
        let devices = self.api.device_list()
            .map(|device_info| ScannedDevice::from_hid_device_info(device_info))
            .collect();
        
        debug!("Found {} HID devices", devices.len());
        Ok(devices)
    }

    /// Scan specifically for keyboard devices
    pub fn scan_keyboards(&self) -> Result<Vec<ScannedDevice>> {
        info!("Scanning for keyboard devices...");
        
        let devices: Vec<ScannedDevice> = self.api.device_list()
            .filter(|device| self.is_likely_keyboard(device))
            .map(|device_info| ScannedDevice::from_hid_device_info(device_info))
            .collect();
        
        info!("Found {} potential keyboard devices", devices.len());
        Ok(devices)
    }

    /// Find devices by vendor and product ID
    pub fn find_devices(&self, vendor_id: u16, product_id: u16) -> Result<Vec<ScannedDevice>> {
        info!("Searching for devices with VID:PID {}:{:04x}", vendor_id, product_id);
        
        let devices: Vec<ScannedDevice> = self.api.device_list()
            .filter(|device| device.vendor_id() == vendor_id && device.product_id() == product_id)
            .map(|device_info| ScannedDevice::from_hid_device_info(device_info))
            .collect();
        
        info!("Found {} matching devices", devices.len());
        Ok(devices)
    }

    /// Find devices by vendor ID only
    pub fn find_devices_by_vendor(&self, vendor_id: u16) -> Result<Vec<ScannedDevice>> {
        info!("Searching for devices from vendor {:04x}", vendor_id);
        
        let devices: Vec<ScannedDevice> = self.api.device_list()
            .filter(|device| device.vendor_id() == vendor_id)
            .map(|device_info| ScannedDevice::from_hid_device_info(device_info))
            .collect();
        
        info!("Found {} devices from vendor {:04x}", devices.len(), vendor_id);
        Ok(devices)
    }

    /// Search for devices by manufacturer or product name
    pub fn find_devices_by_name(&self, search_term: &str) -> Result<Vec<ScannedDevice>> {
        info!("Searching for devices with name containing: '{}'", search_term);
        let search_lower = search_term.to_lowercase();
        
        let devices: Vec<ScannedDevice> = self.api.device_list()
            .filter(|device| {
                let manufacturer = device.manufacturer_string()
                    .unwrap_or_else(|| Some("".to_string()))
                    .unwrap_or_else(|| "".to_string())
                    .to_lowercase();
                
                let product = device.product_string()
                    .unwrap_or_else(|| Some("".to_string()))
                    .unwrap_or_else(|| "".to_string())
                    .to_lowercase();
                
                manufacturer.contains(&search_lower) || product.contains(&search_lower)
            })
            .map(|device_info| ScannedDevice::from_hid_device_info(device_info))
            .collect();
        
        info!("Found {} devices matching '{}'", devices.len(), search_term);
        Ok(devices)
    }

    /// Find potential Durgod keyboards
    pub fn find_durgod_keyboards(&self) -> Result<Vec<ScannedDevice>> {
        info!("Searching for Durgod keyboards...");
        
        // Search by name first
        let mut devices = self.find_devices_by_name("durgod")?;
        
        // Also check for common Durgod vendor IDs if known
        // Note: These would need to be determined from the packet analysis
        let potential_vendor_ids = vec![
            // Add known Durgod vendor IDs here after analysis
            // 0x0c45, // Example - Microdia (sometimes used)
            // 0x04d9, // Example - Holtek (common keyboard controller)
        ];
        
        for vendor_id in potential_vendor_ids {
            let vendor_devices = self.find_devices_by_vendor(vendor_id)?;
            // Filter for keyboard-like devices
            let keyboard_devices: Vec<ScannedDevice> = vendor_devices.into_iter()
                .filter(|device| device.is_likely_keyboard())
                .collect();
            devices.extend(keyboard_devices);
        }
        
        // Remove duplicates
        devices.sort_by_key(|d| (d.vendor_id, d.product_id, d.interface_number));
        devices.dedup_by_key(|d| (d.vendor_id, d.product_id, d.interface_number));
        
        info!("Found {} potential Durgod keyboard devices", devices.len());
        Ok(devices)
    }

    /// Check if a device is likely a keyboard based on usage page and interface class
    fn is_likely_keyboard(&self, device: &HidDeviceInfo) -> bool {
        // Check usage page (0x01 = Generic Desktop) and usage (0x06 = Keyboard)
        if device.usage_page() == 0x01 && device.usage() == 0x06 {
            return true;
        }
        
        // Also check for HID class with keyboard protocol
        // This would require additional inspection of the device
        
        // For now, we'll be more permissive and include devices that might be keyboards
        let manufacturer = device.manufacturer_string()
            .unwrap_or_else(|| Some("".to_string()))
            .unwrap_or_else(|| "".to_string())
            .to_lowercase();
        
        let product = device.product_string()
            .unwrap_or_else(|| Some("".to_string()))
            .unwrap_or_else(|| "".to_string())
            .to_lowercase();
        
        // Check for keyboard-related terms
        let keyboard_terms = ["keyboard", "kbd", "durgod", "mechanical"];
        keyboard_terms.iter().any(|term| 
            manufacturer.contains(term) || product.contains(term)
        )
    }

    /// Get detailed information about a specific device
    pub fn get_device_details(&self, vendor_id: u16, product_id: u16) -> Result<Option<DetailedDeviceInfo>> {
        let device = match self.api.open(vendor_id, product_id) {
            Ok(device) => device,
            Err(_) => return Ok(None),
        };
        
        let manufacturer = device.get_manufacturer_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());
        
        let product = device.get_product_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());
        
        let serial = device.get_serial_number_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(Some(DetailedDeviceInfo {
            vendor_id,
            product_id,
            manufacturer,
            product,
            serial_number: serial,
            // Note: Additional fields like report descriptor would require more complex parsing
            // This would be added based on the results of packet analysis
        }))
    }
}

/// Information about a scanned HID device
#[derive(Debug, Clone)]
pub struct ScannedDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
    pub interface_number: i32,
    pub usage_page: u16,
    pub usage: u16,
    pub path: String,
}

impl ScannedDevice {
    /// Create from hidapi DeviceInfo
    fn from_hid_device_info(info: &HidDeviceInfo) -> Self {
        Self {
            vendor_id: info.vendor_id(),
            product_id: info.product_id(),
            manufacturer: info.manufacturer_string().unwrap_or(None),
            product: info.product_string().unwrap_or(None),
            serial_number: info.serial_number().unwrap_or(None),
            interface_number: info.interface_number(),
            usage_page: info.usage_page(),
            usage: info.usage(),
            path: info.path().to_string_lossy().to_string(),
        }
    }

    /// Check if this device is likely a keyboard
    pub fn is_likely_keyboard(&self) -> bool {
        // USB HID keyboard usage page (0x01) and usage (0x06)
        if self.usage_page == 0x01 && self.usage == 0x06 {
            return true;
        }
        
        // Check product/manufacturer strings
        let manufacturer = self.manufacturer.as_deref().unwrap_or("").to_lowercase();
        let product = self.product.as_deref().unwrap_or("").to_lowercase();
        
        let keyboard_terms = ["keyboard", "kbd", "durgod", "mechanical"];
        keyboard_terms.iter().any(|term| 
            manufacturer.contains(term) || product.contains(term)
        )
    }

    /// Get a human-readable description of the device
    pub fn description(&self) -> String {
        let manufacturer = self.manufacturer.as_deref().unwrap_or("Unknown");
        let product = self.product.as_deref().unwrap_or("Unknown");
        format!("{} {} ({:04x}:{:04x})", manufacturer, product, self.vendor_id, self.product_id)
    }

    /// Convert to a basic DeviceInfo
    pub fn to_device_info(&self) -> DeviceInfo {
        DeviceInfo {
            vendor_id: self.vendor_id,
            product_id: self.product_id,
            manufacturer: self.manufacturer.as_deref().unwrap_or("Unknown").to_string(),
            product: self.product.as_deref().unwrap_or("Unknown").to_string(),
            serial_number: self.serial_number.as_deref().unwrap_or("Unknown").to_string(),
        }
    }
}

/// Detailed device information obtained by opening the device
#[derive(Debug, Clone)]
pub struct DetailedDeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: String,
    // Additional fields could be added here:
    // pub report_descriptor: Vec<u8>,
    // pub input_report_length: usize,
    // pub output_report_length: usize,
    // pub feature_report_length: usize,
}

/// Known keyboard vendor IDs for quick identification
pub struct KnownVendors;

impl KnownVendors {
    pub const LOGITECH: u16 = 0x046d;
    pub const MICROSOFT: u16 = 0x045e;
    pub const CORSAIR: u16 = 0x1b1c;
    pub const RAZER: u16 = 0x1532;
    pub const STEELSERIES: u16 = 0x1038;
    pub const CHERRY: u16 = 0x046a;
    pub const COOLER_MASTER: u16 = 0x2516;
    pub const HOLTEK: u16 = 0x04d9;  // Common keyboard controller manufacturer
    pub const MICRODIA: u16 = 0x0c45; // Another common manufacturer
    
    /// Get vendor name from ID if known
    pub fn vendor_name(vendor_id: u16) -> Option<&'static str> {
        match vendor_id {
            Self::LOGITECH => Some("Logitech"),
            Self::MICROSOFT => Some("Microsoft"),
            Self::CORSAIR => Some("Corsair"),
            Self::RAZER => Some("Razer"),
            Self::STEELSERIES => Some("SteelSeries"),
            Self::CHERRY => Some("Cherry"),
            Self::COOLER_MASTER => Some("Cooler Master"),
            Self::HOLTEK => Some("Holtek"),
            Self::MICRODIA => Some("Microdia"),
            _ => None,
        }
    }
}

impl Default for DeviceScanner {
    fn default() -> Self {
        Self::new().expect("Failed to initialize HID API")
    }
} 