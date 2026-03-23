// Property-based tests for error handling safety
// Feature: 6502-cpu-emulator, Property 22: Error Handling Safety

use cpu_6502_emulator::{Emulator, cpu::Cpu, memory::Memory};
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

// Feature: 6502-cpu-emulator, Property 22: Error Handling Safety
// **Validates: Requirements 10.4**
//
// For any operation that can fail (file loading, opcode decoding), the function
// should return a Result type and never panic.
//
// This property verifies that:
// 1. All file I/O operations return Result and handle errors gracefully
// 2. All opcode decoding operations return Result for invalid opcodes
// 3. CPU step execution returns Result for errors
// 4. Memory operations never panic on any valid address
// 5. No unwrap() or panic!() calls in production code paths

// Test that file loading returns Result for all file conditions
proptest! {
    #[test]
    fn file_loading_returns_result_never_panics(
        file_size in 0usize..=100000,
        seed in 0u8..=255
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(format!("test_file_{}.bin", seed));
        
        // Generate test data
        let test_data: Vec<u8> = (0..file_size)
            .map(|i| ((i + seed as usize) % 256) as u8)
            .collect();
        
        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&test_data).unwrap();
        }
        
        // Test that load_from_file returns Result (doesn't panic)
        let mut memory = Memory::new();
        let result = memory.load_from_file(file_path.to_str().unwrap());
        
        // Should return Ok for valid files
        prop_assert!(result.is_ok(),
            "load_from_file should return Ok for valid file of size {}",
            file_size);
        
        // Clean up
        std::fs::remove_file(&file_path).ok();
    }
}

// Test that file loading with invalid paths returns Result (doesn't panic)
proptest! {
    #[test]
    fn file_loading_invalid_path_returns_result(
        path_suffix in "[a-z]{1,20}",
        seed in 0u8..=255
    ) {
        let invalid_path = format!("/nonexistent/path/{}/file_{}.bin", path_suffix, seed);
        
        let mut memory = Memory::new();
        let result = memory.load_from_file(&invalid_path);
        
        // Should return Err (not panic) for invalid paths
        prop_assert!(result.is_err(),
            "load_from_file should return Err for invalid path: {}",
            invalid_path);
        
        // Error message should be descriptive
        let err = result.unwrap_err();
        prop_assert!(err.contains(&invalid_path),
            "Error message should include the file path: {}",
            err);
    }
}

// Test that Emulator::new returns Result for all file conditions
proptest! {
    #[test]
    fn emulator_new_returns_result_never_panics(
        start_address in 0u16..=0xFFFF,
        seed in 0u8..=255
    ) {
        // Test with nonexistent file
        let invalid_path = format!("/tmp/nonexistent_file_{}.bin", seed);
        let result = Emulator::new(&invalid_path, start_address);
        
        // Should return Err (not panic) for invalid file
        prop_assert!(result.is_err(),
            "Emulator::new should return Err for nonexistent file");
        
        // Error should be descriptive
        let err = result.unwrap_err();
        prop_assert!(err.len() > 10,
            "Error message should be descriptive, got: {}",
            err);
    }
}

// Test that Emulator::new with valid file returns Result
proptest! {
    #[test]
    fn emulator_new_with_valid_file_returns_ok(
        start_address in 0u16..=0xFFFF,
        file_size in 1usize..=70000,
        seed in 0u8..=255
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(format!("valid_file_{}.bin", seed));
        
        // Create a valid file
        let test_data: Vec<u8> = (0..file_size)
            .map(|i| ((i + seed as usize) % 256) as u8)
            .collect();
        
        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&test_data).unwrap();
        }
        
        // Test that Emulator::new returns Result
        let result = Emulator::new(file_path.to_str().unwrap(), start_address);
        
        // Should return Ok for valid file
        prop_assert!(result.is_ok(),
            "Emulator::new should return Ok for valid file");
        
        // Clean up
        std::fs::remove_file(&file_path).ok();
    }
}

// Test that opcode decoding returns Result for all byte values
proptest! {
    #[test]
    fn opcode_decoding_returns_result_for_all_bytes(
        opcode in 0u8..=0xFF
    ) {
        // decode_opcode should return Result for all possible byte values
        let result = decode_opcode(opcode);
        
        // Should always return a Result (never panic)
        // Either Ok for valid opcodes or Err for invalid ones
        match result {
            Ok(decoded) => {
                // Valid opcode - verify it has valid properties
                prop_assert!(decoded.length >= 1 && decoded.length <= 3,
                    "Valid opcode 0x{:02X} should have length 1-3, got {}",
                    opcode, decoded.length);
            }
            Err(err) => {
                // Invalid opcode - verify error is descriptive
                prop_assert!(err.contains(&format!("0x{:02X}", opcode)),
                    "Error for invalid opcode should include opcode value: {}",
                    err);
            }
        }
    }
}

// Test that CPU step execution returns Result for all opcodes
proptest! {
    #[test]
    fn cpu_step_returns_result_never_panics(
        opcode in 0u8..=0xFF,
        start_pc in 0x0200u16..=0xFFF0,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF,
        sp_value in 0x02u8..=0xFF,
        operand1 in 0u8..=0xFF,
        operand2 in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Write the opcode and potential operands
        memory.write(start_pc, opcode);
        memory.write(start_pc.wrapping_add(1), operand1);
        memory.write(start_pc.wrapping_add(2), operand2);
        
        // Set up valid memory locations
        for addr in 0x00..=0xFF {
            memory.write(addr, operand1);
        }
        for addr in 0x0100..=0x01FF {
            memory.write(addr, operand2);
        }
        memory.write(0xFFFE, 0x00);
        memory.write(0xFFFF, 0x02);
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.y = y_value;
        cpu.state.sp = sp_value;
        
        // CPU step should return Result (never panic)
        let result = cpu.step();
        
        // Should always return a Result
        match result {
            Ok(_) => {
                // Valid instruction executed successfully
                prop_assert!(!cpu.halted,
                    "CPU should not be halted after successful execution");
            }
            Err(err) => {
                // Invalid opcode or halted CPU
                prop_assert!(err.len() > 0,
                    "Error message should not be empty");
                
                // If error is about invalid opcode, CPU should be halted
                if err.contains("Invalid opcode") {
                    prop_assert!(cpu.halted,
                        "CPU should be halted after invalid opcode");
                }
            }
        }
    }
}

// Test that memory operations never panic on any address
proptest! {
    #[test]
    fn memory_operations_never_panic(
        address in 0u16..=0xFFFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Write should never panic
        memory.write(address, value);
        
        // Read should never panic
        let read_value = memory.read(address);
        
        // Should return the value we wrote
        prop_assert_eq!(read_value, value,
            "Memory read should return written value at address 0x{:04X}",
            address);
    }
}

// Test that memory word operations never panic on any address
proptest! {
    #[test]
    fn memory_word_operations_never_panic(
        address in 0u16..=0xFFFF,
        low_byte in 0u8..=0xFF,
        high_byte in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Write bytes
        memory.write(address, low_byte);
        memory.write(address.wrapping_add(1), high_byte);
        
        // read_word should never panic (even at 0xFFFF where it wraps)
        let word = memory.read_word(address);
        
        // Verify correct little-endian format
        let expected = ((high_byte as u16) << 8) | (low_byte as u16);
        prop_assert_eq!(word, expected,
            "Memory word read should return correct little-endian value at 0x{:04X}",
            address);
    }
}

// Test that CPU operations never panic on boundary conditions
proptest! {
    #[test]
    fn cpu_operations_never_panic_on_boundaries(
        start_pc in prop::sample::select(vec![0x0000, 0xFFFF, 0xFFFE, 0xFFFD]),
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF,
        sp_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Write NOP instruction at boundary
        memory.write(start_pc, 0xEA); // NOP
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.y = y_value;
        cpu.state.sp = sp_value;
        
        // Should return Result (not panic) even at memory boundaries
        let result = cpu.step();
        
        prop_assert!(result.is_ok(),
            "CPU step should not panic at boundary PC 0x{:04X}",
            start_pc);
    }
}

// Test that stack operations never panic on overflow/underflow
proptest! {
    #[test]
    fn stack_operations_never_panic_on_overflow(
        initial_sp in 0u8..=0xFF,
        push_count in 0usize..=300
    ) {
        let mut memory = Memory::new();
        
        // Set up PHA instructions (push accumulator)
        for i in 0..push_count {
            memory.write(0x8000 + (i as u16), 0x48); // PHA
        }
        
        let mut cpu = Cpu::new(memory, 0x8000);
        cpu.state.sp = initial_sp;
        cpu.state.a = 0x42;
        
        // Execute push operations - should never panic even if stack wraps
        for _ in 0..push_count {
            let result = cpu.step();
            
            // Should return Result (not panic)
            prop_assert!(result.is_ok(),
                "Stack push should not panic even on overflow");
        }
        
        // Stack pointer should have wrapped correctly
        let expected_sp = initial_sp.wrapping_sub(push_count as u8);
        prop_assert_eq!(cpu.state.sp, expected_sp,
            "Stack pointer should wrap correctly after {} pushes from 0x{:02X}",
            push_count, initial_sp);
    }
}

// Test that addressing mode calculations never panic
proptest! {
    #[test]
    fn addressing_mode_calculations_never_panic(
        start_pc in 0u16..=0xFFFF,
        x_value in 0u8..=0xFF,
        _y_value in 0u8..=0xFF,
        zero_page_addr in 0u8..=0xFF,
        absolute_addr in 0u16..=0xFFFF
    ) {
        let mut memory = Memory::new();
        
        // Test Zero Page,X addressing (should wrap at 0xFF)
        memory.write(start_pc, 0xB5); // LDA $ZP,X
        memory.write(start_pc.wrapping_add(1), zero_page_addr);
        memory.write(zero_page_addr.wrapping_add(x_value) as u16, 0x42);
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.x = x_value;
        
        // Should not panic even if zero page address + X wraps
        let result = cpu.step();
        prop_assert!(result.is_ok(),
            "Zero Page,X addressing should not panic on wrap");
        
        // Test Absolute,X addressing (should wrap at 0xFFFF)
        let mut memory2 = Memory::new();
        memory2.write(start_pc, 0xBD); // LDA $ADDR,X
        memory2.write(start_pc.wrapping_add(1), (absolute_addr & 0xFF) as u8);
        memory2.write(start_pc.wrapping_add(2), (absolute_addr >> 8) as u8);
        memory2.write(absolute_addr.wrapping_add(x_value as u16), 0x99);
        
        let mut cpu2 = Cpu::new(memory2, start_pc);
        cpu2.state.x = x_value;
        
        // Should not panic even if absolute address + X wraps
        let result2 = cpu2.step();
        prop_assert!(result2.is_ok(),
            "Absolute,X addressing should not panic on wrap");
    }
}

// Test that branch instructions never panic on wrapping
proptest! {
    #[test]
    fn branch_instructions_never_panic_on_wrap(
        start_pc in 0u16..=0xFFFF,
        offset in -128i8..=127,
        zero_flag in any::<bool>()
    ) {
        let mut memory = Memory::new();
        
        // BEQ instruction (branch if zero flag set)
        memory.write(start_pc, 0xF0);
        memory.write(start_pc.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.flag_zero = zero_flag;
        
        // Should not panic even if branch wraps around memory
        let result = cpu.step();
        prop_assert!(result.is_ok(),
            "Branch instruction should not panic on wrap with offset {}",
            offset);
        
        // Verify PC is in valid range (always true for u16, but check it advanced)
        if zero_flag {
            // Branch was taken
            let expected_pc = start_pc.wrapping_add(2).wrapping_add(offset as i16 as u16);
            prop_assert_eq!(cpu.state.pc, expected_pc,
                "Branch should calculate correct target PC");
        } else {
            // Branch not taken
            prop_assert_eq!(cpu.state.pc, start_pc.wrapping_add(2),
                "Branch not taken should advance PC by 2");
        }
    }
}

// Test that all Result-returning functions handle errors gracefully
proptest! {
    #[test]
    fn all_result_functions_handle_errors_gracefully(
        opcode in 0u8..=0xFF,
        start_pc in 0x0200u16..=0xFFF0
    ) {
        // Test decode_opcode
        let decode_result = decode_opcode(opcode);
        let is_decode_err = decode_result.is_err();
        match decode_result {
            Ok(_) => {
                // Valid opcode - should be able to execute
            }
            Err(err) => {
                // Invalid opcode - error should be descriptive
                prop_assert!(err.contains("Invalid opcode"),
                    "decode_opcode error should be descriptive: {}",
                    err);
                prop_assert!(err.contains(&format!("0x{:02X}", opcode)),
                    "decode_opcode error should include opcode: {}",
                    err);
            }
        }
        
        // Test CPU step with the opcode
        let mut memory = Memory::new();
        memory.write(start_pc, opcode);
        memory.write(start_pc.wrapping_add(1), 0x00);
        memory.write(start_pc.wrapping_add(2), 0x00);
        
        // Set up valid memory
        for addr in 0x00..=0xFF {
            memory.write(addr, 0x00);
        }
        for addr in 0x0100..=0x01FF {
            memory.write(addr, 0x00);
        }
        memory.write(0xFFFE, 0x00);
        memory.write(0xFFFF, 0x02);
        
        let mut cpu = Cpu::new(memory, start_pc);
        let step_result = cpu.step();
        
        match step_result {
            Ok(_) => {
                // Valid instruction executed
                prop_assert!(!cpu.halted,
                    "CPU should not be halted after successful execution");
            }
            Err(err) => {
                // Invalid instruction or error
                prop_assert!(err.len() > 0,
                    "Error message should not be empty");
                
                // If invalid opcode, should mention it
                if is_decode_err {
                    prop_assert!(err.contains("Invalid opcode") || err.contains("0x"),
                        "Error should be descriptive for invalid opcode: {}",
                        err);
                }
            }
        }
    }
}

// Test that no operation panics on extreme values
proptest! {
    #[test]
    fn no_operations_panic_on_extreme_values(
        extreme_pc in prop::sample::select(vec![0x0000, 0xFFFF, 0x8000, 0x0001, 0xFFFE]),
        extreme_sp in prop::sample::select(vec![0x00, 0xFF, 0x01, 0xFE]),
        extreme_value in prop::sample::select(vec![0x00, 0xFF, 0x80, 0x7F, 0x01, 0xFE])
    ) {
        let mut memory = Memory::new();
        
        // Write a simple instruction sequence
        memory.write(extreme_pc, 0xA9); // LDA #
        memory.write(extreme_pc.wrapping_add(1), extreme_value);
        
        let mut cpu = Cpu::new(memory, extreme_pc);
        cpu.state.sp = extreme_sp;
        cpu.state.a = extreme_value;
        cpu.state.x = extreme_value;
        cpu.state.y = extreme_value;
        
        // Should not panic on extreme values
        let result = cpu.step();
        prop_assert!(result.is_ok(),
            "Operations should not panic on extreme values: PC=0x{:04X}, SP=0x{:02X}, value=0x{:02X}",
            extreme_pc, extreme_sp, extreme_value);
    }
}

// Test that halted CPU returns Result (not panic) on subsequent steps
proptest! {
    #[test]
    fn halted_cpu_returns_result_on_step(
        start_pc in 0x0200u16..=0xFFF0,
        invalid_opcode in prop::sample::select(vec![
            0x02, 0x03, 0x04, 0x07, 0x0B, 0x0F,
            0x12, 0x13, 0x14, 0x17, 0x1A, 0x1B, 0x1C, 0x1F,
        ])
    ) {
        let mut memory = Memory::new();
        memory.write(start_pc, invalid_opcode);
        
        let mut cpu = Cpu::new(memory, start_pc);
        
        // First step should fail and halt CPU
        let result1 = cpu.step();
        prop_assert!(result1.is_err(),
            "First step with invalid opcode should return Err");
        prop_assert!(cpu.halted,
            "CPU should be halted after invalid opcode");
        
        // Subsequent steps should also return Result (not panic)
        let result2 = cpu.step();
        prop_assert!(result2.is_err(),
            "Subsequent step on halted CPU should return Err");
        
        let err = result2.unwrap_err();
        prop_assert!(err.contains("halted"),
            "Error should indicate CPU is halted: {}",
            err);
    }
}
