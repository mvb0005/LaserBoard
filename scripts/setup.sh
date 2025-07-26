#!/bin/bash

# Durgod K320 Reverse Engineering - Environment Setup Script
# This script installs all necessary tools for USB HID protocol analysis

set -e

echo "🔧 Setting up Durgod K320 reverse engineering environment..."

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

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   print_error "This script should not be run as root for security reasons."
   exit 1
fi

# Update package repositories
print_step "Updating package repositories..."
sudo apt update

# Install Wireshark and tshark
print_step "Installing Wireshark and tshark..."
sudo apt install -y wireshark tshark

# Install USB monitoring tools
print_step "Installing USB monitoring tools..."
sudo apt install -y usbutils libusb-1.0-0-dev

# Install development tools
print_step "Installing development tools..."
sudo apt install -y build-essential pkg-config git curl

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    print_step "Installing Rust programming language..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_status "Rust installed successfully"
else
    print_status "Rust is already installed"
fi

# Install additional USB analysis tools
print_step "Installing additional USB analysis tools..."
sudo apt install -y lsusb usbview

# Install Python tools for USB analysis (optional but useful)
print_step "Installing Python USB libraries..."
sudo apt install -y python3-pip python3-usb

# Setup usbmon module
print_step "Setting up USB monitoring capabilities..."
sudo modprobe usbmon

# Add user to wireshark group for packet capture permissions
print_step "Configuring Wireshark permissions..."
sudo usermod -a -G wireshark $USER

# Create udev rules for HID device access
print_step "Setting up HID device access rules..."
sudo tee /etc/udev/rules.d/99-hid-permissions.rules > /dev/null << EOF
# Allow users in plugdev group to access HID devices
SUBSYSTEM=="hidraw", GROUP="plugdev", MODE="0664"
SUBSYSTEM=="usb", GROUP="plugdev", MODE="0664"
EOF

# Add user to plugdev group
sudo usermod -a -G plugdev $USER

# Reload udev rules
sudo udevadm control --reload-rules

# Create project structure directories if they don't exist
print_step "Setting up project directories..."
mkdir -p ../docs ../captures ../src

# Initialize Rust project
if [ ! -f "../Cargo.toml" ]; then
    print_step "Initializing Rust project..."
    cd ..
    cargo init --name durgod-k320-controller
    cd scripts
    print_status "Rust project initialized"
fi

# Check if reboot is needed for group changes
print_warning "Group changes require logout/login or reboot to take effect."
print_warning "You may need to run 'newgrp wireshark' and 'newgrp plugdev' in your current session."

# Verify installations
print_step "Verifying installations..."

command -v wireshark >/dev/null 2>&1 && print_status "✓ Wireshark installed" || print_error "✗ Wireshark installation failed"
command -v tshark >/dev/null 2>&1 && print_status "✓ tshark installed" || print_error "✗ tshark installation failed"
command -v lsusb >/dev/null 2>&1 && print_status "✓ lsusb installed" || print_error "✗ lsusb installation failed"
command -v rustc >/dev/null 2>&1 && print_status "✓ Rust installed" || print_error "✗ Rust installation failed"

# Check if usbmon is loaded
if lsmod | grep usbmon > /dev/null; then
    print_status "✓ usbmon kernel module loaded"
else
    print_error "✗ usbmon kernel module not loaded"
fi

print_status "Setup complete! 🎉"
print_warning "Please logout and login again (or reboot) for group permissions to take effect."
print_status "After relogin, you can run: ./identify_keyboard.sh"

echo
echo "Next steps:"
echo "1. Logout and login again (or run 'newgrp wireshark && newgrp plugdev')"
echo "2. Connect your Durgod K320 keyboard"
echo "3. Run ./identify_keyboard.sh to detect the device"
echo "4. Run ./capture_traffic.sh to begin packet capture" 