#!/bin/bash

# Durgod K320 Traffic Capture Script
# Captures USB HID traffic for protocol analysis

set -e

echo "📊 Capturing USB traffic from Durgod K320 keyboard..."

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

# Check if tshark is available
if ! command -v tshark &> /dev/null; then
    print_error "tshark command not found. Please run setup.sh first."
    exit 1
fi

# Check if running with proper permissions
if ! groups | grep -q wireshark; then
    print_error "User not in wireshark group. Please logout/login or run: newgrp wireshark"
    exit 1
fi

# Check if usbmon is loaded
if ! lsmod | grep -q usbmon; then
    print_warning "usbmon module not loaded. Attempting to load..."
    sudo modprobe usbmon || {
        print_error "Failed to load usbmon module"
        exit 1
    }
fi

# Parse command line arguments
DURATION=60  # Default capture duration in seconds
VENDOR_ID=""
PRODUCT_ID=""
CAPTURE_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--duration)
            DURATION="$2"
            shift 2
            ;;
        -v|--vendor)
            VENDOR_ID="$2"
            shift 2
            ;;
        -p|--product)
            PRODUCT_ID="$2"
            shift 2
            ;;
        -f|--file)
            CAPTURE_FILE="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -d, --duration SECONDS    Capture duration (default: 60)"
            echo "  -v, --vendor VENDOR_ID    Filter by vendor ID (hex, e.g., 05ac)"
            echo "  -p, --product PRODUCT_ID  Filter by product ID (hex, e.g., 024f)"
            echo "  -f, --file FILENAME       Output filename (default: auto-generated)"
            echo "  -h, --help               Show this help"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Create capture filename if not specified
if [ -z "$CAPTURE_FILE" ]; then
    TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
    CAPTURE_FILE="../captures/durgod_k320_${TIMESTAMP}.pcap"
fi

print_step "Setting up capture parameters..."
print_status "Capture duration: ${DURATION} seconds"
print_status "Output file: $CAPTURE_FILE"

# Create captures directory if it doesn't exist
mkdir -p ../captures

# Build tshark filter
FILTER="usb"
if [ ! -z "$VENDOR_ID" ] && [ ! -z "$PRODUCT_ID" ]; then
    FILTER="usb and usb.idVendor == 0x$VENDOR_ID and usb.idProduct == 0x$PRODUCT_ID"
    print_status "Filtering for device: $VENDOR_ID:$PRODUCT_ID"
elif [ ! -z "$VENDOR_ID" ]; then
    FILTER="usb and usb.idVendor == 0x$VENDOR_ID"
    print_status "Filtering for vendor: $VENDOR_ID"
fi

# Show available USB interfaces
print_step "Available USB capture interfaces:"
tshark -D | grep -i usb || print_warning "No USB interfaces found"

# Try to find usbmon interfaces
USBMON_INTERFACES=$(tshark -D | grep -i usbmon | head -1 || true)
if [ -z "$USBMON_INTERFACES" ]; then
    print_error "No usbmon interfaces found. Is usbmon loaded?"
    print_status "Available interfaces:"
    tshark -D
    exit 1
fi

# Extract interface name
INTERFACE=$(echo "$USBMON_INTERFACES" | cut -d'.' -f2 | cut -d' ' -f1)
print_status "Using interface: $INTERFACE"

print_step "Starting packet capture..."
print_warning "Please interact with your Durgod K320 keyboard now!"
print_status "Try pressing various keys and key combinations."

# Create a text log of the capture session
LOG_FILE="${CAPTURE_FILE%.pcap}.log"
echo "Capture session started: $(date)" > "$LOG_FILE"
echo "Interface: $INTERFACE" >> "$LOG_FILE"
echo "Filter: $FILTER" >> "$LOG_FILE"
echo "Duration: $DURATION seconds" >> "$LOG_FILE"
echo "========================================" >> "$LOG_FILE"

# Start capture with timeout
echo "🚀 Capturing for $DURATION seconds..."
echo "Press Ctrl+C to stop early"

# Run tshark capture
timeout "$DURATION" tshark -i "$INTERFACE" -f "$FILTER" -w "$CAPTURE_FILE" -P || {
    CAPTURE_RESULT=$?
    if [ $CAPTURE_RESULT -eq 124 ]; then
        print_status "Capture completed (timeout reached)"
    else
        print_warning "Capture stopped early"
    fi
}

# Check if capture file was created and has content
if [ -f "$CAPTURE_FILE" ]; then
    FILE_SIZE=$(stat -f%z "$CAPTURE_FILE" 2>/dev/null || stat -c%s "$CAPTURE_FILE" 2>/dev/null)
    if [ "$FILE_SIZE" -gt 24 ]; then  # pcap header is 24 bytes
        print_status "✓ Capture successful! File size: $FILE_SIZE bytes"
        
        # Get packet count
        PACKET_COUNT=$(tshark -r "$CAPTURE_FILE" | wc -l)
        print_status "✓ Captured $PACKET_COUNT packets"
        
        echo "Packets captured: $PACKET_COUNT" >> "$LOG_FILE"
        echo "File size: $FILE_SIZE bytes" >> "$LOG_FILE"
    else
        print_warning "Capture file is empty or too small"
        echo "WARNING: Empty capture file" >> "$LOG_FILE"
    fi
else
    print_error "Capture file was not created"
    echo "ERROR: Capture file not created" >> "$LOG_FILE"
fi

print_step "Quick analysis of captured data..."

if [ -f "$CAPTURE_FILE" ] && [ "$FILE_SIZE" -gt 24 ]; then
    # Show HID data packets
    echo
    print_status "HID Data packets (first 10):"
    tshark -r "$CAPTURE_FILE" -Y "usb.transfer_type == 1" -T fields -e frame.number -e usb.capdata | head -10 || print_warning "No HID data found"
    
    # Show packet summary
    echo
    print_status "Packet summary:"
    tshark -r "$CAPTURE_FILE" -q -z io,stat,0 || true
    
    # Show unique USB addresses
    echo
    print_status "USB devices in capture:"
    tshark -r "$CAPTURE_FILE" -T fields -e usb.device_address | sort -u | head -10 || true
    
    # Save detailed analysis
    ANALYSIS_FILE="${CAPTURE_FILE%.pcap}_analysis.txt"
    echo "Detailed packet analysis - $(date)" > "$ANALYSIS_FILE"
    echo "========================================" >> "$ANALYSIS_FILE"
    
    echo "All packets:" >> "$ANALYSIS_FILE"
    tshark -r "$CAPTURE_FILE" -T fields -e frame.number -e frame.time -e usb.transfer_type -e usb.endpoint_address -e usb.capdata >> "$ANALYSIS_FILE" 2>/dev/null || true
    
    print_status "Detailed analysis saved to: $ANALYSIS_FILE"
fi

echo
print_status "📁 Files created:"
echo "  Capture file: $CAPTURE_FILE"
echo "  Session log: $LOG_FILE"
if [ -f "$ANALYSIS_FILE" ]; then
    echo "  Analysis: $ANALYSIS_FILE"
fi

echo
print_step "Next steps:"
echo "1. Review the captured data with: tshark -r $CAPTURE_FILE"
echo "2. Filter HID data: tshark -r $CAPTURE_FILE -Y 'usb.transfer_type == 1'"
echo "3. Run ./analyze_packets.sh $CAPTURE_FILE for detailed analysis"
echo "4. Open in Wireshark GUI: wireshark $CAPTURE_FILE"

echo
print_status "Capture session complete! 🎉" 