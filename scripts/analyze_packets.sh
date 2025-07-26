#!/bin/bash

# Durgod K320 Packet Analysis Script
# Analyzes captured USB HID traffic to understand the protocol

set -e

echo "🔬 Analyzing Durgod K320 USB packets..."

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

# Check if capture file is provided
if [ $# -eq 0 ]; then
    print_error "Usage: $0 <capture_file.pcap>"
    echo "Example: $0 ../captures/durgod_k320_20231201_143022.pcap"
    exit 1
fi

CAPTURE_FILE="$1"

# Check if capture file exists
if [ ! -f "$CAPTURE_FILE" ]; then
    print_error "Capture file not found: $CAPTURE_FILE"
    exit 1
fi

# Check if tshark is available
if ! command -v tshark &> /dev/null; then
    print_error "tshark command not found. Please run setup.sh first."
    exit 1
fi

print_step "Analyzing capture file: $CAPTURE_FILE"

# Create analysis output directory
ANALYSIS_DIR="../docs/analysis_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$ANALYSIS_DIR"

print_status "Analysis results will be saved to: $ANALYSIS_DIR"

# Basic file information
FILE_SIZE=$(stat -f%z "$CAPTURE_FILE" 2>/dev/null || stat -c%s "$CAPTURE_FILE" 2>/dev/null)
PACKET_COUNT=$(tshark -r "$CAPTURE_FILE" | wc -l)

print_step "Basic file information:"
echo "  File size: $FILE_SIZE bytes"
echo "  Total packets: $PACKET_COUNT"

# Save basic info
echo "Packet Analysis Report" > "$ANALYSIS_DIR/report.txt"
echo "======================" >> "$ANALYSIS_DIR/report.txt"
echo "File: $CAPTURE_FILE" >> "$ANALYSIS_DIR/report.txt"
echo "Analysis date: $(date)" >> "$ANALYSIS_DIR/report.txt"
echo "File size: $FILE_SIZE bytes" >> "$ANALYSIS_DIR/report.txt"
echo "Total packets: $PACKET_COUNT" >> "$ANALYSIS_DIR/report.txt"
echo "" >> "$ANALYSIS_DIR/report.txt"

if [ "$PACKET_COUNT" -eq 0 ]; then
    print_warning "No packets found in capture file"
    exit 0
fi

# Analyze USB devices in capture
print_step "USB devices found in capture:"
tshark -r "$CAPTURE_FILE" -T fields -e usb.idVendor -e usb.idProduct -e usb.device_address | sort -u > "$ANALYSIS_DIR/devices.txt"

echo "USB Devices:" >> "$ANALYSIS_DIR/report.txt"
while read line; do
    if [ ! -z "$line" ]; then
        VENDOR=$(echo "$line" | cut -f1)
        PRODUCT=$(echo "$line" | cut -f2)
        ADDRESS=$(echo "$line" | cut -f3)
        if [ ! -z "$VENDOR" ] && [ ! -z "$PRODUCT" ]; then
            echo "  Device: $VENDOR:$PRODUCT (Address: $ADDRESS)"
            echo "  Device: $VENDOR:$PRODUCT (Address: $ADDRESS)" >> "$ANALYSIS_DIR/report.txt"
        fi
    fi
done < "$ANALYSIS_DIR/devices.txt"

# Analyze HID interrupt transfers (key presses)
print_step "Extracting HID interrupt transfers..."
tshark -r "$CAPTURE_FILE" -Y "usb.transfer_type == 1 and usb.endpoint_address.direction == 1" \
    -T fields -e frame.number -e frame.time -e usb.capdata -e usb.data_len > "$ANALYSIS_DIR/hid_interrupts.txt"

HID_COUNT=$(wc -l < "$ANALYSIS_DIR/hid_interrupts.txt")
print_status "Found $HID_COUNT HID interrupt transfers"
echo "HID interrupt transfers: $HID_COUNT" >> "$ANALYSIS_DIR/report.txt"

if [ "$HID_COUNT" -gt 0 ]; then
    print_step "Analyzing HID keyboard reports..."
    
    # Extract unique HID data patterns
    cat "$ANALYSIS_DIR/hid_interrupts.txt" | cut -f3 | sort | uniq -c | sort -nr > "$ANALYSIS_DIR/hid_patterns.txt"
    
    echo "" >> "$ANALYSIS_DIR/report.txt"
    echo "Most common HID patterns:" >> "$ANALYSIS_DIR/report.txt"
    head -20 "$ANALYSIS_DIR/hid_patterns.txt" >> "$ANALYSIS_DIR/report.txt"
    
    print_status "Top 10 HID data patterns:"
    head -10 "$ANALYSIS_DIR/hid_patterns.txt"
    
    # Analyze HID report structure
    print_step "Analyzing HID report structure..."
    
    # Look for 8-byte keyboard reports (standard HID keyboard)
    tshark -r "$CAPTURE_FILE" -Y "usb.transfer_type == 1 and usb.data_len == 8" \
        -T fields -e frame.number -e usb.capdata > "$ANALYSIS_DIR/keyboard_reports.txt"
    
    KEYBOARD_REPORTS=$(wc -l < "$ANALYSIS_DIR/keyboard_reports.txt")
    print_status "Found $KEYBOARD_REPORTS standard 8-byte keyboard reports"
    
    if [ "$KEYBOARD_REPORTS" -gt 0 ]; then
        echo "" >> "$ANALYSIS_DIR/report.txt"
        echo "Standard keyboard reports (8 bytes): $KEYBOARD_REPORTS" >> "$ANALYSIS_DIR/report.txt"
        
        # Analyze modifier byte patterns (first byte)
        print_step "Analyzing modifier key patterns..."
        cut -f2 "$ANALYSIS_DIR/keyboard_reports.txt" | cut -c1-2 | sort | uniq -c | sort -nr > "$ANALYSIS_DIR/modifiers.txt"
        
        echo "" >> "$ANALYSIS_DIR/report.txt"
        echo "Modifier byte patterns (first byte of HID report):" >> "$ANALYSIS_DIR/report.txt"
        head -10 "$ANALYSIS_DIR/modifiers.txt" >> "$ANALYSIS_DIR/report.txt"
        
        print_status "Modifier key usage:"
        head -5 "$ANALYSIS_DIR/modifiers.txt"
        
        # Analyze key code patterns (bytes 3-8)
        print_step "Analyzing key code patterns..."
        cut -f2 "$ANALYSIS_DIR/keyboard_reports.txt" | sed 's/^....//' | tr -d ':' | grep -v '^000000000000$' | sort | uniq -c | sort -nr > "$ANALYSIS_DIR/keycodes.txt"
        
        UNIQUE_KEYCODES=$(wc -l < "$ANALYSIS_DIR/keycodes.txt")
        print_status "Found $UNIQUE_KEYCODES unique key code patterns"
        
        echo "" >> "$ANALYSIS_DIR/report.txt"
        echo "Key code patterns (bytes 3-8): $UNIQUE_KEYCODES unique patterns" >> "$ANALYSIS_DIR/report.txt"
        head -10 "$ANALYSIS_DIR/keycodes.txt" >> "$ANALYSIS_DIR/report.txt"
        
        print_status "Top 5 key code patterns:"
        head -5 "$ANALYSIS_DIR/keycodes.txt"
    fi
    
    # Look for non-standard reports
    print_step "Looking for non-standard reports..."
    tshark -r "$CAPTURE_FILE" -Y "usb.transfer_type == 1 and usb.data_len != 8" \
        -T fields -e frame.number -e usb.data_len -e usb.capdata > "$ANALYSIS_DIR/nonstandard_reports.txt"
    
    NONSTANDARD_COUNT=$(wc -l < "$ANALYSIS_DIR/nonstandard_reports.txt")
    if [ "$NONSTANDARD_COUNT" -gt 0 ]; then
        print_warning "Found $NONSTANDARD_COUNT non-standard report sizes"
        echo "" >> "$ANALYSIS_DIR/report.txt"
        echo "Non-standard reports: $NONSTANDARD_COUNT" >> "$ANALYSIS_DIR/report.txt"
        head -10 "$ANALYSIS_DIR/nonstandard_reports.txt" >> "$ANALYSIS_DIR/report.txt"
    fi
    
    # Timeline analysis
    print_step "Creating timeline analysis..."
    tshark -r "$CAPTURE_FILE" -Y "usb.transfer_type == 1" \
        -T fields -e frame.time -e usb.capdata | head -50 > "$ANALYSIS_DIR/timeline.txt"
    
    echo "" >> "$ANALYSIS_DIR/report.txt"
    echo "Sample timeline (first 50 HID events):" >> "$ANALYSIS_DIR/report.txt"
    cat "$ANALYSIS_DIR/timeline.txt" >> "$ANALYSIS_DIR/report.txt"
fi

# Control transfers analysis
print_step "Analyzing control transfers..."
tshark -r "$CAPTURE_FILE" -Y "usb.transfer_type == 2" \
    -T fields -e frame.number -e usb.setup.bmRequestType -e usb.setup.bRequest -e usb.setup.wValue > "$ANALYSIS_DIR/control_transfers.txt"

CONTROL_COUNT=$(wc -l < "$ANALYSIS_DIR/control_transfers.txt")
print_status "Found $CONTROL_COUNT control transfers"

if [ "$CONTROL_COUNT" -gt 0 ]; then
    echo "" >> "$ANALYSIS_DIR/report.txt"
    echo "Control transfers: $CONTROL_COUNT" >> "$ANALYSIS_DIR/report.txt"
    head -10 "$ANALYSIS_DIR/control_transfers.txt" >> "$ANALYSIS_DIR/report.txt"
fi

# Generate protocol summary
print_step "Generating protocol summary..."

PROTOCOL_FILE="$ANALYSIS_DIR/protocol_summary.md"
cat > "$PROTOCOL_FILE" << EOF
# Durgod K320 Protocol Analysis Summary

## Overview
- **Capture file**: $CAPTURE_FILE
- **Total packets**: $PACKET_COUNT
- **HID interrupt transfers**: $HID_COUNT
- **Standard keyboard reports**: $KEYBOARD_REPORTS
- **Analysis date**: $(date)

## HID Keyboard Protocol

### Standard Report Format (8 bytes)
\`\`\`
Byte 0: Modifier keys (Ctrl, Alt, Shift, etc.)
Byte 1: Reserved (usually 0x00)
Byte 2-7: Key codes (up to 6 simultaneous keys)
\`\`\`

### Common Modifier Values
EOF

if [ -f "$ANALYSIS_DIR/modifiers.txt" ]; then
    echo "| Count | Hex Value | Meaning |" >> "$PROTOCOL_FILE"
    echo "|-------|-----------|---------|" >> "$PROTOCOL_FILE"
    head -5 "$ANALYSIS_DIR/modifiers.txt" | while read count hex; do
        case "$hex" in
            "00") meaning="No modifiers" ;;
            "01") meaning="Left Ctrl" ;;
            "02") meaning="Left Shift" ;;
            "04") meaning="Left Alt" ;;
            "08") meaning="Left GUI (Windows)" ;;
            "10") meaning="Right Ctrl" ;;
            "20") meaning="Right Shift" ;;
            "40") meaning="Right Alt" ;;
            "80") meaning="Right GUI" ;;
            *) meaning="Combined/Unknown" ;;
        esac
        echo "| $count | 0x$hex | $meaning |" >> "$PROTOCOL_FILE"
    done
fi

cat >> "$PROTOCOL_FILE" << EOF

### Key Code Analysis
- Unique key patterns found: $UNIQUE_KEYCODES
- Most packets contain standard 8-byte HID keyboard reports
- Key codes follow USB HID usage table standards

## Next Steps for Implementation
1. Focus on 8-byte HID keyboard report format
2. Map specific key codes to their scan codes
3. Implement HID report generation in Rust
4. Test with libusb or hidapi for device communication

## Files Generated
- \`report.txt\`: Complete analysis report
- \`hid_interrupts.txt\`: All HID interrupt transfers
- \`keyboard_reports.txt\`: Standard 8-byte keyboard reports
- \`modifiers.txt\`: Modifier key usage patterns
- \`keycodes.txt\`: Key code patterns
- \`timeline.txt\`: Sample event timeline
EOF

print_status "✅ Analysis complete!"
echo
print_step "Files generated in $ANALYSIS_DIR:"
ls -la "$ANALYSIS_DIR/"

echo
print_step "Key findings:"
echo "- Total packets analyzed: $PACKET_COUNT"
echo "- HID interrupt transfers: $HID_COUNT"
echo "- Standard keyboard reports: $KEYBOARD_REPORTS"
if [ "$NONSTANDARD_COUNT" -gt 0 ]; then
    echo "- Non-standard reports: $NONSTANDARD_COUNT (may be vendor-specific)"
fi

echo
print_status "📖 Read the protocol summary: $PROTOCOL_FILE"
print_status "📊 View full report: $ANALYSIS_DIR/report.txt"

echo
print_step "Recommended next steps:"
echo "1. Review the protocol summary and HID patterns"
echo "2. Identify specific key codes for your target keys"
echo "3. Create Rust implementation using the discovered protocol"
echo "4. Test with a simple key press simulation"

# Mark TODO as complete
echo
print_status "You can now proceed to implement the Rust HID interface! 🦀" 