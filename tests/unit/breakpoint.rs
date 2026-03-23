// Unit tests for breakpoint functionality
// Validates: Requirements 14.1-14.12

use cpu_6502_emulator::Emulator;
use std::fs::File;
use std::io::Write;

/// Helper function to create a test binary file
fn create_test_binary(path: &str, data: &[u8]) {
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
}

#[test]
fn test_breakpoint_initialization() {
    // Test that breakpoint field initializes to None
    // Validates: Requirements 14.2, 14.3
    
    let test_file = "test_breakpoint_init.bin";
    create_test_binary(test_file, &[0xEA, 0xEA, 0xEA]); // NOP instructions
    
    let emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    assert_eq!(emulator.breakpoint, None, "Breakpoint should initialize to None");
    
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_set_breakpoint() {
    // Test that setting breakpoint updates field correctly
    // Validates: Requirements 14.1, 14.2
    
    let test_file = "test_set_breakpoint.bin";
    create_test_binary(test_file, &[0xEA, 0xEA, 0xEA]);
    
    let mut emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    // Set breakpoint
    emulator.breakpoint = Some(0x0005);
    
    assert_eq!(emulator.breakpoint, Some(0x0005), "Breakpoint should be set to 0x0005");
    
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_check_breakpoint_when_pc_matches() {
    // Test check_breakpoint() returns true when PC matches breakpoint
    // Validates: Requirements 14.4
    
    let test_file = "test_breakpoint_match.bin";
    create_test_binary(test_file, &[0xEA, 0xEA, 0xEA]);
    
    let mut emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    // Set breakpoint at current PC
    emulator.breakpoint = Some(0x0000);
    
    assert!(emulator.check_breakpoint(), "check_breakpoint() should return true when PC matches");
    
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_check_breakpoint_when_pc_does_not_match() {
    // Test check_breakpoint() returns false when PC doesn't match breakpoint
    // Validates: Requirements 14.4
    
    let test_file = "test_breakpoint_no_match.bin";
    create_test_binary(test_file, &[0xEA, 0xEA, 0xEA]);
    
    let mut emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    // Set breakpoint at different address
    emulator.breakpoint = Some(0x0005);
    
    assert!(!emulator.check_breakpoint(), "check_breakpoint() should return false when PC doesn't match");
    
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_check_breakpoint_when_no_breakpoint_set() {
    // Test check_breakpoint() returns false when no breakpoint is set
    // Validates: Requirements 14.4
    
    let test_file = "test_no_breakpoint.bin";
    create_test_binary(test_file, &[0xEA, 0xEA, 0xEA]);
    
    let emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    assert!(!emulator.check_breakpoint(), "check_breakpoint() should return false when no breakpoint set");
    
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_replace_existing_breakpoint() {
    // Test replacing existing breakpoint with new address
    // Validates: Requirements 14.1, 14.2
    
    let test_file = "test_replace_breakpoint.bin";
    create_test_binary(test_file, &[0xEA, 0xEA, 0xEA]);
    
    let mut emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    // Set initial breakpoint
    emulator.breakpoint = Some(0x0005);
    assert_eq!(emulator.breakpoint, Some(0x0005));
    
    // Replace with new breakpoint
    emulator.breakpoint = Some(0x0010);
    assert_eq!(emulator.breakpoint, Some(0x0010), "Breakpoint should be updated to new address");
    
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_breakpoint_at_current_pc_triggers_on_next_occurrence() {
    // Test that breakpoint at same address as current PC triggers on next occurrence
    // Validates: Requirements 14.4, 14.12
    
    let test_file = "test_breakpoint_next_occurrence.bin";
    // Create a simple loop: JMP 0x0000 (4C 00 00)
    create_test_binary(test_file, &[0x4C, 0x00, 0x00]);
    
    let mut emulator = Emulator::new(test_file, 0x0000).unwrap();
    
    // Set breakpoint at start address
    emulator.breakpoint = Some(0x0000);
    
    // First check - should match
    assert!(emulator.check_breakpoint(), "Breakpoint should match at PC=0x0000");
    
    // Execute one instruction (JMP 0x0000)
    emulator.cpu.step().unwrap();
    
    // PC should be back at 0x0000
    assert_eq!(emulator.cpu.state.pc, 0x0000);
    
    // Check again - should still match
    assert!(emulator.check_breakpoint(), "Breakpoint should match again after loop");
    
    std::fs::remove_file(test_file).ok();
}
