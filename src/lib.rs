use anyhow::Result;
use hidapi::HidApi;
use log::{debug, info};
use std::time::Duration;

pub mod errors;
pub mod hid_protocol;
pub mod keyboard;
pub mod device_scanner;

pub use errors::*;
pub use hid_protocol::*;
pub use keyboard::*;
pub use device_scanner::*;

/// Configuration for the Durgod K320 keyboard
#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    /// USB Vendor ID (will be determined from analysis)
    pub vendor_id: u16,
    /// USB Product ID (will be determined from analysis)
    pub product_id: u16,
    /// Timeout for HID operations
    pub timeout: Duration,
    /// Interface number (usually 0 for keyboards)
    pub interface: i32,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            vendor_id: 0x0000,  // To be filled from device analysis
            product_id: 0x0000, // To be filled from device analysis
            timeout: Duration::from_millis(1000),
            interface: 0,
        }
    }
}

/// Main controller for the Durgod K320 keyboard
pub struct DurgodController {
    config: KeyboardConfig,
    device: Option<hidapi::HidDevice>,
    api: HidApi,
}

impl DurgodController {
    /// Create a new controller instance
    pub fn new(config: KeyboardConfig) -> Result<Self> {
        let api = HidApi::new().map_err(|e| DurgodError::HidInitError(e.to_string()))?;
        
        Ok(Self {
            config,
            device: None,
            api,
        })
    }

    /// Connect to the keyboard device
    pub fn connect(&mut self) -> Result<()> {
        info!("Connecting to Durgod K320 keyboard ({}:{})", 
              self.config.vendor_id, self.config.product_id);

        let device = self.api
            .open(self.config.vendor_id, self.config.product_id)
            .map_err(|e| DurgodError::DeviceNotFound(format!(
                "Could not open device {}:{} - {}", 
                self.config.vendor_id, self.config.product_id, e
            )))?;

        // Get device info
        let manufacturer = device.get_manufacturer_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());
        
        let product = device.get_product_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        info!("Connected to device: {} {}", manufacturer, product);
        
        self.device = Some(device);
        Ok(())
    }

    /// Disconnect from the keyboard
    pub fn disconnect(&mut self) {
        if self.device.is_some() {
            info!("Disconnecting from keyboard");
            self.device = None;
        }
    }

    /// Check if connected to the keyboard
    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    /// Send a raw HID report to the keyboard
    pub fn send_raw_report(&self, report: &[u8]) -> Result<()> {
        let device = self.device.as_ref()
            .ok_or_else(|| DurgodError::NotConnected)?;

        debug!("Sending HID report: {}", hex::encode(report));
        
        device.write(report)
            .map_err(|e| DurgodError::CommunicationError(e.to_string()))?;
        
        Ok(())
    }

    /// Send a key press event
    pub fn send_key_press(&self, key: KeyCode) -> Result<()> {
        let report = HidKeyboardReport::key_press(key);
        self.send_hid_report(&report)?;
        
        // Send key release after a short delay
        std::thread::sleep(Duration::from_millis(50));
        let release_report = HidKeyboardReport::key_release();
        self.send_hid_report(&release_report)?;
        
        Ok(())
    }

    /// Send a key press with modifiers
    pub fn send_key_with_modifiers(&self, key: KeyCode, modifiers: Modifiers) -> Result<()> {
        let report = HidKeyboardReport::key_press_with_modifiers(key, modifiers);
        self.send_hid_report(&report)?;
        
        // Send key release
        std::thread::sleep(Duration::from_millis(50));
        let release_report = HidKeyboardReport::key_release();
        self.send_hid_report(&release_report)?;
        
        Ok(())
    }

    /// Send a HID keyboard report
    pub fn send_hid_report(&self, report: &HidKeyboardReport) -> Result<()> {
        let raw_report = report.to_bytes();
        self.send_raw_report(&raw_report)
    }

    /// Read a report from the keyboard (for capturing input)
    pub fn read_report(&self, buf: &mut [u8]) -> Result<usize> {
        let device = self.device.as_ref()
            .ok_or_else(|| DurgodError::NotConnected)?;

        let bytes_read = device.read_timeout(buf, self.config.timeout.as_millis() as i32)
            .map_err(|e| DurgodError::CommunicationError(e.to_string()))?;
        
        if bytes_read > 0 {
            debug!("Received HID report: {}", hex::encode(&buf[..bytes_read]));
        }
        
        Ok(bytes_read)
    }

    /// Get device information
    pub fn get_device_info(&self) -> Result<DeviceInfo> {
        let device = self.device.as_ref()
            .ok_or_else(|| DurgodError::NotConnected)?;

        let manufacturer = device.get_manufacturer_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());
        
        let product = device.get_product_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        let serial = device.get_serial_number_string()
            .unwrap_or_else(|_| Some("Unknown".to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        Ok(DeviceInfo {
            vendor_id: self.config.vendor_id,
            product_id: self.config.product_id,
            manufacturer,
            product,
            serial_number: serial,
        })
    }
}

impl Drop for DurgodController {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// Device information structure
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: String,
}

/// Initialize logging for the library
pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
} 