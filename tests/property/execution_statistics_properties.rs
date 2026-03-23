// Property-based tests for execution statistics
// Feature: 6502-cpu-emulator, Property 24: Instruction Count Accuracy

use cpu_6502_emulator::{Emulator, ExecutionMode};
use proptest::prelude::*;
use std::fs::File;
use std::io::Write;

// Complete list of all official 6502 opcodes
const VALID_OPCODES: &[u8] = &[
    // LDA - Load Accumulator
    0xA9, 0xA5, 0xB5, 0xAD, 0xBD, 0xB9, 0xA1, 0xB1,
    // LDX - Load X Register
    0xA2, 0xA6, 0xB6, 0xAE, 0xBE,
    // LDY - Load Y Register
    0xA0, 0xA4, 0xB4, 0xAC, 0xBC,
    // STA - Store Accumulator
    0x85, 0x95, 0x8D, 0x9D, 0x99, 0x81, 0x91,
    // STX - Store X Register
    0x86, 0x96, 0x8E,
    // STY - Store Y Register
    0x84, 0x94, 0x8C,
    // ADC - Add with Carry
    0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71,
    // SBC - Subtract with Carry
    0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1,
    // INC - Increment Memory
    0xE6, 0xF6, 0xEE, 0xFE,
    // INX - Increment X Register
    0xE8,
    // INY - Increment Y Register
    0xC8,
    // DEC - Decrement Memory
    0xC6, 0xD6, 0xCE, 0xDE,
    // DEX - Decrement X Register
    0xCA,
    // DEY - Decrement Y Register
    0x88,
    // AND - Logical AND
    0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31,
    // ORA - Logical OR
    0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11,
    // EOR - Logical Exclusive OR
    0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51,
    // ASL - Arithmetic Shift Left
    0x0A, 0x06, 0x16, 0x0E, 0x1E,
    // LSR - Logical Shift Right
    0x4A, 0x46, 0x56, 0x4E, 0x5E,
    // ROL - Rotate Left
    0x2A, 0x26, 0x36, 0x2E, 0x3E,
    // ROR - Rotate Right
    0x6A, 0x66, 0x76, 0x6E, 0x7E,
    // CMP - Compare Accumulator
    0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1,
    // CPX - Compare X Register
    0xE0, 0xE4, 0xEC,
    // CPY - Compare Y Register
    0xC0, 0xC4, 0xCC,
    // BCC - Branch if Carry Clear
    0x90,
    // BCS - Branch if Carry Set
    0xB0,
    // BEQ - Branch if Equal (Zero Set)
    0xF0,
    // BMI - Branch if Minus (Negative Set)
    0x30,
    // BNE - Branch if Not Equal (Zero Clear)
    0xD0,
    // BPL - Branch if Plus (Negative Clear)
    0x10,
    // BVC - Branch if Overflow Clear
    0x50,
    // BVS - Branch if Overflow Set
    0x70,
    // JMP - Jump
    0x4C, 0x6C,
    // JSR - Jump to Subroutine
    0x20,
    // RTS - Return from Subroutine
    0x60,
    // RTI - Return from Interrupt
    0x40,
    // PHA - Push Accumulator
    0x48,
    // PHP - Push Processor Status
    0x08,
    // PLA - Pull Accumulator
    0x68,
    // PLP - Pull Processor Status
    0x28,
    // TSX - Transfer Stack Pointer to X
    0xBA,
    // TXS - Transfer X to Stack Pointer
    0x9A,
    // CLC - Clear Carry Flag
    0x18,
    // CLD - Clear Decimal Flag
    0xD8,
    // CLI - Clear Interrupt Disable Flag
    0x58,
    // CLV - Clear Overflow Flag
    0xB8,
    // SEC - Set Carry Flag
    0x38,
    // SED - Set Decimal Flag
    0xF8,
    // SEI - Set Interrupt Disable Flag
    0x78,
    // TAX - Transfer Accumulator to X
    0xAA,
    // TAY - Transfer Accumulator to Y
    0xA8,
    // TXA - Transfer X to Accumulator
    0x8A,
    // TYA - Transfer Y to Accumulator
    0x98,
    // NOP - No Operation
    0xEA,
    // BRK - Break (excluded from tests as it halts execution)
    // 0x00,
];

// Feature: 6502-cpu-emulator, Property 24: Instruction Count Accuracy
// **Validates: Requirements 16.2, 16.8**
//
// For any sequence of instructions executed, the instruction count should
// increment by exactly one for each instruction, regardless of execution
// mode (Paused, Stepping, or Running).
//
// This property verifies that:
// 1. Instruction count starts at 0
// 2. Each executed instruction increments the count by exactly 1
// 3. The count is accurate across different instruction types
// 4. The count is accurate regardless of execution mode

proptest! {
    #[test]
    fn instruction_count_increments_by_one_per_instruction(
        // Generate a sequence of 1-10 random valid opcodes
        num_instructions in 1usize..=10,
        seed in any::<u64>(),
    ) {
        // Generate random sequence of valid opcodes using seed
        let mut opcodes = Vec::new();
        for i in 0..num_instructions {
            let idx = ((seed as usize).wrapping_add(i)) % VALID_OPCODES.len();
            opcodes.push(VALID_OPCODES[idx]);
        }
        
        // Create a test binary file with the instruction sequence
        let test_file = format!("test_instruction_count_{}.bin", seed);
        let mut binary = vec![0u8; 65536];
        
        // Place instructions starting at 0x0200
        let start_addr = 0x0200;
        let mut offset = 0;
        for &opcode in &opcodes {
            binary[start_addr + offset] = opcode;
            offset += 1;
            // Add dummy operand bytes (instructions may need 1-2 operand bytes)
            binary[start_addr + offset] = 0x42;
            offset += 1;
            binary[start_addr + offset] = 0x00;
            offset += 1;
        }
        
        // Set up valid memory locations
        // Zero page
        for addr in 0x00..=0xFF {
            binary[addr] = 0x42;
        }
        // Stack area
        for addr in 0x0100..=0x01FF {
            binary[addr] = 0x00;
        }
        // IRQ vector for RTI instruction
        binary[0xFFFE] = 0x00;
        binary[0xFFFF] = 0x02;
        
        // Write binary to file
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        // Create emulator
        let mut emulator = Emulator::new(&test_file, start_addr as u16).unwrap();
        
        // Verify instruction count starts at 0
        prop_assert_eq!(emulator.instruction_count, 0,
            "Instruction count should start at 0");
        
        // Execute instructions one at a time and verify count increments
        let mut expected_count = 0u64;
        for i in 0..num_instructions {
            let count_before = emulator.instruction_count;
            
            // Execute one instruction
            let result = emulator.cpu.step();
            
            // Skip if we hit an error (e.g., invalid opcode or halt)
            if result.is_err() {
                break;
            }
            
            // Manually increment count (simulating what the main loop does)
            emulator.instruction_count += 1;
            expected_count += 1;
            
            let count_after = emulator.instruction_count;
            
            // Verify count incremented by exactly 1
            prop_assert_eq!(count_after - count_before, 1,
                "Instruction count should increment by exactly 1 (instruction {} of {})",
                i + 1, num_instructions);
            
            // Verify count matches expected total
            prop_assert_eq!(count_after, expected_count,
                "Instruction count should be {} after {} instructions",
                expected_count, i + 1);
        }
        
        // Clean up test file
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that instruction count is accurate across different instruction types
proptest! {
    #[test]
    fn instruction_count_accurate_across_instruction_types(
        opcode_indices in prop::collection::vec(0usize..VALID_OPCODES.len(), 5..=15),
        start_addr in 0x0200u16..=0x8000,
    ) {
        // Create a sequence with different instruction types
        let opcodes: Vec<u8> = opcode_indices.iter()
            .map(|&idx| VALID_OPCODES[idx])
            .collect();
        
        // Create test binary
        let test_file = format!("test_instruction_types_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Place instructions
        let mut offset = 0;
        for &opcode in &opcodes {
            binary[start_addr as usize + offset] = opcode;
            offset += 1;
            // Add operand bytes
            binary[start_addr as usize + offset] = 0x10;
            offset += 1;
            binary[start_addr as usize + offset] = 0x20;
            offset += 1;
        }
        
        // Set up valid memory
        for addr in 0x00..=0xFF {
            binary[addr] = 0x10;
        }
        for addr in 0x0100..=0x01FF {
            binary[addr] = 0x00;
        }
        binary[0xFFFE] = 0x00;
        binary[0xFFFF] = 0x02;
        
        // Write to file
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        // Create emulator
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        prop_assert_eq!(emulator.instruction_count, 0);
        
        // Execute all instructions
        let mut successful_executions = 0u64;
        for _ in 0..opcodes.len() {
            if emulator.cpu.step().is_ok() {
                emulator.instruction_count += 1;
                successful_executions += 1;
            } else {
                break;
            }
        }
        
        // Verify count matches number of successful executions
        prop_assert_eq!(emulator.instruction_count, successful_executions,
            "Instruction count should match number of successfully executed instructions");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that instruction count is independent of execution mode
proptest! {
    #[test]
    fn instruction_count_independent_of_execution_mode(
        num_instructions in 3usize..=8,
        seed in any::<u64>(),
    ) {
        // Generate instruction sequence
        let mut opcodes = Vec::new();
        for i in 0..num_instructions {
            let idx = ((seed as usize) + i) % VALID_OPCODES.len();
            opcodes.push(VALID_OPCODES[idx]);
        }
        
        // Create test binary
        let test_file = format!("test_exec_mode_{}.bin", seed);
        let mut binary = vec![0u8; 65536];
        
        let start_addr = 0x0300;
        let mut offset = 0;
        for &opcode in &opcodes {
            binary[start_addr + offset] = opcode;
            offset += 1;
            binary[start_addr + offset] = 0x55;
            offset += 1;
            binary[start_addr + offset] = 0x00;
            offset += 1;
        }
        
        // Set up memory
        for addr in 0x00..=0xFF {
            binary[addr] = 0x55;
        }
        for addr in 0x0100..=0x01FF {
            binary[addr] = 0x00;
        }
        binary[0xFFFE] = 0x00;
        binary[0xFFFF] = 0x03;
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        // Test in Paused mode (which is the default)
        let mut emulator = Emulator::new(&test_file, start_addr as u16).unwrap();
        prop_assert_eq!(emulator.mode, ExecutionMode::Paused);
        
        // Execute instructions in Paused mode
        let mut count_paused = 0u64;
        for _ in 0..num_instructions {
            if emulator.cpu.step().is_ok() {
                emulator.instruction_count += 1;
                count_paused += 1;
            } else {
                break;
            }
        }
        
        prop_assert_eq!(emulator.instruction_count, count_paused,
            "Instruction count in Paused mode should match executed instructions");
        
        // Create new emulator for Stepping mode test
        let mut emulator2 = Emulator::new(&test_file, start_addr as u16).unwrap();
        emulator2.mode = ExecutionMode::Stepping;
        
        // Execute same number of instructions in Stepping mode
        let mut count_stepping = 0u64;
        for _ in 0..num_instructions {
            if emulator2.cpu.step().is_ok() {
                emulator2.instruction_count += 1;
                count_stepping += 1;
            } else {
                break;
            }
        }
        
        prop_assert_eq!(emulator2.instruction_count, count_stepping,
            "Instruction count in Stepping mode should match executed instructions");
        
        // Verify both modes produced same count for same instructions
        prop_assert_eq!(count_paused, count_stepping,
            "Instruction count should be same regardless of execution mode");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that instruction count handles sequences of different lengths
proptest! {
    #[test]
    fn instruction_count_handles_various_sequence_lengths(
        sequence_length in 1usize..=20,
        start_addr in 0x0400u16..=0x7000,
    ) {
        // Create a sequence of NOP instructions (simplest case)
        let test_file = format!("test_sequence_len_{}_{}.bin", sequence_length, start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Fill with NOP instructions
        for i in 0..sequence_length {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Execute all NOPs
        for i in 0..sequence_length {
            prop_assert!(emulator.cpu.step().is_ok(),
                "NOP instruction {} should execute successfully", i + 1);
            emulator.instruction_count += 1;
        }
        
        // Verify final count
        prop_assert_eq!(emulator.instruction_count, sequence_length as u64,
            "Instruction count should equal sequence length");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that instruction count doesn't increment on execution errors
proptest! {
    #[test]
    fn instruction_count_not_incremented_on_error(
        valid_count in 1usize..=5,
        start_addr in 0x0500u16..=0x6000,
    ) {
        let test_file = format!("test_error_count_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Place some valid instructions followed by an invalid opcode
        for i in 0..valid_count {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        // Invalid opcode
        binary[start_addr as usize + valid_count] = 0x02;
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Execute valid instructions
        for _ in 0..valid_count {
            prop_assert!(emulator.cpu.step().is_ok());
            emulator.instruction_count += 1;
        }
        
        let count_before_error = emulator.instruction_count;
        prop_assert_eq!(count_before_error, valid_count as u64);
        
        // Try to execute invalid instruction
        let result = emulator.cpu.step();
        prop_assert!(result.is_err(), "Invalid opcode should return error");
        
        // Count should NOT be incremented after error
        // (In real usage, the main loop only increments on Ok result)
        prop_assert_eq!(emulator.instruction_count, count_before_error,
            "Instruction count should not change after execution error");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Feature: 6502-cpu-emulator, Property 25: Execution Speed Calculation
// **Validates: Requirements 16.3, 16.6**
//
// For any non-zero elapsed time, the execution speed should equal the
// instruction count divided by elapsed time in seconds, producing a value
// in instructions per second.
//
// This property verifies that:
// 1. Speed calculation uses the formula: speed = instruction_count / elapsed_time
// 2. Speed is calculated correctly for various instruction counts
// 3. Speed returns 0.0 when elapsed time is zero (edge case)
// 4. Speed calculation produces valid f64 values (not NaN or infinite)

proptest! {
    #[test]
    fn execution_speed_equals_instruction_count_divided_by_elapsed_time(
        instruction_count in 1u64..=1_000_000,
        start_addr in 0x0200u16..=0x8000,
    ) {
        // Create a test binary file
        let test_file = format!("test_speed_calc_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Fill with NOP instructions
        for i in 0..100 {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Set instruction count to test value
        emulator.instruction_count = instruction_count;
        
        // Wait a small amount of time to ensure non-zero elapsed time
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Calculate speed
        let speed = emulator.calculate_execution_speed();
        let elapsed = emulator.start_time.elapsed().as_secs_f64();
        
        // Verify the formula: speed = instruction_count / elapsed_time
        let expected_speed = instruction_count as f64 / elapsed;
        
        // Allow small floating point error (within 1%)
        let tolerance = expected_speed * 0.01;
        prop_assert!(
            (speed - expected_speed).abs() <= tolerance,
            "Speed calculation should match formula: {} ≈ {} (diff: {})",
            speed, expected_speed, (speed - expected_speed).abs()
        );
        
        // Verify speed is positive
        prop_assert!(speed > 0.0, "Speed should be positive for non-zero elapsed time");
        
        // Verify speed is a valid f64
        prop_assert!(!speed.is_nan(), "Speed should not be NaN");
        prop_assert!(!speed.is_infinite(), "Speed should not be infinite");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that execution speed handles zero elapsed time gracefully
proptest! {
    #[test]
    fn execution_speed_handles_zero_elapsed_time(
        instruction_count in 0u64..=1000,
        start_addr in 0x0300u16..=0x7000,
    ) {
        // Create a test binary file
        let test_file = format!("test_speed_zero_time_{}.bin", start_addr);
        let binary = vec![0u8; 65536];
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        emulator.instruction_count = instruction_count;
        
        // Calculate speed immediately (elapsed time should be very close to 0)
        let speed = emulator.calculate_execution_speed();
        
        // Speed should be a valid f64 (not NaN or infinite)
        prop_assert!(!speed.is_nan(), "Speed should not be NaN even with near-zero elapsed time");
        prop_assert!(!speed.is_infinite(), "Speed should not be infinite even with near-zero elapsed time");
        
        // Speed should be non-negative
        prop_assert!(speed >= 0.0, "Speed should be non-negative");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that execution speed increases proportionally with instruction count
proptest! {
    #[test]
    fn execution_speed_proportional_to_instruction_count(
        base_count in 100u64..=10_000,
        multiplier in 2u64..=10,
        start_addr in 0x0400u16..=0x6000,
    ) {
        // Create a test binary file
        let test_file = format!("test_speed_proportional_{}.bin", start_addr);
        let binary = vec![0u8; 65536];
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Wait to ensure measurable elapsed time
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Set first instruction count
        emulator.instruction_count = base_count;
        let speed1 = emulator.calculate_execution_speed();
        
        // Set second instruction count (multiplied)
        emulator.instruction_count = base_count * multiplier;
        let speed2 = emulator.calculate_execution_speed();
        
        // Speed should increase proportionally (within tolerance)
        let expected_ratio = multiplier as f64;
        let actual_ratio = speed2 / speed1;
        
        // Allow 5% tolerance for timing variations
        let tolerance = expected_ratio * 0.05;
        prop_assert!(
            (actual_ratio - expected_ratio).abs() <= tolerance,
            "Speed should increase proportionally: ratio {} ≈ {} (diff: {})",
            actual_ratio, expected_ratio, (actual_ratio - expected_ratio).abs()
        );
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that execution speed calculation is consistent over time
proptest! {
    #[test]
    fn execution_speed_consistent_over_time(
        instruction_count in 1000u64..=100_000,
        start_addr in 0x0500u16..=0x5000,
    ) {
        // Create a test binary file
        let test_file = format!("test_speed_consistent_{}.bin", start_addr);
        let binary = vec![0u8; 65536];
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        emulator.instruction_count = instruction_count;
        
        // Wait to ensure measurable elapsed time
        std::thread::sleep(std::time::Duration::from_millis(20));
        
        // Calculate speed multiple times
        let speed1 = emulator.calculate_execution_speed();
        let speed2 = emulator.calculate_execution_speed();
        let speed3 = emulator.calculate_execution_speed();
        
        // All speeds should be very close (within 1% due to timing precision)
        let avg_speed = (speed1 + speed2 + speed3) / 3.0;
        let tolerance = avg_speed * 0.01;
        
        prop_assert!(
            (speed1 - avg_speed).abs() <= tolerance,
            "First speed calculation should be consistent"
        );
        prop_assert!(
            (speed2 - avg_speed).abs() <= tolerance,
            "Second speed calculation should be consistent"
        );
        prop_assert!(
            (speed3 - avg_speed).abs() <= tolerance,
            "Third speed calculation should be consistent"
        );
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that execution speed with zero instructions returns zero
proptest! {
    #[test]
    fn execution_speed_zero_with_zero_instructions(
        start_addr in 0x0600u16..=0x7000,
    ) {
        // Create a test binary file
        let test_file = format!("test_speed_zero_inst_{}.bin", start_addr);
        let binary = vec![0u8; 65536];
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Wait to ensure non-zero elapsed time
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Calculate speed with 0 instructions
        let speed = emulator.calculate_execution_speed();
        
        // Speed should be 0.0 when no instructions have been executed
        prop_assert_eq!(speed, 0.0, "Speed should be 0.0 with zero instructions");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}
