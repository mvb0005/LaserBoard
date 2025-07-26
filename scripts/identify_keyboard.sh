#!/bin/bash

# Durgod K320 Keyboard Identification Script
# Identifies the USB device details for the Durgod K320 keyboard

set -e

echo "🔍 Identifying Durgod K320 keyboard..."

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if lsusb is available
if ! command -v lsusb &> /dev/null; then
    print_error "lsusb command not found. Please run setup.sh first."
    exit 1
fi

print_step "Scanning for USB devices..."

# Look for Durgod devices (common vendor IDs)
# Note: Durgod might use different vendor IDs, we'll search broadly
DURGOD_DEVICES=$(lsusb | grep -i -E "(durgod|keyboard)" || true)

if [ -z "$DURGOD_DEVICES" ]; then
    print_warning "No devices found with 'durgod' or 'keyboard' in name."
    print_status "Showing all USB HID devices instead..."
    
    # Show all HID devices (class 03)
    print_step "USB HID devices found:"
    lsusb | while read line; do
        BUS=$(echo "$line" | cut -d' ' -f2)
        DEVICE=$(echo "$line" | cut -d' ' -f4 | tr -d ':')
        VENDOR_PRODUCT=$(echo "$line" | cut -d' ' -f6)
        NAME=$(echo "$line" | cut -d' ' -f7-)
        
        # Check if device is HID class
        DEVICE_CLASS=$(lsusb -v -s "$BUS:$DEVICE" 2>/dev/null | grep "bInterfaceClass.*3 Human Interface Device" || true)
        if [ ! -z "$DEVICE_CLASS" ]; then
            echo "  $VENDOR_PRODUCT - $NAME"
            echo "    Bus: $BUS, Device: $DEVICE"
        fi
    done
else
    print_status "Found potential Durgod/keyboard devices:"
    echo "$DURGOD_DEVICES"
fi

echo
print_step "All connected USB devices:"
lsusb

echo
print_step "Detailed HID device information:"

# Create output file for device details
OUTPUT_FILE="../docs/device_info.txt"
echo "USB Device Scan - $(date)" > "$OUTPUT_FILE"
echo "================================================" >> "$OUTPUT_FILE"

# Scan each USB device for HID interfaces
lsusb | while read line; do
    BUS=$(echo "$line" | cut -d' ' -f2)
    DEVICE=$(echo "$line" | cut -d' ' -f4 | tr -d ':')
    VENDOR_PRODUCT=$(echo "$line" | cut -d' ' -f6)
    VENDOR_ID=$(echo "$VENDOR_PRODUCT" | cut -d':' -f1)
    PRODUCT_ID=$(echo "$VENDOR_PRODUCT" | cut -d':' -f2)
    NAME=$(echo "$line" | cut -d' ' -f7-)
    
    # Get detailed device information
    DEVICE_INFO=$(lsusb -v -s "$BUS:$DEVICE" 2>/dev/null || true)
    
    # Check if this device has HID interfaces
    HID_INTERFACES=$(echo "$DEVICE_INFO" | grep -c "bInterfaceClass.*3 Human Interface Device" || true)
    
    if [ "$HID_INTERFACES" -gt 0 ]; then
        echo
        echo "HID Device Found:"
        echo "  Name: $NAME"
        echo "  Vendor ID: $VENDOR_ID"
        echo "  Product ID: $PRODUCT_ID"
        echo "  Bus: $BUS, Device: $DEVICE"
        echo "  HID Interfaces: $HID_INTERFACES"
        
        # Save to file
        echo "" >> "$OUTPUT_FILE"
        echo "Device: $NAME" >> "$OUTPUT_FILE"
        echo "Vendor:Product = $VENDOR_PRODUCT" >> "$OUTPUT_FILE"
        echo "Bus $BUS Device $DEVICE" >> "$OUTPUT_FILE"
        echo "HID Interfaces: $HID_INTERFACES" >> "$OUTPUT_FILE"
        
        # Extract HID report descriptor if possible
        echo "  Checking for HID report descriptors..."
        HID_REPORT=$(echo "$DEVICE_INFO" | grep -A 20 "HID Device Descriptor" || true)
        if [ ! -z "$HID_REPORT" ]; then
            echo "    HID report descriptor found"
            echo "$HID_REPORT" >> "$OUTPUT_FILE"
        fi
        
        # Check for keyboard protocol
        KEYBOARD_PROTOCOL=$(echo "$DEVICE_INFO" | grep "bInterfaceProtocol.*1 Keyboard" || true)
        if [ ! -z "$KEYBOARD_PROTOCOL" ]; then
            echo "  ⌨️  KEYBOARD PROTOCOL DETECTED!"
            echo "KEYBOARD PROTOCOL DETECTED" >> "$OUTPUT_FILE"
        fi
        
        # Check for mouse protocol  
        MOUSE_PROTOCOL=$(echo "$DEVICE_INFO" | grep "bInterfaceProtocol.*2 Mouse" || true)
        if [ ! -z "$MOUSE_PROTOCOL" ]; then
            echo "  🖱️  MOUSE PROTOCOL DETECTED!"
            echo "MOUSE PROTOCOL DETECTED" >> "$OUTPUT_FILE"
        fi
        
        echo "----------------------------------------" >> "$OUTPUT_FILE"
    fi
done

echo
print_step "Checking /dev/hidraw devices..."
if ls /dev/hidraw* &>/dev/null; then
    echo "HID raw devices found:"
    for hidraw in /dev/hidraw*; do
        if [ -r "$hidraw" ]; then
            echo "  $hidraw (readable)"
        else
            echo "  $hidraw (not readable - check permissions)"
        fi
    done
else
    print_warning "No /dev/hidraw devices found"
fi

echo
print_status "Device information saved to: $OUTPUT_FILE"

echo
print_step "Manual identification help:"
echo "If your Durgod K320 wasn't automatically identified, look for:"
echo "1. A device with HID class (Human Interface Device)"
echo "2. Keyboard protocol (bInterfaceProtocol = 1)"
echo "3. Vendor names like 'Holtek', 'Chicony', or generic descriptions"
echo "4. Recent devices if you just plugged in the keyboard"

echo
echo "To capture traffic for a specific device, note its:"
echo "- Vendor ID (first 4 hex digits, e.g., 05ac)"
echo "- Product ID (last 4 hex digits, e.g., 024f)"
echo "- Bus and Device numbers"

echo
print_status "Next step: Run ./capture_traffic.sh to begin packet capture" 