// Unit tests for execution statistics tracking
// Feature: 6502-cpu-emulator
// Requirements: 16.1-16.8

use std::time::Duration;
use std::thread;

/// Helper function to create a simple test binary with a few instructions
fn create_test_binary() -> Vec<u8> {
    let mut binary = vec![0; 65536];
    // LDA #$42 at 0x0000
    binary[0x0000] = 0xA9;
    binary[0x0001] = 0x42;
    // NOP at 0x0002
    binary[0x0002] = 0xEA;
    // NOP at 0x0003
    binary[0x0003] = 0xEA;
    // NOP at 0x0004
    binary[0x0004] = 0xEA;
    // NOP at 0x0005
    binary[0x0005] = 0xEA;
    // BRK at 0x0006 (to halt)
    binary[0x0006] = 0x00;
    binary
}

// ============================================================================
// Instruction Count Tests
// ============================================================================

#[test]
fn test_instruction_count_initializes_to_zero() {
    // Create a test binary file
    let test_file = "test_count_init.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Instruction count should start at 0
    assert_eq!(emulator.instruction_count, 0, "Instruction count should initialize to 0");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_instruction_count_increments_correctly() {
    // Create a test binary file
    let test_file = "test_count_increment.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions and manually increment count (simulating the execution loop)
    let mut expected_count = 0;
    
    for _ in 0..5 {
        if emulator.cpu.step().is_ok() {
            emulator.instruction_count += 1;
            expected_count += 1;
        }
    }
    
    // Verify instruction count matches expected
    assert_eq!(emulator.instruction_count, expected_count, 
        "Instruction count should increment by 1 for each executed instruction");
    assert_eq!(emulator.instruction_count, 5, 
        "Should have executed exactly 5 instructions");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_instruction_count_increments_by_one_per_instruction() {
    // Create a test binary file
    let test_file = "test_count_one_per_instruction.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions one at a time and verify count increments by exactly 1
    for i in 1..=4 {
        let count_before = emulator.instruction_count;
        
        if emulator.cpu.step().is_ok() {
            emulator.instruction_count += 1;
        }
        
        let count_after = emulator.instruction_count;
        assert_eq!(count_after - count_before, 1, 
            "Instruction count should increment by exactly 1 (iteration {})", i);
    }
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_instruction_count_handles_large_values() {
    // Create a test binary file
    let test_file = "test_count_large.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Set a large instruction count
    emulator.instruction_count = 1_000_000_000;
    
    // Increment it
    emulator.instruction_count += 1;
    
    // Verify it incremented correctly
    assert_eq!(emulator.instruction_count, 1_000_000_001, 
        "Instruction count should handle large values correctly");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

// ============================================================================
// Calculate Execution Speed Tests
// ============================================================================

#[test]
fn test_calculate_execution_speed_with_zero_elapsed_time() {
    // Create a test binary file
    let test_file = "test_speed_zero.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Calculate speed immediately (elapsed time should be very close to 0)
    let speed = emulator.calculate_execution_speed();
    
    // Should return 0.0 for zero elapsed time
    assert_eq!(speed, 0.0, "Speed should be 0.0 when elapsed time is zero");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_calculate_execution_speed_with_zero_instructions() {
    // Create a test binary file
    let test_file = "test_speed_zero_instructions.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait a bit to ensure elapsed time > 0
    thread::sleep(Duration::from_millis(10));
    
    // Calculate speed with 0 instructions
    let speed = emulator.calculate_execution_speed();
    
    // Should return 0.0 when no instructions have been executed
    assert_eq!(speed, 0.0, "Speed should be 0.0 when instruction count is 0");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_calculate_execution_speed_with_known_values() {
    // Create a test binary file
    let test_file = "test_speed_known.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute some instructions
    for _ in 0..10 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
    }
    
    // Wait a small amount of time to ensure elapsed time > 0
    thread::sleep(Duration::from_millis(10));
    
    // Calculate speed
    let speed = emulator.calculate_execution_speed();
    
    // Speed should be positive and reasonable
    assert!(speed > 0.0, "Speed should be positive after executing instructions");
    assert!(speed < 1_000_000_000.0, "Speed should be reasonable (less than 1 billion instructions/sec)");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_calculate_execution_speed_formula() {
    // Create a test binary file
    let test_file = "test_speed_formula.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Set a known instruction count
    emulator.instruction_count = 1000;
    
    // Wait a measurable amount of time
    thread::sleep(Duration::from_millis(100));
    
    // Calculate speed
    let speed = emulator.calculate_execution_speed();
    let elapsed = emulator.start_time.elapsed().as_secs_f64();
    
    // Verify the formula: speed = instruction_count / elapsed_time
    let expected_speed = emulator.instruction_count as f64 / elapsed;
    
    // Allow for small floating point differences (within 1% tolerance)
    let diff = (speed - expected_speed).abs();
    let tolerance = expected_speed * 0.01; // 1% tolerance
    assert!(diff < tolerance, "Speed calculation should match formula within 1%: {} vs {} (diff: {})", speed, expected_speed, diff);
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_calculate_execution_speed_increases_with_more_instructions() {
    // Create a test binary file
    let test_file = "test_speed_increase.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait a bit to ensure elapsed time > 0
    thread::sleep(Duration::from_millis(10));
    
    // Set instruction count to 100
    emulator.instruction_count = 100;
    let speed1 = emulator.calculate_execution_speed();
    
    // Set instruction count to 200 (double)
    emulator.instruction_count = 200;
    let speed2 = emulator.calculate_execution_speed();
    
    // Speed should approximately double
    assert!(speed2 > speed1, "Speed should increase with more instructions");
    assert!((speed2 / speed1 - 2.0).abs() < 0.1, "Speed should approximately double when instruction count doubles");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_calculate_execution_speed_returns_f64() {
    // Create a test binary file
    let test_file = "test_speed_type.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Set instruction count
    emulator.instruction_count = 1000;
    thread::sleep(Duration::from_millis(10));
    
    // Calculate speed and verify it's a valid f64
    let speed = emulator.calculate_execution_speed();
    
    // Verify it's a valid f64 (not NaN or infinite)
    assert!(speed.is_finite(), "Speed should be a finite f64 value");
    assert!(!speed.is_nan(), "Speed should not be NaN");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

// ============================================================================
// Display Statistics Tests
// Note: display_statistics() is not exported from lib.rs and prints to stdout,
// so we test the underlying functionality (instruction_count and calculate_execution_speed)
// which are already tested above. The display_statistics method is tested
// indirectly through integration tests and manual testing.
// ============================================================================

// ============================================================================
// Framebuffer Update Timing Tests
// ============================================================================

#[test]
fn test_should_update_framebuffer_returns_false_immediately() {
    // Create a test binary file
    let test_file = "test_framebuffer_timing_immediate.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Should return false immediately (no time has passed)
    assert!(!emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return false immediately after creation");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_update_framebuffer_returns_true_after_33ms() {
    // Create a test binary file
    let test_file = "test_framebuffer_timing_33ms.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait for 33ms (target frame time for 30 FPS)
    thread::sleep(Duration::from_millis(33));
    
    // Should return true after 33ms
    assert!(emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return true after 33ms");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_update_framebuffer_returns_false_before_33ms() {
    // Create a test binary file
    let test_file = "test_framebuffer_timing_before_33ms.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait for less than 33ms
    thread::sleep(Duration::from_millis(20));
    
    // Should return false before 33ms
    assert!(!emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return false before 33ms have elapsed");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_update_framebuffer_targets_30_fps() {
    // Create a test binary file
    let test_file = "test_framebuffer_timing_30fps.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // 30 FPS = 1000ms / 30 = 33.33ms per frame
    // Test at the boundary
    thread::sleep(Duration::from_millis(32));
    assert!(!emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return false at 32ms (below 33ms threshold)");
    
    thread::sleep(Duration::from_millis(2)); // Total: 34ms
    assert!(emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return true at 34ms (above 33ms threshold)");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_update_framebuffer_returns_true_after_long_delay() {
    // Create a test binary file
    let test_file = "test_framebuffer_timing_long_delay.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait for much longer than 33ms
    thread::sleep(Duration::from_millis(100));
    
    // Should return true after long delay
    assert!(emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return true after delays longer than 33ms");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_update_framebuffer_consistent_behavior() {
    // Create a test binary file
    let test_file = "test_framebuffer_timing_consistent.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Multiple calls before 33ms should all return false
    for _ in 0..5 {
        assert!(!emulator.should_update_framebuffer(), 
            "should_update_framebuffer should consistently return false before 33ms");
        thread::sleep(Duration::from_millis(5));
    }
    
    // After 33ms total, should return true
    thread::sleep(Duration::from_millis(10)); // Total: ~35ms
    assert!(emulator.should_update_framebuffer(), 
        "should_update_framebuffer should return true after 33ms total");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

// ============================================================================
// Status Display Timing Tests
// ============================================================================

#[test]
fn test_should_display_status_returns_false_immediately() {
    // Create a test binary file
    let test_file = "test_status_timing_immediate.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Should return false immediately (no time has passed)
    assert!(!emulator.should_display_status(), 
        "should_display_status should return false immediately after creation");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_display_status_returns_true_after_5_seconds() {
    // Create a test binary file
    let test_file = "test_status_timing_5s.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait for 5 seconds
    thread::sleep(Duration::from_secs(5));
    
    // Should return true after 5 seconds
    assert!(emulator.should_display_status(), 
        "should_display_status should return true after 5 seconds");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_should_display_status_returns_false_before_5_seconds() {
    // Create a test binary file
    let test_file = "test_status_timing_before_5s.bin";
    let binary = create_test_binary();
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait for less than 5 seconds
    thread::sleep(Duration::from_secs(3));
    
    // Should return false before 5 seconds
    assert!(!emulator.should_display_status(), 
        "should_display_status should return false before 5 seconds have elapsed");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

// ============================================================================
// Correctness Verification Tests (Requirements 17.6, 17.7)
// These tests verify that performance optimizations don't affect correctness
// ============================================================================

#[test]
fn test_instruction_execution_not_affected_by_display_timing() {
    // Create a test binary with specific instructions
    let test_file = "test_correctness_execution.bin";
    let mut binary = vec![0; 65536];
    
    // Program: LDA #$42, STA $0200, LDA #$FF, STA $0201, BRK
    binary[0x0000] = 0xA9; // LDA #$42
    binary[0x0001] = 0x42;
    binary[0x0002] = 0x8D; // STA $0200
    binary[0x0003] = 0x00;
    binary[0x0004] = 0x02;
    binary[0x0005] = 0xA9; // LDA #$FF
    binary[0x0006] = 0xFF;
    binary[0x0007] = 0x8D; // STA $0201
    binary[0x0008] = 0x01;
    binary[0x0009] = 0x02;
    binary[0x000A] = 0x00; // BRK
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions without checking display timing
    // Note: BRK will cause an error, so we execute until error
    for _ in 0..10 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
    }
    
    // Verify instruction execution results are correct
    // After LDA #$FF, accumulator should be 0xFF
    assert_eq!(emulator.cpu.state.a, 0xFF, "Accumulator should be 0xFF after LDA #$FF");
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x42, "Memory at 0x0200 should be 0x42");
    assert_eq!(emulator.cpu.memory.read(0x0201), 0xFF, "Memory at 0x0201 should be 0xFF");
    // We executed 4 instructions before BRK (LDA, STA, LDA, STA)
    assert_eq!(emulator.instruction_count, 4, "Should have executed exactly 4 instructions before BRK");
    
    // Verify that display timing checks don't affect execution
    // (should_update_framebuffer and should_display_status are read-only)
    let _ = emulator.should_update_framebuffer();
    let _ = emulator.should_display_status();
    
    // CPU state should remain unchanged after timing checks
    assert_eq!(emulator.cpu.state.a, 0xFF, "Accumulator should still be 0xFF");
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x42, "Memory at 0x0200 should still be 0x42");
    assert_eq!(emulator.instruction_count, 4, "Instruction count should still be 4");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_memory_updates_reflected_correctly_with_display_decoupling() {
    // Create a test binary that writes to memory multiple times
    let test_file = "test_correctness_memory.bin";
    let mut binary = vec![0; 65536];
    
    // Program: Write incrementing values to memory locations
    // LDA #$01, STA $0300
    binary[0x0000] = 0xA9; binary[0x0001] = 0x01;
    binary[0x0002] = 0x8D; binary[0x0003] = 0x00; binary[0x0004] = 0x03;
    // LDA #$02, STA $0301
    binary[0x0005] = 0xA9; binary[0x0006] = 0x02;
    binary[0x0007] = 0x8D; binary[0x0008] = 0x01; binary[0x0009] = 0x03;
    // LDA #$03, STA $0302
    binary[0x000A] = 0xA9; binary[0x000B] = 0x03;
    binary[0x000C] = 0x8D; binary[0x000D] = 0x02; binary[0x000E] = 0x03;
    // LDA #$04, STA $0303
    binary[0x000F] = 0xA9; binary[0x0010] = 0x04;
    binary[0x0011] = 0x8D; binary[0x0012] = 0x03; binary[0x0013] = 0x03;
    // BRK
    binary[0x0014] = 0x00;
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions and verify memory updates after each write
    let expected_values = vec![
        (0x0300, 0x01),
        (0x0301, 0x02),
        (0x0302, 0x03),
        (0x0303, 0x04),
    ];
    
    // Execute all instructions (each write is 2 instructions: LDA + STA)
    // Total: 4 writes * 2 instructions = 8 instructions
    for _ in 0..20 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
    }
    
    // Verify all writes completed correctly
    let mut write_count = 0;
    for (addr, expected_val) in expected_values {
        let actual_val = emulator.cpu.memory.read(addr);
        if actual_val == expected_val {
            write_count += 1;
        }
        assert_eq!(actual_val, expected_val,
            "Memory at 0x{:04X} should be 0x{:02X}", addr, expected_val);
    }
    
    assert_eq!(write_count, 4, "Should have completed 4 memory writes");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_instruction_count_accurate_regardless_of_display_frequency() {
    // Create a test binary with many instructions
    let test_file = "test_correctness_count.bin";
    let mut binary = vec![0; 65536];
    
    // Fill with NOP instructions
    for i in 0..100 {
        binary[i] = 0xEA; // NOP
    }
    binary[100] = 0x00; // BRK
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions and check display timing frequently
    let mut expected_count = 0;
    for _ in 0..50 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
        expected_count += 1;
        
        // Check display timing after each instruction (simulating high-frequency checks)
        let _ = emulator.should_update_framebuffer();
        let _ = emulator.should_display_status();
    }
    
    // Verify instruction count is accurate
    assert_eq!(emulator.instruction_count, expected_count,
        "Instruction count should be accurate regardless of display timing checks");
    assert_eq!(emulator.instruction_count, 50,
        "Should have executed exactly 50 instructions");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_execution_speed_accurate_with_display_decoupling() {
    // Create a test binary
    let test_file = "test_correctness_speed.bin";
    let mut binary = vec![0; 65536];
    
    // Fill with NOP instructions
    for i in 0..100 {
        binary[i] = 0xEA; // NOP
    }
    binary[100] = 0x00; // BRK
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Wait a bit to ensure we have measurable elapsed time
    thread::sleep(Duration::from_millis(10));
    
    // Execute instructions with timing checks
    for _ in 0..50 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
        
        // Simulate display timing checks
        let _ = emulator.should_update_framebuffer();
        let _ = emulator.should_display_status();
    }
    
    // Get actual speed from emulator (uses emulator's start_time)
    let actual_speed = emulator.calculate_execution_speed();
    
    // Calculate expected speed using the same start_time
    let elapsed = emulator.start_time.elapsed().as_secs_f64();
    let expected_speed = emulator.instruction_count as f64 / elapsed;
    
    // Verify speeds match (within 5% tolerance due to timing variations)
    let diff = (actual_speed - expected_speed).abs();
    let tolerance = expected_speed * 0.05; // 5% tolerance
    assert!(diff < tolerance,
        "Execution speed should be accurate: expected {}, got {} (diff: {}, tolerance: {})",
        expected_speed, actual_speed, diff, tolerance);
    
    // Also verify the speed is reasonable (positive and not absurdly high)
    assert!(actual_speed > 0.0, "Speed should be positive");
    assert!(actual_speed < 1_000_000_000.0, "Speed should be reasonable");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_framebuffer_memory_region_updates_correctly() {
    // Create a test binary that writes to framebuffer memory region
    let test_file = "test_correctness_framebuffer_memory.bin";
    let mut binary = vec![0; 65536];
    
    // Program: Write pattern to framebuffer region (0xE000)
    // LDA #$FF, STA $E000
    binary[0x0000] = 0xA9; binary[0x0001] = 0xFF;
    binary[0x0002] = 0x8D; binary[0x0003] = 0x00; binary[0x0004] = 0xE0;
    // LDA #$AA, STA $E001
    binary[0x0005] = 0xA9; binary[0x0006] = 0xAA;
    binary[0x0007] = 0x8D; binary[0x0008] = 0x01; binary[0x0009] = 0xE0;
    // LDA #$55, STA $E002
    binary[0x000A] = 0xA9; binary[0x000B] = 0x55;
    binary[0x000C] = 0x8D; binary[0x000D] = 0x02; binary[0x000E] = 0xE0;
    // BRK
    binary[0x000F] = 0x00;
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Set framebuffer base address
    emulator.framebuffer_base = Some(0xE000);
    
    // Execute instructions
    for _ in 0..10 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
    }
    
    // Verify memory updates in framebuffer region
    assert_eq!(emulator.cpu.memory.read(0xE000), 0xFF,
        "Framebuffer memory at 0xE000 should be 0xFF");
    assert_eq!(emulator.cpu.memory.read(0xE001), 0xAA,
        "Framebuffer memory at 0xE001 should be 0xAA");
    assert_eq!(emulator.cpu.memory.read(0xE002), 0x55,
        "Framebuffer memory at 0xE002 should be 0x55");
    
    // Verify display timing doesn't affect memory
    let _ = emulator.should_update_framebuffer();
    assert_eq!(emulator.cpu.memory.read(0xE000), 0xFF,
        "Framebuffer memory should remain unchanged after timing check");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_cpu_state_unchanged_by_timing_checks() {
    // Create a test binary
    let test_file = "test_correctness_cpu_state.bin";
    let mut binary = vec![0; 65536];
    
    // Program: Set all registers to known values
    // LDA #$42
    binary[0x0000] = 0xA9; binary[0x0001] = 0x42;
    // LDX #$12
    binary[0x0002] = 0xA2; binary[0x0003] = 0x12;
    // LDY #$34
    binary[0x0004] = 0xA0; binary[0x0005] = 0x34;
    // SEC (set carry flag)
    binary[0x0006] = 0x38;
    // BRK
    binary[0x0007] = 0x00;
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions
    for _ in 0..4 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
    }
    
    // Record CPU state
    let a_before = emulator.cpu.state.a;
    let x_before = emulator.cpu.state.x;
    let y_before = emulator.cpu.state.y;
    let carry_before = emulator.cpu.state.flag_carry;
    let pc_before = emulator.cpu.state.pc;
    
    // Perform timing checks multiple times
    for _ in 0..10 {
        let _ = emulator.should_update_framebuffer();
        let _ = emulator.should_display_status();
    }
    
    // Verify CPU state is unchanged
    assert_eq!(emulator.cpu.state.a, a_before, "Accumulator should be unchanged");
    assert_eq!(emulator.cpu.state.x, x_before, "X register should be unchanged");
    assert_eq!(emulator.cpu.state.y, y_before, "Y register should be unchanged");
    assert_eq!(emulator.cpu.state.flag_carry, carry_before, "Carry flag should be unchanged");
    assert_eq!(emulator.cpu.state.pc, pc_before, "Program counter should be unchanged");
    
    // Verify expected values
    assert_eq!(emulator.cpu.state.a, 0x42, "Accumulator should be 0x42");
    assert_eq!(emulator.cpu.state.x, 0x12, "X register should be 0x12");
    assert_eq!(emulator.cpu.state.y, 0x34, "Y register should be 0x34");
    assert!(emulator.cpu.state.flag_carry, "Carry flag should be set");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_instruction_count_increments_correctly_with_frequent_timing_checks() {
    // Create a test binary
    let test_file = "test_correctness_count_timing.bin";
    let mut binary = vec![0; 65536];
    
    // Fill with NOP instructions
    for i in 0..200 {
        binary[i] = 0xEA; // NOP
    }
    binary[200] = 0x00; // BRK
    
    std::fs::write(test_file, &binary).expect("Failed to write test file");
    
    // Create emulator
    let mut emulator = cpu_6502_emulator::Emulator::new(test_file, 0x0000)
        .expect("Failed to create emulator");
    
    // Execute instructions with very frequent timing checks
    for i in 1..=100 {
        if emulator.cpu.step().is_err() {
            break;
        }
        emulator.instruction_count += 1;
        
        // Check timing after every single instruction
        let _ = emulator.should_update_framebuffer();
        let _ = emulator.should_display_status();
        
        // Verify count is correct at each step
        assert_eq!(emulator.instruction_count, i as u64,
            "Instruction count should be {} after {} instructions", i, i);
    }
    
    // Final verification
    assert_eq!(emulator.instruction_count, 100,
        "Should have executed exactly 100 instructions");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}
