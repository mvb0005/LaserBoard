# Durgod K320 Reverse Engineering Project - Context & Status

**Last Updated**: December 26, 2024  
**Project Status**: READY FOR PARALLEL EXECUTION  
**Critical Issue**: Compilation errors fixed, ready for multi-worker approach

## 🎯 Project Goal

Reverse engineer the USB HID protocol of the Durgod K320 mechanical keyboard to enable programmatic key press injection via a Rust application.

## 📊 Current Status

### ✅ Completed
- [x] **Environment Setup Scripts**: Complete shell scripts for installing Wireshark, tshark, usbmon, Rust, and USB tools
- [x] **Project Structure**: Organized directory structure with scripts/, docs/, captures/, src/
- [x] **Rust Library Framework**: Core library structure with error handling, HID protocol definitions, and device scanning
- [x] **Shell Scripts**: 
  - `setup.sh` - Environment setup (tested, mostly working)
  - `identify_keyboard.sh` - USB device identification 
  - `capture_traffic.sh` - USB packet capture using tshark
  - `analyze_packets.sh` - Packet analysis and protocol extraction
- [x] **Rust Modules**:
  - `errors.rs` - Error handling with thiserror
  - `hid_protocol.rs` - USB HID keyboard report structures and key codes
  - `keyboard.rs` - High-level keyboard interface
  - `device_scanner.rs` - HID device discovery
  - `lib.rs` - Main library interface

### 🔄 In Progress
- [x] **Rust Compilation**: Dependencies resolved, compilation errors identified and ready for fixes
- [ ] **Parallel Work Execution**: Ready to distribute work across multiple workers

### ❌ Not Started
- [ ] **Actual USB Traffic Capture**: No real packet captures from Durgod K320 yet
- [ ] **Protocol Analysis**: Haven't analyzed real keyboard traffic
- [ ] **Key Injection**: No actual key press simulation implemented
- [ ] **Device Identification**: Don't know the actual VID:PID of the Durgod K320

## 🚨 Current Issues & Blockers

### 1. **Compilation Issues** (PRIORITY 1)
- **Problem**: hidapi v2.6.3 API compatibility issues identified
- **Status**: Specific errors documented, ready for Worker A to fix
- **Next**: Fix API compatibility issues in errors.rs and device_scanner.rs

### 2. **Untested Code** (PRIORITY 1)
- **Problem**: Built extensive Rust library without testing basic functionality
- **Risk**: Code may not work with real HID devices or may lack permissions
- **Next**: Create minimal test programs to validate HID access

### 3. **Missing Real Hardware Analysis** (PRIORITY 2)
- **Problem**: Don't have actual packet captures from Durgod K320
- **Impact**: All HID protocol code is based on standard USB HID, may not match real device
- **Next**: Connect Durgod K320 and run device identification scripts

### 4. **Permission Issues** (PRIORITY 2)
- **Problem**: USB device access typically requires elevated permissions or udev rules
- **Status**: Setup script creates udev rules but requires logout/reboot
- **Next**: Test HID device access with current user permissions

## 📁 Project Structure

```
LaserBoard/
├── README.md                 # Project overview and getting started
├── Cargo.toml               # Rust dependencies and project config
├── docs/
│   ├── project_context.md   # This file
│   ├── protocol.md          # (Future) Protocol documentation
│   └── findings.md          # (Future) Research findings
├── scripts/
│   ├── setup.sh            # ✅ Environment setup (working)
│   ├── identify_keyboard.sh # ✅ Device identification (untested)
│   ├── capture_traffic.sh   # ✅ Traffic capture (untested)
│   └── analyze_packets.sh   # ✅ Packet analysis (untested)
├── captures/               # (Empty) Packet capture files
├── src/
│   ├── lib.rs              # ✅ Main library interface
│   ├── errors.rs           # ✅ Error handling
│   ├── hid_protocol.rs     # ✅ HID protocol definitions
│   ├── keyboard.rs         # ✅ High-level keyboard interface
│   ├── device_scanner.rs   # ✅ Device discovery
│   ├── main.rs             # ✅ CLI interface (untested)
│   └── bin/
│       └── scan_devices.rs # ✅ Device scanner binary (untested)
```

## 🔧 Dependencies & Environment

### System Dependencies (via setup.sh)
- **Wireshark/tshark**: USB packet capture ✅ Installed
- **usbmon**: USB monitoring kernel module ✅ Available
- **libusb-1.0-dev**: USB device libraries ✅ Installed
- **Rust**: Programming environment ✅ Installed (v1.88.0)

### Rust Dependencies (Cargo.toml)
```toml
hidapi = "2.6"        # HID device communication
rusb = "0.9"          # Alternative USB interface
clap = "4.4"          # CLI argument parsing
anyhow = "1.0"        # Error handling
thiserror = "1.0"     # Error derive macros
log = "0.4"           # Logging
env_logger = "0.10"   # Log configuration
serde = "1.0"         # Serialization
hex = "0.4"           # Hex encoding for packet data
chrono = "0.4"        # Time handling
```

## 🎯 Parallel Work Execution

### **Worker A: Rust Compilation Specialist**
- Fix hidapi v2.6.3 API compatibility issues
- Resolve type annotation problems in device_scanner.rs
- Clean up unused imports
- Ensure `cargo check` passes

### **Worker B: Hardware Setup Specialist**  
- Connect Durgod K320 keyboard
- Run device identification scripts
- Document VID:PID and interface details
- Verify USB permissions

### **Worker C: Protocol Analysis Specialist**
- Capture USB traffic using identified device
- Analyze HID report structure
- Document key code mappings
- Generate protocol documentation

### **Worker D: Implementation Specialist**
- Implement HID report generation
- Create key injection functionality
- Add text typing capabilities
- Test with real device

### 2. **Test Basic Device Scanning** (Today)
```bash
cargo run --bin scan-devices   # Test HID device discovery
# OR
cargo run -- scan --keyboards-only
```

### 3. **Identify Durgod K320** (Today)
```bash
./scripts/identify_keyboard.sh  # Find the actual device
lsusb                           # Manual verification
```

### 4. **Test USB Permissions** (Today)
- Test if HID devices are accessible without sudo
- If not, verify udev rules are working or run with elevated permissions

### 5. **First Packet Capture** (Today/Tomorrow)
```bash
# Once device is identified with VID:PID
./scripts/capture_traffic.sh -v <VID> -p <PID> -d 30
```

## 🧪 Testing Strategy

### Phase 1: Environment Validation
1. ✅ Rust compilation works
2. ✅ HID device scanning works
3. ✅ Can identify Durgod K320
4. ✅ USB packet capture works

### Phase 2: Protocol Analysis
1. ✅ Capture baseline traffic (no key presses)
2. ✅ Capture single key press events
3. ✅ Capture key combinations (Ctrl+C, etc.)
4. ✅ Analyze HID report format
5. ✅ Document key code mappings

### Phase 3: Implementation
1. ✅ Simple key press injection
2. ✅ Text typing functionality
3. ✅ Key combination support
4. ✅ Error handling and validation

## ⚠️ Known Risks & Assumptions

### Assumptions Made
- **Standard HID Protocol**: Assumed Durgod K320 follows standard USB HID keyboard protocol
- **8-byte Reports**: Assumed standard 8-byte keyboard HID reports
- **No Vendor Extensions**: Assumed no proprietary Durgod-specific commands needed
- **Single Interface**: Assumed keyboard uses single HID interface

### Risks
- **Permission Issues**: May need sudo/udev rules for HID access
- **Non-standard Protocol**: Durgod may use proprietary extensions
- **Multiple Interfaces**: Keyboard may have multiple HID interfaces (media keys, etc.)
- **Security Restrictions**: Modern systems may block HID injection

## 💡 Lessons Learned

1. **Test Early**: Should have tested basic compilation and device access before building complex library
2. **Incremental Development**: Should build and test small pieces rather than complete implementation
3. **Real Hardware First**: Should identify actual device before assuming standard protocols
4. **Permission Planning**: USB/HID access permissions are complex and should be addressed early

## 🔄 Next Session Checklist

Before starting next work session:
1. [ ] Verify Rust compilation completes successfully
2. [ ] Test basic device scanning works
3. [ ] Confirm Durgod K320 is connected and detectable
4. [ ] Run first USB packet capture
5. [ ] Update this context with findings

## 📞 Quick Commands Reference

```bash
# Setup and compilation
source "$HOME/.cargo/env"
cargo check                    # Quick compilation check
cargo build --release         # Full build

# Testing
cargo run -- scan            # Scan all HID devices
cargo run -- info            # Show tool information
./scripts/identify_keyboard.sh  # Find keyboards

# USB monitoring
sudo modprobe usbmon          # Load USB monitoring
./scripts/capture_traffic.sh  # Capture USB traffic
tshark -D | grep -i usb       # Show USB interfaces
```

---

**Note**: This project is currently in early development phase. All code should be considered experimental and tested thoroughly before use. 