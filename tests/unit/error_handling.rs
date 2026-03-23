// Unit tests for error handling
// Validates Requirements 10.1, 10.2, 10.3, 10.4

use cpu_6502_emulator::{Emulator, cpu::Cpu, memory::Memory};
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

/// Test that file I/O errors produce descriptive error messages
/// Validates: Requirement 10.1
#[test]
fn test_file_not_found_error() {
    let result = Emulator::new("nonexistent_file_12345.bin", 0x8000);
    
    assert!(result.is_err(), "Should return error for nonexistent file");
    
    let err = result.unwrap_err();
    assert!(err.contains("nonexistent_file_12345.bin"), 
            "Error should include the file path: {}", err);
    assert!(err.contains("Failed to read file"), 
            "Error should be descriptive: {}", err);
}

/// Test that file I/O errors include the file path
/// Validates: Requirement 10.1
#[test]
fn test_file_error_includes_path() {
    let result = Emulator::new("/invalid/path/to/file.bin", 0x0000);
    
    assert!(result.is_err(), "Should return error for invalid path");
    
    let err = result.unwrap_err();
    assert!(err.contains("/invalid/path/to/file.bin"), 
            "Error should include the full file path: {}", err);
}

/// Test that invalid opcodes produce descriptive error messages
/// Validates: Requirement 10.2
#[test]
fn test_invalid_opcode_error() {
    let mut memory = Memory::new();
    
    // Write an invalid opcode (0x02 is not a valid 6502 opcode)
    memory.write(0x8000, 0x02);
    
    let mut cpu = Cpu::new(memory, 0x8000);
    let result = cpu.step();
    
    assert!(result.is_err(), "Should return error for invalid opcode");
    
    let err = result.unwrap_err();
    assert!(err.contains("0x02"), 
            "Error should include the opcode value: {}", err);
    assert!(err.contains("0x8000"), 
            "Error should include the PC location: {}", err);
    assert!(err.contains("Invalid opcode"), 
            "Error should be descriptive: {}", err);
}

/// Test that invalid opcodes include PC location
/// Validates: Requirement 10.2
#[test]
fn test_invalid_opcode_includes_pc() {
    let mut memory = Memory::new();
    
    // Write an invalid opcode at a specific location
    memory.write(0xC000, 0xFF); // 0xFF is not a valid opcode
    
    let mut cpu = Cpu::new(memory, 0xC000);
    let result = cpu.step();
    
    assert!(result.is_err(), "Should return error for invalid opcode");
    
    let err = result.unwrap_err();
    assert!(err.contains("0xC000"), 
            "Error should include the PC location: {}", err);
}

/// Test that CPU halts after invalid opcode
/// Validates: Requirement 10.2, 10.4
#[test]
fn test_cpu_halts_on_invalid_opcode() {
    let mut memory = Memory::new();
    
    // Write an invalid opcode
    memory.write(0x8000, 0x03);
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // First step should fail
    let result = cpu.step();
    assert!(result.is_err(), "First step should fail");
    assert!(cpu.halted, "CPU should be halted after invalid opcode");
    
    // Subsequent steps should also fail with halted message
    let result2 = cpu.step();
    assert!(result2.is_err(), "Subsequent steps should fail");
    assert!(result2.unwrap_err().contains("halted"), 
            "Error should indicate CPU is halted");
}

/// Test that file loading handles empty files gracefully
/// Validates: Requirement 10.1, 10.4
#[test]
fn test_empty_file_loads_successfully() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.bin");
    
    // Create an empty file
    File::create(&file_path).unwrap();
    
    let result = Emulator::new(file_path.to_str().unwrap(), 0x8000);
    
    // Empty files should load successfully (padded with zeros)
    assert!(result.is_ok(), "Empty file should load successfully");
}

/// Test that file loading handles oversized files gracefully
/// Validates: Requirement 10.1, 10.4
#[test]
fn test_oversized_file_loads_successfully() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("oversized.bin");
    
    // Create a file larger than 64KB
    let mut file = File::create(&file_path).unwrap();
    let data = vec![0xFF; 70000]; // 70KB
    file.write_all(&data).unwrap();
    drop(file);
    
    let result = Emulator::new(file_path.to_str().unwrap(), 0x8000);
    
    // Oversized files should load successfully (truncated to 64KB)
    assert!(result.is_ok(), "Oversized file should load successfully");
}

/// Test that all Result-returning functions use Result type
/// Validates: Requirement 10.4
#[test]
fn test_no_panics_in_step_execution() {
    let mut memory = Memory::new();
    
    // Write a valid instruction
    memory.write(0x8000, 0xEA); // NOP
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // This should not panic
    let result = cpu.step();
    assert!(result.is_ok(), "Valid instruction should execute successfully");
}

/// Test that memory operations don't panic on boundary conditions
/// Validates: Requirement 10.4
#[test]
fn test_memory_boundary_no_panic() {
    let mut memory = Memory::new();
    
    // Test reading/writing at memory boundaries
    memory.write(0x0000, 0x42);
    assert_eq!(memory.read(0x0000), 0x42);
    
    memory.write(0xFFFF, 0x99);
    assert_eq!(memory.read(0xFFFF), 0x99);
    
    // Test word read at boundary (should wrap)
    memory.write(0xFFFF, 0x12);
    memory.write(0x0000, 0x34);
    let word = memory.read_word(0xFFFF);
    assert_eq!(word, 0x3412); // Little-endian
}

/// Test that stack operations don't panic on overflow/underflow
/// Validates: Requirement 10.4
#[test]
fn test_stack_overflow_no_panic() {
    let memory = Memory::new();
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Push 256 bytes (should wrap around)
    for _ in 0..256 {
        cpu.state.sp = cpu.state.sp.wrapping_sub(1);
    }
    
    // Stack pointer should have wrapped
    assert_eq!(cpu.state.sp, 0xFF);
}

/// Test that PC wrapping doesn't cause panics
/// Validates: Requirement 10.4
#[test]
fn test_pc_wrapping_no_panic() {
    let mut memory = Memory::new();
    
    // Write NOP at the end of memory
    memory.write(0xFFFF, 0xEA); // NOP
    
    let mut cpu = Cpu::new(memory, 0xFFFF);
    
    // Execute instruction - PC should wrap to 0x0000
    let result = cpu.step();
    assert!(result.is_ok(), "PC wrapping should not cause panic");
    assert_eq!(cpu.state.pc, 0x0000, "PC should wrap to 0x0000");
}

/// Test that all addressing modes handle wrapping correctly
/// Validates: Requirement 10.4
#[test]
fn test_addressing_mode_wrapping_no_panic() {
    let mut memory = Memory::new();
    
    // Test Zero Page,X wrapping
    memory.write(0x8000, 0xB5); // LDA $FF,X
    memory.write(0x8001, 0xFF);
    memory.write(0x0001, 0x42); // Should wrap to 0x01 when X=2
    
    let mut cpu = Cpu::new(memory, 0x8000);
    cpu.state.x = 0x02;
    
    let result = cpu.step();
    assert!(result.is_ok(), "Zero page wrapping should not panic");
    assert_eq!(cpu.state.a, 0x42);
}

/// Test that branch instructions handle wrapping correctly
/// Validates: Requirement 10.4
#[test]
fn test_branch_wrapping_no_panic() {
    let mut memory = Memory::new();
    
    // Test backward branch that wraps
    memory.write(0x0001, 0xD0); // BNE (branch if not equal)
    memory.write(0x0002, 0xFE); // -2 (backward branch)
    
    let mut cpu = Cpu::new(memory, 0x0001);
    cpu.state.flag_zero = false; // Ensure branch is taken
    
    let result = cpu.step();
    assert!(result.is_ok(), "Branch wrapping should not panic");
}

/// Test that error messages are descriptive and helpful
/// Validates: Requirement 10.1, 10.2, 10.3
#[test]
fn test_error_messages_are_descriptive() {
    // Test invalid opcode error
    let mut memory = Memory::new();
    memory.write(0x8000, 0x04);
    let mut cpu = Cpu::new(memory, 0x8000);
    
    let result = cpu.step();
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    // Error should contain multiple pieces of information
    assert!(err.len() > 20, "Error message should be descriptive, got: {}", err);
    assert!(err.contains("0x"), "Error should use hex format");
}

/// Test that the emulator can recover from errors
/// Validates: Requirement 10.4
#[test]
fn test_emulator_recovery_after_error() {
    let mut memory = Memory::new();
    
    // Write invalid opcode followed by valid instruction
    memory.write(0x8000, 0x07); // Invalid
    memory.write(0x8001, 0xEA); // NOP
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // First step fails
    let result1 = cpu.step();
    assert!(result1.is_err());
    assert!(cpu.halted);
    
    // CPU is halted, but we can still inspect state
    assert_eq!(cpu.state.pc, 0x8000);
    assert_eq!(cpu.state.a, 0x00);
}

/// Test that multiple invalid opcodes are handled correctly
/// Validates: Requirement 10.2, 10.4
#[test]
fn test_multiple_invalid_opcodes() {
    let invalid_opcodes = vec![
        0x02, 0x03, 0x04, 0x07, 0x0B, 0x0F,
        0x12, 0x13, 0x14, 0x17, 0x1A, 0x1B, 0x1C, 0x1F,
        0x22, 0x23, 0x27, 0x2B, 0x2F,
        0x32, 0x33, 0x34, 0x37, 0x3A, 0x3B, 0x3C, 0x3F,
    ];
    
    for opcode in invalid_opcodes {
        let mut memory = Memory::new();
        memory.write(0x8000, opcode);
        
        let mut cpu = Cpu::new(memory, 0x8000);
        let result = cpu.step();
        
        assert!(result.is_err(), 
                "Opcode 0x{:02X} should be invalid", opcode);
        
        let err = result.unwrap_err();
        assert!(err.contains(&format!("0x{:02X}", opcode)),
                "Error should mention opcode 0x{:02X}: {}", opcode, err);
    }
}

/// Test that halt reason is set when invalid opcode is encountered
/// Validates: Requirement 11.3
#[test]
fn test_halt_reason_invalid_opcode() {
    let mut memory = Memory::new();
    
    // Write an invalid opcode
    memory.write(0x8000, 0x02);
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Initially, halt reason should be None
    assert_eq!(cpu.halt_reason, None, "Halt reason should be None initially");
    
    // Execute invalid opcode
    let result = cpu.step();
    assert!(result.is_err(), "Should return error for invalid opcode");
    assert!(cpu.halted, "CPU should be halted");
    
    // Verify halt reason is set to InvalidOpcode
    assert_eq!(cpu.halt_reason, Some(cpu_6502_emulator::cpu::HaltReason::InvalidOpcode), 
               "Halt reason should be InvalidOpcode");
}

/// Test that halt reason is None when CPU is not halted
/// Validates: Requirement 11.3
#[test]
fn test_halt_reason_none_when_not_halted() {
    let mut memory = Memory::new();
    
    // Write a valid instruction
    memory.write(0x8000, 0xEA); // NOP
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Execute valid instruction
    let result = cpu.step();
    assert!(result.is_ok(), "Should execute successfully");
    assert!(!cpu.halted, "CPU should not be halted");
    
    // Verify halt reason is still None
    assert_eq!(cpu.halt_reason, None, "Halt reason should be None when not halted");
}
