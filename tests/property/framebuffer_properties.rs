// Property-based tests for framebuffer update decoupling
// Feature: 6502-cpu-emulator, Property 26: Framebuffer Update Decoupling

use cpu_6502_emulator::Emulator;
use proptest::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

// Feature: 6502-cpu-emulator, Property 26: Framebuffer Update Decoupling
// **Validates: Requirements 17.1, 17.2, 17.3, 17.6**
//
// For any sequence of instructions in Running mode, framebuffer updates should
// occur at a fixed refresh rate independent of instruction execution rate,
// while maintaining correct memory representation.
//
// This property verifies that:
// 1. Framebuffer updates occur at the correct intervals (~30 FPS, i.e., every 33ms)
// 2. Instruction execution is not blocked by framebuffer updates
// 3. Memory updates are correctly reflected when framebuffer does update
// 4. The decoupling doesn't affect correctness

proptest! {
    #[test]
    fn framebuffer_updates_at_fixed_refresh_rate(
        num_instructions in 10usize..=50,
        start_addr in 0x0200u16..=0x8000,
    ) {
        // Create a test binary with NOP instructions
        let test_file = format!("test_fb_refresh_rate_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Fill with NOP instructions
        for i in 0..num_instructions {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Enable framebuffer at a test address
        emulator.framebuffer_base = Some(0x2000);
        
        // Verify initial state: should_update_framebuffer returns false immediately
        prop_assert!(!emulator.should_update_framebuffer(),
            "Framebuffer should not need update immediately after initialization");
        
        // Execute some instructions quickly (much faster than 33ms)
        let start_time = Instant::now();
        for _ in 0..5 {
            if emulator.cpu.step().is_err() {
                break;
            }
            emulator.instruction_count += 1;
        }
        let execution_time = start_time.elapsed();
        
        // Verify execution was fast (should be well under 33ms)
        prop_assert!(execution_time.as_millis() < 33,
            "Instruction execution should be fast (< 33ms), was {}ms", 
            execution_time.as_millis());
        
        // Framebuffer should still not need update (not enough time passed)
        prop_assert!(!emulator.should_update_framebuffer(),
            "Framebuffer should not need update before 33ms have elapsed");
        
        // Wait for the 33ms threshold
        std::thread::sleep(Duration::from_millis(35));
        
        // Now framebuffer should need update
        prop_assert!(emulator.should_update_framebuffer(),
            "Framebuffer should need update after 33ms have elapsed");
        
        // Simulate updating the framebuffer
        emulator.last_frame_time = Instant::now();
        
        // After update, should not need another update immediately
        prop_assert!(!emulator.should_update_framebuffer(),
            "Framebuffer should not need update immediately after being updated");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that instruction execution is not blocked by framebuffer timing
proptest! {
    #[test]
    fn instruction_execution_not_blocked_by_framebuffer(
        num_instructions in 20usize..=100,
        start_addr in 0x0300u16..=0x7000,
    ) {
        // Create a test binary with NOP instructions
        let test_file = format!("test_fb_nonblocking_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Fill with NOP instructions
        for i in 0..num_instructions {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Enable framebuffer
        emulator.framebuffer_base = Some(0x2000);
        
        // Execute many instructions without waiting for framebuffer update
        let start_time = Instant::now();
        let mut executed_count = 0;
        
        for _ in 0..num_instructions {
            if emulator.cpu.step().is_ok() {
                emulator.instruction_count += 1;
                executed_count += 1;
            } else {
                break;
            }
        }
        
        let execution_time = start_time.elapsed();
        
        // Verify all instructions executed successfully
        prop_assert_eq!(executed_count, num_instructions,
            "All instructions should execute without being blocked");
        
        // Verify execution was fast (not blocked by 33ms framebuffer timing)
        // Even 100 instructions should execute in well under 33ms
        prop_assert!(execution_time.as_millis() < 33,
            "Instruction execution should not be blocked by framebuffer timing, \
             executed {} instructions in {}ms",
            executed_count, execution_time.as_millis());
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that memory updates are correctly reflected when framebuffer updates
proptest! {
    #[test]
    fn memory_updates_reflected_in_framebuffer_region(
        write_value in 0u8..=0xFF,
        fb_offset in 0u16..=2399, // Framebuffer is 2400 bytes
        start_addr in 0x0400u16..=0x6000,
    ) {
        // Create a test binary with STA instruction to write to framebuffer memory
        let test_file = format!("test_fb_memory_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        let fb_base = 0x2000u16;
        let write_addr = fb_base.wrapping_add(fb_offset);
        
        // LDA #value
        binary[start_addr as usize] = 0xA9; // LDA immediate
        binary[start_addr as usize + 1] = write_value;
        
        // STA absolute
        binary[start_addr as usize + 2] = 0x8D; // STA absolute
        binary[start_addr as usize + 3] = (write_addr & 0xFF) as u8; // Low byte
        binary[start_addr as usize + 4] = (write_addr >> 8) as u8; // High byte
        
        // NOP to continue
        binary[start_addr as usize + 5] = 0xEA;
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Enable framebuffer
        emulator.framebuffer_base = Some(fb_base);
        
        // Execute LDA instruction
        prop_assert!(emulator.cpu.step().is_ok(),
            "LDA instruction should execute successfully");
        emulator.instruction_count += 1;
        
        // Verify accumulator has the value
        prop_assert_eq!(emulator.cpu.state.a, write_value,
            "Accumulator should contain the loaded value");
        
        // Execute STA instruction
        prop_assert!(emulator.cpu.step().is_ok(),
            "STA instruction should execute successfully");
        emulator.instruction_count += 1;
        
        // Verify memory was updated
        let updated_value = emulator.cpu.memory.read(write_addr);
        prop_assert_eq!(updated_value, write_value,
            "Memory should be updated with the written value");
        
        // Verify framebuffer timing is independent of memory updates
        // (should_update_framebuffer should still be based on time, not memory changes)
        prop_assert!(!emulator.should_update_framebuffer(),
            "Framebuffer update timing should be independent of memory updates");
        
        // Wait for framebuffer update interval
        std::thread::sleep(Duration::from_millis(35));
        
        // Now framebuffer should need update
        prop_assert!(emulator.should_update_framebuffer(),
            "Framebuffer should need update after time threshold");
        
        // Verify memory still has the correct value
        let final_value = emulator.cpu.memory.read(write_addr);
        prop_assert_eq!(final_value, write_value,
            "Memory value should persist correctly for framebuffer update");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that decoupling doesn't affect correctness of instruction execution
proptest! {
    #[test]
    fn decoupling_preserves_instruction_correctness(
        num_instructions in 10usize..=30,
        start_addr in 0x0500u16..=0x5000,
    ) {
        // Create a test binary with various instructions
        let test_file = format!("test_fb_correctness_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        let mut offset = 0;
        
        // LDA #$42
        binary[start_addr as usize + offset] = 0xA9;
        binary[start_addr as usize + offset + 1] = 0x42;
        offset += 2;
        
        // STA $2000 (write to framebuffer region)
        binary[start_addr as usize + offset] = 0x8D;
        binary[start_addr as usize + offset + 1] = 0x00;
        binary[start_addr as usize + offset + 2] = 0x20;
        offset += 3;
        
        // LDA #$FF
        binary[start_addr as usize + offset] = 0xA9;
        binary[start_addr as usize + offset + 1] = 0xFF;
        offset += 2;
        
        // STA $2001 (write to framebuffer region)
        binary[start_addr as usize + offset] = 0x8D;
        binary[start_addr as usize + offset + 1] = 0x01;
        binary[start_addr as usize + offset + 2] = 0x20;
        offset += 3;
        
        // Fill rest with NOPs
        for i in offset..num_instructions * 3 {
            binary[start_addr as usize + i] = 0xEA;
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Enable framebuffer
        emulator.framebuffer_base = Some(0x2000);
        
        // Execute first instruction (LDA #$42)
        prop_assert!(emulator.cpu.step().is_ok());
        emulator.instruction_count += 1;
        prop_assert_eq!(emulator.cpu.state.a, 0x42,
            "First LDA should load 0x42 into accumulator");
        
        // Execute second instruction (STA $2000)
        prop_assert!(emulator.cpu.step().is_ok());
        emulator.instruction_count += 1;
        prop_assert_eq!(emulator.cpu.memory.read(0x2000), 0x42,
            "First STA should write 0x42 to memory");
        
        // Execute third instruction (LDA #$FF)
        prop_assert!(emulator.cpu.step().is_ok());
        emulator.instruction_count += 1;
        prop_assert_eq!(emulator.cpu.state.a, 0xFF,
            "Second LDA should load 0xFF into accumulator");
        
        // Execute fourth instruction (STA $2001)
        prop_assert!(emulator.cpu.step().is_ok());
        emulator.instruction_count += 1;
        prop_assert_eq!(emulator.cpu.memory.read(0x2001), 0xFF,
            "Second STA should write 0xFF to memory");
        
        // Verify framebuffer timing hasn't affected instruction execution
        prop_assert!(!emulator.should_update_framebuffer(),
            "Framebuffer should not need update yet (fast execution)");
        
        // Execute remaining instructions
        let remaining = num_instructions.saturating_sub(4);
        for _ in 0..remaining {
            if emulator.cpu.step().is_err() {
                break;
            }
            emulator.instruction_count += 1;
        }
        
        // Verify memory values are still correct
        prop_assert_eq!(emulator.cpu.memory.read(0x2000), 0x42,
            "Memory at 0x2000 should still be 0x42");
        prop_assert_eq!(emulator.cpu.memory.read(0x2001), 0xFF,
            "Memory at 0x2001 should still be 0xFF");
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that framebuffer update rate is consistent (30 FPS = 33ms intervals)
proptest! {
    #[test]
    fn framebuffer_update_rate_consistent_at_30fps(
        start_addr in 0x0600u16..=0x7000,
    ) {
        // Create a test binary
        let test_file = format!("test_fb_30fps_{}.bin", start_addr);
        let binary = vec![0u8; 65536];
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Enable framebuffer
        emulator.framebuffer_base = Some(0x2000);
        
        // Test multiple update cycles
        for cycle in 0..3 {
            // Should not need update initially
            prop_assert!(!emulator.should_update_framebuffer(),
                "Cycle {}: Should not need update initially", cycle);
            
            // Wait just under 33ms
            std::thread::sleep(Duration::from_millis(32));
            prop_assert!(!emulator.should_update_framebuffer(),
                "Cycle {}: Should not need update at 32ms", cycle);
            
            // Wait to cross 33ms threshold
            std::thread::sleep(Duration::from_millis(2));
            prop_assert!(emulator.should_update_framebuffer(),
                "Cycle {}: Should need update at 34ms", cycle);
            
            // Simulate framebuffer update
            emulator.last_frame_time = Instant::now();
        }
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that many instructions can execute between framebuffer updates
proptest! {
    #[test]
    fn many_instructions_execute_between_framebuffer_updates(
        num_instructions in 100usize..=500,
        start_addr in 0x0700u16..=0x6000,
    ) {
        // Create a test binary with many NOP instructions
        let test_file = format!("test_fb_many_inst_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Fill with NOP instructions
        for i in 0..num_instructions {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        let mut emulator = Emulator::new(&test_file, start_addr).unwrap();
        
        // Enable framebuffer
        emulator.framebuffer_base = Some(0x2000);
        
        // Execute all instructions as fast as possible
        let start_time = Instant::now();
        let mut executed_count = 0;
        
        for _ in 0..num_instructions {
            if emulator.cpu.step().is_ok() {
                emulator.instruction_count += 1;
                executed_count += 1;
            } else {
                break;
            }
        }
        
        let execution_time = start_time.elapsed();
        
        // Verify all instructions executed
        prop_assert_eq!(executed_count, num_instructions,
            "All {} instructions should execute", num_instructions);
        
        // Verify execution was much faster than framebuffer update interval
        // This demonstrates the decoupling: we can execute many instructions
        // without waiting for framebuffer updates
        prop_assert!(execution_time.as_millis() < 33,
            "Should execute {} instructions in < 33ms (was {}ms), \
             demonstrating decoupling from framebuffer updates",
            num_instructions, execution_time.as_millis());
        
        // Calculate how many instructions per framebuffer update
        let instructions_per_frame = if execution_time.as_millis() > 0 {
            (num_instructions as f64 / execution_time.as_millis() as f64) * 33.0
        } else {
            num_instructions as f64
        };
        
        // Should be able to execute many instructions per frame
        prop_assert!(instructions_per_frame >= num_instructions as f64,
            "Should execute at least {} instructions per 33ms frame", num_instructions);
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}

// Test that framebuffer disabled state doesn't affect execution
proptest! {
    #[test]
    fn framebuffer_disabled_doesnt_affect_execution(
        num_instructions in 10usize..=50,
        start_addr in 0x0800u16..=0x7000,
    ) {
        // Create a test binary
        let test_file = format!("test_fb_disabled_{}.bin", start_addr);
        let mut binary = vec![0u8; 65536];
        
        // Fill with NOP instructions
        for i in 0..num_instructions {
            binary[start_addr as usize + i] = 0xEA; // NOP
        }
        
        {
            let mut file = File::create(&test_file).unwrap();
            file.write_all(&binary).unwrap();
        }
        
        // Test with framebuffer disabled
        let mut emulator_disabled = Emulator::new(&test_file, start_addr).unwrap();
        prop_assert_eq!(emulator_disabled.framebuffer_base, None,
            "Framebuffer should be disabled by default");
        
        let start_time = Instant::now();
        for _ in 0..num_instructions {
            if emulator_disabled.cpu.step().is_err() {
                break;
            }
            emulator_disabled.instruction_count += 1;
        }
        let time_disabled = start_time.elapsed();
        
        // Test with framebuffer enabled
        let mut emulator_enabled = Emulator::new(&test_file, start_addr).unwrap();
        emulator_enabled.framebuffer_base = Some(0x2000);
        
        let start_time = Instant::now();
        for _ in 0..num_instructions {
            if emulator_enabled.cpu.step().is_err() {
                break;
            }
            emulator_enabled.instruction_count += 1;
        }
        let time_enabled = start_time.elapsed();
        
        // Both should execute same number of instructions
        prop_assert_eq!(emulator_disabled.instruction_count, emulator_enabled.instruction_count,
            "Both should execute same number of instructions");
        
        // Both should execute in similar time (within reasonable tolerance)
        // The framebuffer being enabled shouldn't significantly slow down execution
        // since updates are decoupled
        let time_diff = if time_disabled > time_enabled {
            time_disabled.as_millis() - time_enabled.as_millis()
        } else {
            time_enabled.as_millis() - time_disabled.as_millis()
        };
        
        // Allow up to 10ms difference (generous tolerance for timing variations)
        prop_assert!(time_diff <= 10,
            "Execution time should be similar with/without framebuffer \
             (diff: {}ms, disabled: {}ms, enabled: {}ms)",
            time_diff, time_disabled.as_millis(), time_enabled.as_millis());
        
        // Clean up
        std::fs::remove_file(&test_file).ok();
    }
}
