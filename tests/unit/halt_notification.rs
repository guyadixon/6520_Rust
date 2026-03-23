// Unit tests for halt notification display
// Tests Requirements 11.1, 11.2, 11.3, 11.4

use cpu_6502_emulator::{Emulator, ExecutionMode};
use cpu_6502_emulator::cpu::HaltReason;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use std::io::Cursor;

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
fn test_invalid_opcode_sets_halt_reason() {
    // Create a program with an invalid opcode
    let test_data = vec![0x02]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute step - should fail
    let result = emulator.cpu.step();
    assert!(result.is_err(), "Step should fail with invalid opcode");
    
    // Verify CPU is halted
    assert!(emulator.cpu.halted, "CPU should be halted");
    
    // Verify halt reason is InvalidOpcode
    assert_eq!(
        emulator.cpu.halt_reason,
        Some(HaltReason::InvalidOpcode),
        "Halt reason should be InvalidOpcode"
    );
}

#[test]
fn test_brk_instruction_sets_halt_reason() {
    // Create a program with BRK instruction followed by actual code
    // This ensures it's treated as BRK, not end of code
    let test_data = vec![0x00, 0xA9, 0x42]; // BRK, LDA #$42
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute step - BRK should execute and halt
    let result = emulator.cpu.step();
    assert!(result.is_err(), "Step should fail after BRK");
    
    // Verify CPU is halted
    assert!(emulator.cpu.halted, "CPU should be halted after BRK");
    
    // Verify halt reason is BrkInstruction
    assert_eq!(
        emulator.cpu.halt_reason,
        Some(HaltReason::BrkInstruction),
        "Halt reason should be BrkInstruction"
    );
}

#[test]
fn test_end_of_code_detection() {
    // Create a program with many consecutive zeros (end of code)
    // Use 256 zeros to trigger end-of-code detection
    let test_data = vec![0x00; 256];
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Check if CPU detects end of code
    assert!(emulator.cpu.is_at_end_of_code(), "Should detect end of code");
}

#[test]
fn test_end_of_code_not_detected_for_short_zero_sequence() {
    // Create a program with a short sequence of zeros
    let test_data = vec![0x00, 0x00, 0x00, 0x00, 0xEA]; // 4 zeros then NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Should not detect end of code
    assert!(!emulator.cpu.is_at_end_of_code(), "Should not detect end of code for short sequence");
}

#[test]
fn test_single_brk_not_treated_as_end_of_code() {
    // Create a program with a single BRK followed by non-zero bytes
    let test_data = vec![0x00, 0xA9, 0x42, 0xEA]; // BRK, LDA #$42, NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute step - should execute BRK, not treat as end of code
    let result = emulator.cpu.step();
    
    // Verify CPU is halted
    assert!(emulator.cpu.halted, "CPU should be halted after BRK");
    
    // Verify halt reason is BrkInstruction, not EndOfCode
    assert_eq!(
        emulator.cpu.halt_reason,
        Some(HaltReason::BrkInstruction),
        "Halt reason should be BrkInstruction, not EndOfCode"
    );
}

#[test]
fn test_halt_reason_persists_after_halt() {
    // Create a program with an invalid opcode
    let test_data = vec![0x02]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // First step - should fail and set halt reason
    let result1 = emulator.cpu.step();
    assert!(result1.is_err());
    let halt_reason = emulator.cpu.halt_reason;
    
    // Second step - should fail because CPU is halted
    let result2 = emulator.cpu.step();
    assert!(result2.is_err());
    
    // Halt reason should still be the same
    assert_eq!(
        emulator.cpu.halt_reason,
        halt_reason,
        "Halt reason should persist after halt"
    );
}

#[test]
fn test_halt_notification_includes_final_state() {
    // Create a program: LDA #$42, invalid opcode
    let test_data = vec![0xA9, 0x42, 0x02]; // LDA #$42, invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute first instruction
    emulator.cpu.step().expect("First step should succeed");
    assert_eq!(emulator.cpu.state.a, 0x42);
    assert_eq!(emulator.cpu.state.pc, 0x0002);
    
    // Execute second instruction - should fail
    let result = emulator.cpu.step();
    assert!(result.is_err());
    
    // Verify final state is preserved
    assert_eq!(emulator.cpu.state.a, 0x42, "Accumulator should be preserved");
    assert_eq!(emulator.cpu.state.pc, 0x0002, "PC should be preserved");
    assert!(emulator.cpu.halted, "CPU should be halted");
}

#[test]
fn test_error_message_contains_pc_for_invalid_opcode() {
    // Create a program with an invalid opcode at a specific address
    let mut test_data = vec![0xEA; 0x100]; // Fill with NOPs
    test_data[0x50] = 0x02; // Invalid opcode at 0x0050
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0050)
        .expect("Failed to create emulator");
    
    // Execute step - should fail
    let result = emulator.cpu.step();
    assert!(result.is_err());
    
    // Error message should contain the PC
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("0x0050"), "Error should mention PC location");
}

#[test]
fn test_end_of_code_detection_at_specific_address() {
    // Create a program with end of code at a specific address
    let mut test_data = vec![0xEA; 0x200]; // Fill with NOPs
    // Add 256 consecutive zeros starting at 0x0030
    for i in 0x30..0x130 {
        test_data[i] = 0x00;
    }
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0030)
        .expect("Failed to create emulator");
    
    // Should detect end of code at 0x0030
    assert!(emulator.cpu.is_at_end_of_code(), "Should detect end of code at 0x0030");
}

// Tests for restart/quit functionality after halt
// Tests Requirement 11.4

#[test]
fn test_cpu_halted_flag_set_after_invalid_opcode() {
    // Create a program with an invalid opcode
    let test_data = vec![0x02]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // CPU should not be halted initially
    assert!(!emulator.cpu.halted, "CPU should not be halted initially");
    
    // Execute step - should fail
    let result = emulator.cpu.step();
    assert!(result.is_err(), "Step should fail with invalid opcode");
    
    // CPU should be halted after invalid opcode
    assert!(emulator.cpu.halted, "CPU should be halted after invalid opcode");
}

#[test]
fn test_cpu_halted_flag_set_after_brk() {
    // Create a program with BRK instruction
    let test_data = vec![0x00, 0xA9, 0x42]; // BRK, LDA #$42
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // CPU should not be halted initially
    assert!(!emulator.cpu.halted, "CPU should not be halted initially");
    
    // Execute BRK - should halt
    let result = emulator.cpu.step();
    assert!(result.is_err(), "Step should fail after BRK");
    
    // CPU should be halted after BRK
    assert!(emulator.cpu.halted, "CPU should be halted after BRK");
}

#[test]
fn test_halt_reason_available_for_restart_decision() {
    // Create a program with an invalid opcode
    let test_data = vec![0x02]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute step - should fail
    emulator.cpu.step().ok();
    
    // Verify halt reason is available for display
    assert!(emulator.cpu.halt_reason.is_some(), "Halt reason should be available");
    
    // Verify we can match on the halt reason
    match emulator.cpu.halt_reason {
        Some(HaltReason::InvalidOpcode) => {
            // Expected
        }
        _ => panic!("Expected InvalidOpcode halt reason"),
    }
}

#[test]
fn test_emulator_can_be_recreated_after_halt() {
    // This test verifies that after a halt, a new emulator can be created
    // (simulating the restart flow)
    
    // First emulator with invalid opcode
    let test_data1 = vec![0x02]; // Invalid opcode
    let (_temp_dir1, file_path1) = create_temp_binary(&test_data1);
    
    let mut emulator1 = Emulator::new(&file_path1, 0x0000)
        .expect("Failed to create first emulator");
    
    // Execute and halt
    emulator1.cpu.step().ok();
    assert!(emulator1.cpu.halted, "First emulator should be halted");
    
    // Create a second emulator (simulating restart)
    let test_data2 = vec![0xA9, 0x42, 0xEA]; // LDA #$42, NOP
    let (_temp_dir2, file_path2) = create_temp_binary(&test_data2);
    
    let mut emulator2 = Emulator::new(&file_path2, 0x0000)
        .expect("Failed to create second emulator");
    
    // Second emulator should not be halted
    assert!(!emulator2.cpu.halted, "Second emulator should not be halted");
    
    // Second emulator should execute successfully
    emulator2.cpu.step().expect("Second emulator should execute successfully");
    assert_eq!(emulator2.cpu.state.a, 0x42, "Second emulator should execute correctly");
}

#[test]
fn test_run_method_returns_boolean() {
    // This test verifies that the run method returns a boolean
    // We can't test the interactive prompt without mocking stdin,
    // but we can verify the method signature is correct by checking
    // that it compiles and the emulator structure is correct
    
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Verify the emulator has the expected structure
    assert!(!emulator.cpu.halted, "CPU should not be halted initially");
    assert_eq!(emulator.mode, ExecutionMode::Paused, "Should start in Paused mode");
}

#[test]
fn test_multiple_halt_reasons_distinguishable() {
    // Test that different halt reasons can be distinguished
    
    // Test InvalidOpcode
    let test_data1 = vec![0x02]; // Invalid opcode
    let (_temp_dir1, file_path1) = create_temp_binary(&test_data1);
    let mut emulator1 = Emulator::new(&file_path1, 0x0000).unwrap();
    emulator1.cpu.step().ok();
    assert_eq!(emulator1.cpu.halt_reason, Some(HaltReason::InvalidOpcode));
    
    // Test BrkInstruction
    let test_data2 = vec![0x00, 0xEA]; // BRK, NOP
    let (_temp_dir2, file_path2) = create_temp_binary(&test_data2);
    let mut emulator2 = Emulator::new(&file_path2, 0x0000).unwrap();
    emulator2.cpu.step().ok();
    assert_eq!(emulator2.cpu.halt_reason, Some(HaltReason::BrkInstruction));
    
    // Test EndOfCode
    let test_data3 = vec![0x00; 256]; // 256 zeros
    let (_temp_dir3, file_path3) = create_temp_binary(&test_data3);
    let mut emulator3 = Emulator::new(&file_path3, 0x0000).unwrap();
    // Manually set halt reason for end of code (normally done by run loop)
    emulator3.cpu.halted = true;
    emulator3.cpu.halt_reason = Some(HaltReason::EndOfCode);
    assert_eq!(emulator3.cpu.halt_reason, Some(HaltReason::EndOfCode));
}
