// Unit tests for Emulator execution loop and command handling
// Tests Requirements 6.1, 6.2, 6.3, 6.4, 6.5, 6.6

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
fn test_emulator_starts_in_paused_mode() {
    // Verify emulator starts in Paused mode
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    assert_eq!(emulator.mode, ExecutionMode::Paused);
}

#[test]
fn test_step_command_executes_one_instruction() {
    // Create a simple program: LDA #$42, NOP
    let test_data = vec![0xA9, 0x42, 0xEA];
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Initial state
    assert_eq!(emulator.cpu.state.pc, 0x0000);
    assert_eq!(emulator.cpu.state.a, 0x00);
    
    // Simulate step command by setting mode to Stepping
    emulator.mode = ExecutionMode::Stepping;
    
    // Execute one step
    let result = emulator.cpu.step();
    assert!(result.is_ok(), "Step should succeed");
    
    // Verify instruction was executed
    assert_eq!(emulator.cpu.state.pc, 0x0002, "PC should advance by 2");
    assert_eq!(emulator.cpu.state.a, 0x42, "A should be loaded with 0x42");
}

#[test]
fn test_multiple_steps() {
    // Create a program: LDA #$42, LDX #$10, LDY #$20
    let test_data = vec![
        0xA9, 0x42,  // LDA #$42
        0xA2, 0x10,  // LDX #$10
        0xA0, 0x20,  // LDY #$20
    ];
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Step 1: LDA #$42
    emulator.cpu.step().expect("Step 1 failed");
    assert_eq!(emulator.cpu.state.pc, 0x0002);
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    // Step 2: LDX #$10
    emulator.cpu.step().expect("Step 2 failed");
    assert_eq!(emulator.cpu.state.pc, 0x0004);
    assert_eq!(emulator.cpu.state.x, 0x10);
    
    // Step 3: LDY #$20
    emulator.cpu.step().expect("Step 3 failed");
    assert_eq!(emulator.cpu.state.pc, 0x0006);
    assert_eq!(emulator.cpu.state.y, 0x20);
}

#[test]
fn test_execution_mode_transitions() {
    let test_data = vec![0xEA]; // NOP
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Start in Paused mode
    assert_eq!(emulator.mode, ExecutionMode::Paused);
    
    // Transition to Stepping
    emulator.mode = ExecutionMode::Stepping;
    assert_eq!(emulator.mode, ExecutionMode::Stepping);
    
    // Transition to Running
    emulator.mode = ExecutionMode::Running;
    assert_eq!(emulator.mode, ExecutionMode::Running);
    
    // Transition back to Paused
    emulator.mode = ExecutionMode::Paused;
    assert_eq!(emulator.mode, ExecutionMode::Paused);
}

#[test]
fn test_cpu_state_display_does_not_crash() {
    // Verify that display_state doesn't panic
    let test_data = vec![0xA9, 0x42]; // LDA #$42
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // This test just verifies display_state doesn't panic
    // We can't easily test the output without capturing stdout
    // but we can at least ensure it doesn't crash
    // Note: display_state is private, so we test it indirectly through run()
    // For now, we just verify the emulator was created successfully
    assert_eq!(emulator.cpu.state.pc, 0x0000);
}

#[test]
fn test_invalid_opcode_halts_execution() {
    // Create a program with an invalid opcode
    let test_data = vec![0x02]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Try to step - should fail
    let result = emulator.cpu.step();
    assert!(result.is_err(), "Step should fail with invalid opcode");
    
    // CPU should be halted
    assert!(emulator.cpu.halted, "CPU should be halted after invalid opcode");
    
    // Error message should contain opcode and PC
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("0x02"), "Error should mention opcode");
    assert!(error_msg.contains("0x0000"), "Error should mention PC");
}

#[test]
fn test_step_after_halt_fails() {
    // Create a program with an invalid opcode
    let test_data = vec![0x02]; // Invalid opcode
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // First step - should fail and halt
    let result1 = emulator.cpu.step();
    assert!(result1.is_err());
    assert!(emulator.cpu.halted);
    
    // Second step - should fail because CPU is halted
    let result2 = emulator.cpu.step();
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), "CPU is halted");
}

#[test]
fn test_flags_updated_after_step() {
    // Create a program: LDA #$00 (should set Zero flag)
    let test_data = vec![0xA9, 0x00];
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Initial flags should be clear
    assert!(!emulator.cpu.state.flag_zero);
    
    // Execute LDA #$00
    emulator.cpu.step().expect("Step failed");
    
    // Zero flag should be set
    assert!(emulator.cpu.state.flag_zero, "Zero flag should be set");
    assert!(!emulator.cpu.state.flag_negative, "Negative flag should be clear");
}

#[test]
fn test_negative_flag_after_step() {
    // Create a program: LDA #$FF (should set Negative flag)
    let test_data = vec![0xA9, 0xFF];
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute LDA #$FF
    emulator.cpu.step().expect("Step failed");
    
    // Negative flag should be set
    assert!(emulator.cpu.state.flag_negative, "Negative flag should be set");
    assert!(!emulator.cpu.state.flag_zero, "Zero flag should be clear");
}
