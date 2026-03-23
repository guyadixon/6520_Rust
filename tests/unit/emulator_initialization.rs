// Unit tests for Emulator initialization and execution control
// Tests Requirements 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2

use cpu_6502_emulator::{Emulator, ExecutionMode};
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

/// Helper function to create a temporary binary file with specified content
fn create_temp_binary(content: &[u8]) -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.bin");
    let mut file = File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content).expect("Failed to write to temp file");
    let path_str = file_path.to_str().unwrap().to_string();
    (temp_dir, path_str)
}

#[test]
fn test_emulator_new_with_valid_file() {
    // Create a small test binary
    let test_data = vec![0xA9, 0x42, 0x00]; // LDA #$42, BRK
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    // Create emulator with start address 0x8000
    let result = Emulator::new(&file_path, 0x8000);
    
    assert!(result.is_ok(), "Emulator creation should succeed");
    let emulator = result.unwrap();
    
    // Verify CPU is initialized with correct start address
    assert_eq!(emulator.cpu.state.pc, 0x8000, "PC should be set to start address");
    
    // Verify CPU registers are initialized to zero
    assert_eq!(emulator.cpu.state.a, 0x00, "Accumulator should be 0");
    assert_eq!(emulator.cpu.state.x, 0x00, "X register should be 0");
    assert_eq!(emulator.cpu.state.y, 0x00, "Y register should be 0");
    assert_eq!(emulator.cpu.state.sp, 0xFF, "Stack pointer should be 0xFF");
    
    // Verify emulator starts in Paused mode
    assert_eq!(emulator.mode, ExecutionMode::Paused, "Emulator should start in Paused mode");
    
    // Verify memory was loaded correctly
    assert_eq!(emulator.cpu.memory.read(0x0000), 0xA9, "First byte should be loaded");
    assert_eq!(emulator.cpu.memory.read(0x0001), 0x42, "Second byte should be loaded");
    assert_eq!(emulator.cpu.memory.read(0x0002), 0x00, "Third byte should be loaded");
}

#[test]
fn test_emulator_new_with_different_start_addresses() {
    // Test various start addresses to verify PC initialization
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let test_addresses = [0x0000, 0x0100, 0x8000, 0xC000, 0xFFFF];
    
    for &start_addr in &test_addresses {
        let emulator = Emulator::new(&file_path, start_addr)
            .expect(&format!("Failed to create emulator with start address 0x{:04X}", start_addr));
        
        assert_eq!(
            emulator.cpu.state.pc, start_addr,
            "PC should be set to start address 0x{:04X}", start_addr
        );
    }
}

#[test]
fn test_emulator_new_with_small_file() {
    // Create a file smaller than 64KB (should be padded with zeros)
    let test_data = vec![0xFF; 100]; // 100 bytes of 0xFF
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator with small file");
    
    // Verify the first 100 bytes are 0xFF
    for i in 0..100 {
        assert_eq!(
            emulator.cpu.memory.read(i),
            0xFF,
            "Byte {} should be 0xFF", i
        );
    }
    
    // Verify the rest is padded with zeros
    for i in 100..200 {
        assert_eq!(
            emulator.cpu.memory.read(i),
            0x00,
            "Byte {} should be padded with 0x00", i
        );
    }
}

#[test]
fn test_emulator_new_with_exact_64kb_file() {
    // Create exactly 64KB file
    let test_data = vec![0xAA; 65536]; // 64KB of 0xAA
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator with 64KB file");
    
    // Verify all bytes are loaded correctly
    assert_eq!(emulator.cpu.memory.read(0x0000), 0xAA);
    assert_eq!(emulator.cpu.memory.read(0x8000), 0xAA);
    assert_eq!(emulator.cpu.memory.read(0xFFFF), 0xAA);
}

#[test]
fn test_emulator_new_with_oversized_file() {
    // Create a file larger than 64KB (should be truncated)
    let test_data = vec![0xBB; 70000]; // 70KB of 0xBB
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator with oversized file");
    
    // Verify first 64KB is loaded
    assert_eq!(emulator.cpu.memory.read(0x0000), 0xBB);
    assert_eq!(emulator.cpu.memory.read(0xFFFF), 0xBB);
    
    // Memory should only contain 64KB (the extra bytes are truncated)
    // This is implicitly tested by the memory size being fixed at 64KB
}

#[test]
fn test_emulator_new_with_invalid_file() {
    // Try to create emulator with non-existent file
    let result = Emulator::new("/nonexistent/path/to/file.bin", 0x8000);
    
    assert!(result.is_err(), "Emulator creation should fail with invalid file");
    
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Failed to open file") || error_msg.contains("No such file"),
        "Error message should indicate file not found: {}", error_msg
    );
}

#[test]
fn test_emulator_execution_mode_initial_state() {
    // Verify emulator always starts in Paused mode
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    assert_eq!(emulator.mode, ExecutionMode::Paused);
}

#[test]
fn test_emulator_cpu_not_halted_initially() {
    // Verify CPU is not halted when emulator is created
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    assert!(!emulator.cpu.halted, "CPU should not be halted initially");
}

#[test]
fn test_emulator_all_flags_cleared_initially() {
    // Verify all CPU flags are cleared on initialization
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    assert!(!emulator.cpu.state.flag_carry, "Carry flag should be clear");
    assert!(!emulator.cpu.state.flag_zero, "Zero flag should be clear");
    assert!(!emulator.cpu.state.flag_interrupt_disable, "Interrupt disable flag should be clear");
    assert!(!emulator.cpu.state.flag_decimal, "Decimal flag should be clear");
    assert!(!emulator.cpu.state.flag_break, "Break flag should be clear");
    assert!(!emulator.cpu.state.flag_overflow, "Overflow flag should be clear");
    assert!(!emulator.cpu.state.flag_negative, "Negative flag should be clear");
}
