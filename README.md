# Durgod K320 Keyboard Protocol Reverse Engineering

## Project Overview

This project aims to reverse engineer the USB HID protocol used by the Durgod K320 mechanical keyboard to enable programmatic key press simulation. The end goal is to create a Rust application that can send synthetic key press events directly to the keyboard interface.

## Hardware Target

- **Device**: Durgod K320 Mechanical Keyboard
- **Interface**: USB HID (Human Interface Device)
- **Expected Protocol**: Standard HID keyboard protocol with potential vendor-specific extensions

## Project Phases

### Phase 1: Environment Setup
- Install Wireshark/tshark for packet capture
- Configure usbmon for USB traffic monitoring
- Set up development tools and dependencies
- Identify keyboard USB device details

### Phase 2: Traffic Capture & Analysis
- Capture baseline USB traffic during normal keyboard operation
- Record traffic patterns for different key combinations
- Analyze HID report descriptors and endpoint configurations
- Document packet structure and timing

### Phase 3: Protocol Documentation
- Map key codes to USB HID scan codes
- Document packet format and protocol specifics
- Identify any vendor-specific commands or features
- Create protocol specification document

### Phase 4: Rust Implementation
- Create Rust application using HID libraries (hidapi/rusb)
- Implement low-level USB communication
- Build key press injection functionality
- Add error handling and device detection

### Phase 5: Testing & Validation
- Test synthetic key press generation
- Validate timing and protocol compliance
- Ensure compatibility and stability
- Document usage and limitations

## Technical Approach

### USB HID Fundamentals
The Durgod K320 likely implements the standard USB HID keyboard boot protocol:
- **Class**: HID (0x03)
- **Subclass**: Boot Interface (0x01) 
- **Protocol**: Keyboard (0x01)
- **Report Size**: Typically 8 bytes for standard keyboards

### Expected Packet Structure
Standard HID keyboard reports typically follow this format:
```
Byte 0: Modifier keys (Ctrl, Alt, Shift, etc.)
Byte 1: Reserved (usually 0x00)
Byte 2-7: Key codes (up to 6 simultaneous key presses)
```

### Tools and Dependencies
- **Wireshark/tshark**: Packet capture and analysis
- **usbmon**: Linux USB monitoring
- **libusb**: Low-level USB communication
- **Rust**: Final implementation language
- **hidapi-rs**: Rust HID library

## Security and Ethics Notice

⚠️ **Important**: This project is for educational and research purposes. Ensure you:
- Only test on hardware you own
- Understand the legal implications in your jurisdiction
- Do not use for malicious purposes
- Respect manufacturer warranties and terms of service

## Getting Started

1. Run the setup script: `./scripts/setup.sh`
2. Connect your Durgod K320 keyboard
3. Run device identification: `./scripts/identify_keyboard.sh`
4. Begin packet capture: `./scripts/capture_traffic.sh`

## Repository Structure

```
LaserBoard/
├── README.md                 # This file
├── scripts/
│   ├── setup.sh             # Environment setup
│   ├── identify_keyboard.sh  # Device identification
│   ├── capture_traffic.sh    # Traffic capture
│   └── analyze_packets.sh    # Packet analysis
├── docs/
│   ├── protocol.md          # Protocol documentation
│   └── findings.md          # Research findings
├── captures/                # Packet capture files
├── src/                     # Rust source code
└── Cargo.toml              # Rust dependencies
```

## Contributing

This is a research project. Contributions, findings, and insights are welcome through issues and pull requests.

## License

MIT License - See LICENSE file for details. 