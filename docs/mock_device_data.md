# Mock Device Data for Durgod K320

**Purpose**: Simulated device information for development without physical hardware  
**Created by**: Stream B (Hardware Setup Specialist)

## 🎯 Typical Durgod K320 USB HID Information

Based on research of similar mechanical keyboards and standard USB HID specifications:

### Device Identification
```
Vendor ID (VID): 05f3    # Typical for mechanical keyboard manufacturers
Product ID (PID): 0007   # Example product ID
Device Name: "Durgod K320 Mechanical Keyboard"
Manufacturer: "Durgod"
Interface Class: 03 (Human Interface Device)
Interface Protocol: 01 (Keyboard)
Bus: USB 2.0
```

### HID Report Descriptor (Mock)
```
Standard USB HID Keyboard Report Format:
- Byte 0: Modifier keys (Ctrl, Alt, Shift, Windows key)
- Byte 1: Reserved/OEM
- Bytes 2-7: Up to 6 simultaneous key codes
```

### Key Code Mapping Examples
```
Key         | HID Code | Notes
------------|----------|------------------
A           | 0x04     | Standard letter
Enter       | 0x28     | Return key
Space       | 0x2C     | Space bar
Left Ctrl   | 0x01     | In modifier byte
Left Shift  | 0x02     | In modifier byte
Escape      | 0x29     | Function key
F1          | 0x3A     | Function key
```

### Sample USB Traffic Captures (Mock)

#### Idle State (No Keys Pressed)
```
HID Report: 00 00 00 00 00 00 00 00
- All bytes zero = no keys pressed
```

#### Single Key Press (Letter 'A')
```
Press Event:   00 00 04 00 00 00 00 00
Release Event: 00 00 00 00 00 00 00 00
- Byte 2 = 0x04 (HID code for 'A')
```

#### Modifier + Key (Ctrl+C)
```
Press Event:   01 00 06 00 00 00 00 00
Release Event: 00 00 00 00 00 00 00 00
- Byte 0 = 0x01 (Left Ctrl modifier)
- Byte 2 = 0x06 (HID code for 'C')
```

#### Multiple Keys (Shift+A+B)
```
Press Event:   02 00 04 05 00 00 00 00
Release Event: 00 00 00 00 00 00 00 00
- Byte 0 = 0x02 (Left Shift modifier)
- Byte 2 = 0x04 (HID code for 'A')
- Byte 3 = 0x05 (HID code for 'B')
```

## 🔧 Device Interface Information

### USB Descriptors (Mock)
```
Device Descriptor:
  bcdUSB               2.00
  bDeviceClass            0 
  bDeviceSubClass         0 
  bDeviceProtocol         0 
  bMaxPacketSize0        64
  idVendor           0x05f3
  idProduct          0x0007
  bcdDevice            1.00
  iManufacturer           1 Durgod
  iProduct                2 K320 Mechanical Keyboard
  iSerial                 3 DK320-001
  bNumConfigurations      1

Interface Descriptor:
  bInterfaceNumber        0
  bAlternateSetting       0
  bNumEndpoints           1
  bInterfaceClass         3 Human Interface Device
  bInterfaceSubClass      1 Boot Interface Subclass
  bInterfaceProtocol      1 Keyboard
  iInterface              4 HID Keyboard
```

### Endpoint Information
```
Endpoint Descriptor:
  bEndpointAddress     0x81  EP 1 IN
  bmAttributes            3
    Transfer Type            Interrupt
    Synch Type               None
    Usage Type               Data
  wMaxPacketSize     0x0008  1x 8 bytes
  bInterval              10
```

## 📁 Mock Files for Development

### Sample Packet Capture (for Stream C)
```
# File: captures/mock_k320_traffic.txt
# Timestamp: 2024-12-26 14:30:00
# Device: Mock Durgod K320 (05f3:0007)

# Idle state
14:30:00.000000 URB_INTERRUPT in  00 00 00 00 00 00 00 00

# Press 'H' key
14:30:01.000000 URB_INTERRUPT in  00 00 0B 00 00 00 00 00
14:30:01.100000 URB_INTERRUPT in  00 00 00 00 00 00 00 00

# Press 'i' key  
14:30:02.000000 URB_INTERRUPT in  00 00 0C 00 00 00 00 00
14:30:02.100000 URB_INTERRUPT in  00 00 00 00 00 00 00 00

# Type "Hello" sequence would continue...
```

### Device Information File
```
# File: docs/device_info.txt (Mock)
USB Device Scan - December 26, 2024 (MOCK DATA)
================================================

Device: Durgod K320 Mechanical Keyboard
Vendor:Product = 05f3:0007
Bus 001 Device 003
HID Interfaces: 1
KEYBOARD PROTOCOL DETECTED
```

## 🎮 Testing Commands (Mock Results)

When physical hardware becomes available, these commands should work:

```bash
# Device identification
lsusb | grep -i durgod
# Expected: Bus 001 Device 003: ID 05f3:0007 Durgod K320 Mechanical Keyboard

# HID device access
ls /dev/hidraw*
# Expected: /dev/hidraw0 /dev/hidraw1 (etc.)

# Traffic capture
tshark -i usbmon1 -Y "usb.device_address == 3"
```

## 🚀 Development Instructions

### For Stream A (Rust Compilation)
- Use VID: `0x05f3`, PID: `0x0007` in test code
- Mock device path: `/dev/hidraw0` (will fail gracefully)

### For Stream C (Protocol Analysis)  
- Use mock packet data above for protocol parsing tests
- Standard 8-byte HID keyboard reports
- Implement parsing for the sample traffic patterns

### For Stream D (Implementation)
- Create HID reports using the 8-byte format above
- Test key code mapping with the provided examples
- Implement modifier key combinations

---

**Note**: This mock data represents typical USB HID keyboard behavior. Actual Durgod K320 may have variations in VID/PID or additional features.