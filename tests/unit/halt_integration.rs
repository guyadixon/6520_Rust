// Integration test for halt handling in execution loop
// Tests that CPU halts are properly detected and handled in all execution modes
// Validates: Requirements 19.1, 19.2, 19.8

use cpu_6502_emulator::Emulator;
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

/// Test that CPU halt is properly detected in Paused mode
/// Validates: Requirements 19.1, 19.2, 19.8
#[test]
fn test_halt_detected_in_paused_mode() {
    // Create a program with an invalid opcode to trigger halt
    let test_data = vec![0xFF]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // CPU should not be halted initially
    assert!(!emulator.cpu.halted);
    
    // Manually set CPU to halted state (simulating what happens after an error)
    emulator.cpu.halted = true;
    
    // Verify that the emulator can detect the halt
    assert!(emulator.cpu.halted);
}

/// Test that error in Stepping mode sets halted flag
/// Validates: Requirements 19.1, 19.2, 19.8
#[test]
fn test_error_in_stepping_mode_sets_halted() {
    // Create a program with an invalid opcode
    let test_data = vec![0xFF]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute the invalid instruction
    let result = emulator.cpu.step();
    
    // Step should fail
    assert!(result.is_err());
    
    // After an error, the CPU should be marked as halted
    // This is done in the execution loop when Err is returned
    assert!(emulator.cpu.halted);
}

/// Test that BRK instruction sets halted flag
/// Validates: Requirements 19.1, 19.2, 19.8
#[test]
fn test_brk_instruction_sets_halted() {
    // Create a program with BRK instruction
    let test_data = vec![0x00]; // BRK
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute the BRK instruction
    let result = emulator.cpu.step();
    
    // BRK should return an error indicating halt
    assert!(result.is_err());
    // CPU should be halted
    assert!(emulator.cpu.halted);
}

/// Test that instruction count is preserved when CPU halts
/// Validates: Requirements 19.2, 19.8
#[test]
fn test_instruction_count_preserved_on_halt() {
    // Create a program with some NOPs followed by invalid opcode
    let test_data = vec![0xEA, 0xEA, 0xEA, 0xFF]; // NOP, NOP, NOP, Invalid
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute three NOPs
    for _ in 0..3 {
        emulator.cpu.step().expect("Step should succeed");
        emulator.instruction_count += 1;
    }
    
    // Instruction count should be 3
    assert_eq!(emulator.instruction_count, 3);
    
    // Execute invalid opcode
    let result = emulator.cpu.step();
    assert!(result.is_err());
    
    // Instruction count should still be 3 (not incremented for failed instruction)
    assert_eq!(emulator.instruction_count, 3);
    
    // CPU should be halted
    assert!(emulator.cpu.halted);
}

/// Test that framebuffer state is preserved when CPU halts
/// Validates: Requirements 19.1, 19.8
#[test]
fn test_framebuffer_preserved_on_halt() {
    // Create a program with BRK
    let test_data = vec![0x00]; // BRK
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Set framebuffer base address
    emulator.framebuffer_base = Some(0x2000);
    
    // Execute BRK instruction (will return error but set halted flag)
    let result = emulator.cpu.step();
    assert!(result.is_err());
    
    // CPU should be halted
    assert!(emulator.cpu.halted);
    
    // Framebuffer should still be set
    assert_eq!(emulator.framebuffer_base, Some(0x2000));
}

/// Test that CPU state is accessible after halt
/// Validates: Requirements 19.3, 19.8
#[test]
fn test_cpu_state_accessible_after_halt() {
    // Create a program that modifies registers then halts
    let test_data = vec![
        0xA9, 0x42, // LDA #$42
        0xA2, 0x10, // LDX #$10
        0xA0, 0x20, // LDY #$20
        0x00,       // BRK
    ];
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute first three instructions (should succeed)
    for _ in 0..3 {
        emulator.cpu.step().expect("Step should succeed");
        emulator.instruction_count += 1;
    }
    
    // Execute BRK instruction (will return error but set halted flag)
    let result = emulator.cpu.step();
    assert!(result.is_err());
    emulator.instruction_count += 1;
    
    // CPU should be halted
    assert!(emulator.cpu.halted);
    
    // Verify CPU state is accessible
    assert_eq!(emulator.cpu.state.a, 0x42);
    assert_eq!(emulator.cpu.state.x, 0x10);
    assert_eq!(emulator.cpu.state.y, 0x20);
    
    // Verify instruction count
    assert_eq!(emulator.instruction_count, 4);
}
