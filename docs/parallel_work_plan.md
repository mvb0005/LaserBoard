# Durgod K320 Reverse Engineering - Parallel Work Plan

**Date**: December 26, 2024  
**Status**: Ready for parallel execution  
**Critical Path**: Compilation fixes → Device identification → Protocol analysis

## 🎯 Work Streams Overview

### Stream A: **Rust Compilation & API Fixes** (PRIORITY 1)
**Worker Type**: Rust Developer  
**Estimated Time**: 30-60 minutes  
**Dependencies**: None  
**Blocking**: All other Rust-based work

### Stream B: **Hardware Identification & Setup** (PRIORITY 1)  
**Worker Type**: Hardware/System Admin  
**Estimated Time**: 15-30 minutes  
**Dependencies**: None  
**Blocking**: Packet capture work

### Stream C: **USB Traffic Capture & Analysis** (PRIORITY 2)
**Worker Type**: Network/Protocol Analyst  
**Estimated Time**: 1-2 hours  
**Dependencies**: Stream A (compilation), Stream B (hardware)  
**Blocking**: Protocol implementation

### Stream D: **Protocol Implementation & Testing** (PRIORITY 3)
**Worker Type**: Rust Developer + Protocol Specialist  
**Estimated Time**: 2-4 hours  
**Dependencies**: Stream A, Stream C  
**Blocking**: Final validation

---

## 📋 Detailed Worker Prompts

### **Worker A: Rust Compilation Specialist**

**PROMPT:**
```
You are working on a Rust project for USB HID keyboard reverse engineering. The project has compilation errors with the hidapi crate (v2.6.3). 

Current errors:
1. hidapi::HidError::FromWideCharError field mismatch (wide_char_error vs wide_char)
2. Type annotation issues in device_scanner.rs
3. String handling issues with hidapi API changes

Files to fix:
- src/errors.rs (line 66)
- src/device_scanner.rs (multiple string handling issues)
- src/lib.rs (unused imports)

Requirements:
- Fix all compilation errors
- Ensure hidapi v2.6.3 compatibility
- Maintain error handling quality
- Test compilation with `cargo check`

Deliverables:
- Fixed source files
- Confirmation that `cargo check` passes
- Brief summary of API changes made

Repository: /home/MVB/LaserBoard
```

**Success Criteria:**
- [ ] `cargo check` completes without errors
- [ ] All hidapi API calls are compatible with v2.6.3
- [ ] No warnings about unused imports
- [ ] Error handling remains robust

---

### **Worker B: Hardware Setup & Device Identification**

**PROMPT:**
```
You are setting up hardware analysis for a Durgod K320 mechanical keyboard reverse engineering project.

Tasks:
1. Connect Durgod K320 keyboard to Linux system
2. Run device identification scripts
3. Verify USB permissions and udev rules
4. Document device VID:PID and interface details

Commands to run:
```bash
cd /home/MVB/LaserBoard
./scripts/identify_keyboard.sh
lsusb | grep -i keyboard
lsusb -v -s <BUS>:<DEVICE>  # For detailed device info
```

Requirements:
- Identify exact VID:PID of Durgod K320
- Document USB interface details
- Test HID device access permissions
- Verify usbmon module is loaded

Deliverables:
- Device VID:PID numbers
- USB interface configuration
- Permission status (works with/without sudo)
- Device path and HID interface numbers

Repository: /home/MVB/LaserBoard
```

**Success Criteria:**
- [ ] Durgod K320 is detected and identified
- [ ] VID:PID documented
- [ ] USB permissions verified
- [ ] Device accessible for packet capture

---

### **Worker C: USB Traffic Capture & Protocol Analysis**

**PROMPT:**
```
You are capturing and analyzing USB HID traffic from a Durgod K320 keyboard to reverse engineer its protocol.

Prerequisites (from other workers):
- Device VID:PID from Worker B
- Working Rust compilation from Worker A

Tasks:
1. Capture baseline USB traffic (no key presses)
2. Capture single key press events (A-Z, 0-9, special keys)
3. Capture key combinations (Ctrl+C, Shift+A, etc.)
4. Analyze HID report structure and timing
5. Document key code mappings

Commands to run:
```bash
cd /home/MVB/LaserBoard
# Replace VID:PID with actual values from Worker B
./scripts/capture_traffic.sh -v <VID> -p <PID> -d 30
./scripts/analyze_packets.sh captures/durgod_k320_*.pcap
```

Capture scenarios:
- 30 seconds idle (baseline)
- Individual key presses (A, B, C, 1, 2, 3, Enter, Space, etc.)
- Modifier combinations (Ctrl+A, Shift+B, Alt+Tab)
- Rapid key sequences (typing "hello world")

Deliverables:
- Packet capture files (.pcap)
- Analysis reports (.txt, .md)
- Key code mapping table
- HID report structure documentation
- Timing analysis

Repository: /home/MVB/LaserBoard
```

**Success Criteria:**
- [ ] Multiple packet capture files created
- [ ] HID report structure documented
- [ ] Key code mappings identified
- [ ] Analysis reports generated
- [ ] Protocol timing documented

---

### **Worker D: Protocol Implementation & Testing**

**PROMPT:**
```
You are implementing the USB HID protocol for the Durgod K320 keyboard based on analysis from Worker C.

Prerequisites:
- Working Rust compilation (Worker A)
- Protocol analysis results (Worker C)

Tasks:
1. Implement HID report generation based on captured data
2. Create key press injection functionality
3. Add text typing capabilities
4. Implement error handling and validation
5. Test with real device

Implementation focus:
- HIDKeyboardReport structure (8-byte format)
- Key code to scan code mapping
- Modifier key handling
- Timing and release events
- Error handling for device communication

Test scenarios:
- Single key press (A, Enter, Space)
- Key combinations (Ctrl+C, Shift+A)
- Text typing ("Hello World")
- Error handling (device not found, permission denied)

Deliverables:
- Working key injection code
- Text typing functionality
- Error handling improvements
- Test results and validation

Repository: /home/MVB/LaserBoard
```

**Success Criteria:**
- [ ] Key press injection works
- [ ] Text typing functionality implemented
- [ ] Error handling robust
- [ ] Tests pass with real device
- [ ] Documentation updated

---

## 🔄 Coordination Points

### **Handoff Requirements**

**Worker A → Worker C:**
- Compilation fixes complete
- `cargo check` passes
- hidapi compatibility confirmed

**Worker B → Worker C:**
- Device VID:PID documented
- USB permissions verified
- Device accessible for capture

**Worker C → Worker D:**
- Protocol analysis complete
- Key code mappings documented
- HID report structure understood

### **Parallel Work Opportunities**

**Workers A & B can work simultaneously:**
- No dependencies between compilation fixes and hardware setup
- Both are prerequisites for Worker C

**Worker D can start partial work after Worker A:**
- Can implement basic structures while waiting for protocol analysis
- Can prepare test frameworks

---

## 📊 Progress Tracking

### **Stream A (Rust Compilation)**
- [ ] Fix hidapi API compatibility issues
- [ ] Resolve type annotation problems
- [ ] Clean up unused imports
- [ ] Verify `cargo check` passes

### **Stream B (Hardware Setup)**
- [ ] Connect Durgod K320
- [ ] Run device identification
- [ ] Document VID:PID
- [ ] Verify permissions

### **Stream C (Traffic Analysis)**
- [ ] Capture baseline traffic
- [ ] Capture key press events
- [ ] Analyze HID reports
- [ ] Document protocol structure

### **Stream D (Implementation)**
- [ ] Implement HID report generation
- [ ] Add key injection functionality
- [ ] Create text typing features
- [ ] Test with real device

---

## 🚨 Risk Mitigation

### **Blocking Issues**
- **Compilation failures**: Worker A must complete before others can test
- **Hardware not detected**: Worker B must verify device connectivity
- **Permission issues**: May need sudo or udev rule adjustments

### **Fallback Plans**
- **If hidapi issues persist**: Consider alternative USB libraries (rusb)
- **If device not found**: Manual USB device enumeration
- **If permissions fail**: Temporary sudo usage with proper documentation

---

## 📞 Communication Protocol

### **Status Updates**
Each worker should provide:
1. **Start confirmation**: "Worker X starting on [task]"
2. **Progress updates**: "Worker X: [specific milestone] completed"
3. **Blocking issues**: "Worker X blocked on [issue]"
4. **Completion**: "Worker X: [deliverables] ready"

### **Handoff Format**
```
WORKER X COMPLETE
================
Deliverables:
- [list of files/changes]

Blocking issues resolved:
- [list of fixes]

Next worker dependencies:
- [what Worker Y needs from this work]
```

---

**Note**: This plan assumes all workers have access to the same repository and can coordinate through shared documentation. Each worker should update the project context file with their findings. 