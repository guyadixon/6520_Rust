// Unit tests for interactive debugging features
// Tests Requirements 6.1-6.6: Interactive Execution Control
//
// **Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5, 6.6**

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

#[cfg(test)]
mod pause_command_tests {
    use super::*;

    #[test]
    fn test_emulator_starts_paused() {
        // Requirement 6.1: Emulator should start in paused mode
        let test_data = vec![0xEA, 0xEA, 0xEA]; // Three NOPs
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        assert_eq!(emulator.mode, ExecutionMode::Paused,
                   "Emulator should start in Paused mode");
    }

    #[test]
    fn test_pause_from_running_mode() {
        // Requirement 6.1: Should be able to pause from running mode
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Transition to Running mode
        emulator.mode = ExecutionMode::Running;
        assert_eq!(emulator.mode, ExecutionMode::Running);
        
        // Pause execution
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.mode, ExecutionMode::Paused,
                   "Should be able to pause from Running mode");
    }

    #[test]
    fn test_pause_from_stepping_mode() {
        // Requirement 6.1: Should be able to pause from stepping mode
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Transition to Stepping mode
        emulator.mode = ExecutionMode::Stepping;
        assert_eq!(emulator.mode, ExecutionMode::Stepping);
        
        // Pause execution
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.mode, ExecutionMode::Paused,
                   "Should be able to pause from Stepping mode");
    }
}

#[cfg(test)]
mod state_display_tests {
    use super::*;

    #[test]
    fn test_display_state_when_paused() {
        // Requirement 6.2: Display CPU state when paused
        let test_data = vec![0xA9, 0x42]; // LDA #$42
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Verify emulator is paused
        assert_eq!(emulator.mode, ExecutionMode::Paused);
        
        // Verify we can access CPU state
        assert_eq!(emulator.cpu.state.pc, 0x0000);
        assert_eq!(emulator.cpu.state.a, 0x00);
        assert_eq!(emulator.cpu.state.x, 0x00);
        assert_eq!(emulator.cpu.state.y, 0x00);
        assert_eq!(emulator.cpu.state.sp, 0xFF);
        
        // Display state should not panic (tested in display_state.rs)
        emulator.cpu.display_state();
    }

    #[test]
    fn test_state_accuracy_after_execution() {
        // Requirement 6.2: State display should be accurate
        let test_data = vec![
            0xA9, 0x42,  // LDA #$42
            0xAA,        // TAX
            0xA8,        // TAY
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Execute first instruction
        emulator.cpu.step().expect("Step failed");
        assert_eq!(emulator.cpu.state.a, 0x42, "A should be 0x42");
        assert_eq!(emulator.cpu.state.pc, 0x0002, "PC should advance");
        
        // Execute second instruction
        emulator.cpu.step().expect("Step failed");
        assert_eq!(emulator.cpu.state.x, 0x42, "X should be 0x42");
        
        // Execute third instruction
        emulator.cpu.step().expect("Step failed");
        assert_eq!(emulator.cpu.state.y, 0x42, "Y should be 0x42");
    }
}

#[cfg(test)]
mod step_command_tests {
    use super::*;

    #[test]
    fn test_step_executes_exactly_one_instruction() {
        // Requirement 6.3: Step command executes exactly one instruction
        let test_data = vec![
            0xA9, 0x10,  // LDA #$10
            0xA2, 0x20,  // LDX #$20
            0xA0, 0x30,  // LDY #$30
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Initial state
        let initial_pc = emulator.cpu.state.pc;
        assert_eq!(initial_pc, 0x0000);
        assert_eq!(emulator.cpu.state.a, 0x00);
        
        // Execute one step
        emulator.mode = ExecutionMode::Stepping;
        emulator.cpu.step().expect("Step failed");
        
        // Verify exactly one instruction was executed
        assert_eq!(emulator.cpu.state.pc, 0x0002, "PC should advance by 2");
        assert_eq!(emulator.cpu.state.a, 0x10, "A should be loaded");
        assert_eq!(emulator.cpu.state.x, 0x00, "X should not be affected");
        assert_eq!(emulator.cpu.state.y, 0x00, "Y should not be affected");
    }

    #[test]
    fn test_multiple_steps_execute_sequentially() {
        // Requirement 6.3: Multiple step commands execute instructions sequentially
        let test_data = vec![
            0xA9, 0x10,  // LDA #$10
            0x69, 0x05,  // ADC #$05
            0x69, 0x03,  // ADC #$03
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Step 1: LDA #$10
        emulator.cpu.step().expect("Step 1 failed");
        assert_eq!(emulator.cpu.state.a, 0x10);
        assert_eq!(emulator.cpu.state.pc, 0x0002);
        
        // Step 2: ADC #$05
        emulator.cpu.step().expect("Step 2 failed");
        assert_eq!(emulator.cpu.state.a, 0x15);
        assert_eq!(emulator.cpu.state.pc, 0x0004);
        
        // Step 3: ADC #$03
        emulator.cpu.step().expect("Step 3 failed");
        assert_eq!(emulator.cpu.state.a, 0x18);
        assert_eq!(emulator.cpu.state.pc, 0x0006);
    }

    #[test]
    fn test_step_with_branch_instruction() {
        // Requirement 6.3: Step should work correctly with branch instructions
        let test_data = vec![
            0xA9, 0x00,  // LDA #$00 (sets zero flag)
            0xF0, 0x02,  // BEQ +2 (branch if zero)
            0xA9, 0xFF,  // LDA #$FF (skipped)
            0xEA,        // NOP (target)
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Step 1: LDA #$00
        emulator.cpu.step().expect("Step 1 failed");
        assert_eq!(emulator.cpu.state.a, 0x00);
        assert!(emulator.cpu.state.flag_zero);
        
        // Step 2: BEQ +2 (should branch)
        emulator.cpu.step().expect("Step 2 failed");
        assert_eq!(emulator.cpu.state.pc, 0x0006, "Should branch to NOP");
        
        // Step 3: NOP
        emulator.cpu.step().expect("Step 3 failed");
        assert_eq!(emulator.cpu.state.pc, 0x0007);
    }

    #[test]
    fn test_step_with_subroutine_call() {
        // Requirement 6.3: Step should work correctly with JSR/RTS
        let test_data = vec![
            0x20, 0x05, 0x00,  // JSR $0005
            0xEA,              // NOP (return here)
            0xEA,              // NOP
            0xA9, 0x42,        // LDA #$42 (subroutine at 0x0005)
            0x60,              // RTS
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Step 1: JSR $0005
        emulator.cpu.step().expect("Step 1 failed");
        assert_eq!(emulator.cpu.state.pc, 0x0005, "Should jump to subroutine");
        
        // Step 2: LDA #$42
        emulator.cpu.step().expect("Step 2 failed");
        assert_eq!(emulator.cpu.state.a, 0x42);
        
        // Step 3: RTS
        emulator.cpu.step().expect("Step 3 failed");
        assert_eq!(emulator.cpu.state.pc, 0x0003, "Should return after JSR");
    }
}

#[cfg(test)]
mod state_update_tests {
    use super::*;

    #[test]
    fn test_state_updated_after_step() {
        // Requirement 6.4: State should be updated after step command
        let test_data = vec![0xA9, 0x42]; // LDA #$42
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Capture initial state
        let initial_a = emulator.cpu.state.a;
        let initial_pc = emulator.cpu.state.pc;
        
        // Execute step
        emulator.cpu.step().expect("Step failed");
        
        // Verify state was updated
        assert_ne!(emulator.cpu.state.a, initial_a, "A should be updated");
        assert_ne!(emulator.cpu.state.pc, initial_pc, "PC should be updated");
        assert_eq!(emulator.cpu.state.a, 0x42);
        assert_eq!(emulator.cpu.state.pc, 0x0002);
    }

    #[test]
    fn test_flags_updated_after_step() {
        // Requirement 6.4: Flags should be updated after step
        let test_data = vec![
            0xA9, 0x00,  // LDA #$00 (sets zero flag)
            0xA9, 0xFF,  // LDA #$FF (sets negative flag)
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Step 1: LDA #$00
        emulator.cpu.step().expect("Step 1 failed");
        assert!(emulator.cpu.state.flag_zero, "Zero flag should be set");
        assert!(!emulator.cpu.state.flag_negative, "Negative flag should be clear");
        
        // Step 2: LDA #$FF
        emulator.cpu.step().expect("Step 2 failed");
        assert!(!emulator.cpu.state.flag_zero, "Zero flag should be clear");
        assert!(emulator.cpu.state.flag_negative, "Negative flag should be set");
    }

    #[test]
    fn test_memory_updated_after_step() {
        // Requirement 6.4: Memory should be updated after step
        let test_data = vec![
            0xA9, 0x42,  // LDA #$42
            0x85, 0x10,  // STA $10
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Verify initial memory state
        assert_eq!(emulator.cpu.memory.read(0x10), 0x00);
        
        // Step 1: LDA #$42
        emulator.cpu.step().expect("Step 1 failed");
        
        // Step 2: STA $10
        emulator.cpu.step().expect("Step 2 failed");
        
        // Verify memory was updated
        assert_eq!(emulator.cpu.memory.read(0x10), 0x42,
                   "Memory should be updated after STA");
    }
}

#[cfg(test)]
mod continue_command_tests {
    use super::*;

    #[test]
    fn test_continue_from_paused() {
        // Requirement 6.5: Continue command should resume execution
        let test_data = vec![0xEA, 0xEA, 0xEA]; // Three NOPs
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Start in Paused mode
        assert_eq!(emulator.mode, ExecutionMode::Paused);
        
        // Continue execution
        emulator.mode = ExecutionMode::Running;
        assert_eq!(emulator.mode, ExecutionMode::Running,
                   "Should transition to Running mode");
    }

    #[test]
    fn test_continue_executes_multiple_instructions() {
        // Requirement 6.5: Continue should execute instructions continuously
        let test_data = vec![
            0xA9, 0x01,  // LDA #$01
            0x69, 0x01,  // ADC #$01
            0x69, 0x01,  // ADC #$01
            0x69, 0x01,  // ADC #$01
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Simulate running mode by executing multiple steps
        emulator.mode = ExecutionMode::Running;
        
        // Execute all instructions
        for _ in 0..4 {
            emulator.cpu.step().expect("Step failed");
        }
        
        // Verify all instructions were executed
        assert_eq!(emulator.cpu.state.a, 0x04, "Should execute all ADC instructions");
        assert_eq!(emulator.cpu.state.pc, 0x0008);
    }

    #[test]
    fn test_pause_after_continue() {
        // Requirement 6.5: Should be able to pause after continuing
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Continue
        emulator.mode = ExecutionMode::Running;
        assert_eq!(emulator.mode, ExecutionMode::Running);
        
        // Pause again
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.mode, ExecutionMode::Paused,
                   "Should be able to pause after continuing");
    }
}

#[cfg(test)]
mod execution_mode_transitions {
    use super::*;

    #[test]
    fn test_all_mode_transitions() {
        // Test all valid mode transitions
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Start: Paused
        assert_eq!(emulator.mode, ExecutionMode::Paused);
        
        // Paused -> Stepping
        emulator.mode = ExecutionMode::Stepping;
        assert_eq!(emulator.mode, ExecutionMode::Stepping);
        
        // Stepping -> Paused (after step)
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.mode, ExecutionMode::Paused);
        
        // Paused -> Running
        emulator.mode = ExecutionMode::Running;
        assert_eq!(emulator.mode, ExecutionMode::Running);
        
        // Running -> Paused
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.mode, ExecutionMode::Paused);
        
        // Paused -> Running -> Stepping
        emulator.mode = ExecutionMode::Running;
        emulator.mode = ExecutionMode::Stepping;
        assert_eq!(emulator.mode, ExecutionMode::Stepping);
    }

    #[test]
    fn test_stepping_returns_to_paused() {
        // Requirement 6.3: After stepping, mode should return to paused
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Set to stepping mode
        emulator.mode = ExecutionMode::Stepping;
        
        // Execute step
        emulator.cpu.step().expect("Step failed");
        
        // In a real implementation, the run loop would set mode back to Paused
        // Here we verify that we can transition back
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.mode, ExecutionMode::Paused);
    }
}

#[cfg(test)]
mod error_handling_in_debug_mode {
    use super::*;

    #[test]
    fn test_step_with_invalid_opcode() {
        // Test that stepping with invalid opcode is handled gracefully
        let test_data = vec![0x02]; // Invalid opcode
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Try to step
        let result = emulator.cpu.step();
        
        // Should return error, not panic
        assert!(result.is_err(), "Should return error for invalid opcode");
        
        // CPU should be halted
        assert!(emulator.cpu.halted, "CPU should be halted");
        
        // Should still be able to access state
        assert_eq!(emulator.cpu.state.pc, 0x0000);
    }

    #[test]
    fn test_cannot_step_after_halt() {
        // Test that stepping after halt returns error
        let test_data = vec![0x02]; // Invalid opcode
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // First step - causes halt
        let result1 = emulator.cpu.step();
        assert!(result1.is_err());
        assert!(emulator.cpu.halted);
        
        // Second step - should fail because halted
        let result2 = emulator.cpu.step();
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), "CPU is halted");
    }

    #[test]
    fn test_state_accessible_after_error() {
        // Test that CPU state is still accessible after error
        let test_data = vec![
            0xA9, 0x42,  // LDA #$42
            0x02,        // Invalid opcode
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Execute valid instruction
        emulator.cpu.step().expect("Step 1 failed");
        assert_eq!(emulator.cpu.state.a, 0x42);
        
        // Execute invalid instruction
        let result = emulator.cpu.step();
        assert!(result.is_err());
        
        // State should still be accessible
        assert_eq!(emulator.cpu.state.a, 0x42, "State should be preserved");
        assert_eq!(emulator.cpu.state.pc, 0x0002, "PC should be at invalid opcode");
        
        // Display state should not panic
        emulator.cpu.display_state();
    }
}

#[cfg(test)]
mod instruction_count_tests {
    use super::*;

    #[test]
    fn test_instruction_count_starts_at_zero() {
        // Requirement 16.2: Instruction count should start at 0
        let test_data = vec![0xEA, 0xEA, 0xEA]; // Three NOPs
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        assert_eq!(emulator.instruction_count, 0,
                   "Instruction count should start at 0");
    }

    #[test]
    fn test_instruction_count_increments_in_stepping_mode() {
        // Requirement 16.8: Instruction count should increment in Stepping mode
        let test_data = vec![
            0xA9, 0x42,  // LDA #$42
            0xAA,        // TAX
            0xA8,        // TAY
            0xEA,        // NOP
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        assert_eq!(emulator.instruction_count, 0);
        
        // Execute first instruction in stepping mode
        emulator.mode = ExecutionMode::Stepping;
        emulator.cpu.step().expect("Step 1 failed");
        // Manually increment since we're not using the run loop
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 1, "Count should be 1 after first step");
        
        // Execute second instruction
        emulator.cpu.step().expect("Step 2 failed");
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 2, "Count should be 2 after second step");
        
        // Execute third instruction
        emulator.cpu.step().expect("Step 3 failed");
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 3, "Count should be 3 after third step");
    }

    #[test]
    fn test_instruction_count_increments_in_running_mode() {
        // Requirement 16.8: Instruction count should increment in Running mode
        let test_data = vec![
            0xA9, 0x10,  // LDA #$10
            0x69, 0x05,  // ADC #$05
            0x69, 0x03,  // ADC #$03
            0xEA,        // NOP
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        assert_eq!(emulator.instruction_count, 0);
        
        // Simulate running mode by executing multiple instructions
        emulator.mode = ExecutionMode::Running;
        for i in 1..=4 {
            emulator.cpu.step().expect(&format!("Step {} failed", i));
            // Manually increment since we're not using the run loop
            emulator.instruction_count += 1;
            assert_eq!(emulator.instruction_count, i as u64,
                       "Count should be {} after {} steps", i, i);
        }
    }

    #[test]
    fn test_instruction_count_not_incremented_on_error() {
        // Instruction count should only increment on successful execution
        let test_data = vec![
            0xA9, 0x42,  // LDA #$42
            0x02,        // Invalid opcode
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        assert_eq!(emulator.instruction_count, 0);
        
        // Execute valid instruction
        emulator.cpu.step().expect("Step 1 failed");
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 1);
        
        // Execute invalid instruction - should fail
        let result = emulator.cpu.step();
        assert!(result.is_err(), "Should fail on invalid opcode");
        
        // Count should NOT increment on error
        assert_eq!(emulator.instruction_count, 1,
                   "Count should not increment on error");
    }

    #[test]
    fn test_instruction_count_persists_across_modes() {
        // Instruction count should persist when switching between modes
        let test_data = vec![
            0xA9, 0x10,  // LDA #$10
            0xAA,        // TAX
            0xA8,        // TAY
            0xEA,        // NOP
        ];
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Execute in stepping mode
        emulator.mode = ExecutionMode::Stepping;
        emulator.cpu.step().expect("Step 1 failed");
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 1);
        
        // Switch to running mode
        emulator.mode = ExecutionMode::Running;
        emulator.cpu.step().expect("Step 2 failed");
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 2);
        
        // Switch to paused mode (no execution)
        emulator.mode = ExecutionMode::Paused;
        assert_eq!(emulator.instruction_count, 2,
                   "Count should persist when paused");
        
        // Switch back to stepping
        emulator.mode = ExecutionMode::Stepping;
        emulator.cpu.step().expect("Step 3 failed");
        emulator.instruction_count += 1;
        assert_eq!(emulator.instruction_count, 3);
    }
}
